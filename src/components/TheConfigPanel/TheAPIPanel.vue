<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from "vue";
import VButton from "@/components/base/VButton.vue";
import VInput from "@/components/base/VInput.vue";
import VSelector from "@/components/base/VSelector.vue";
import { useConfigStore } from "@/stores/config";

type Editor = {
  kind: "provider" | "model";
  index: number | null;
  value: string;
};
type SelectorAction =
  | { type: "create" }
  | { type: "edit"; index: number }
  | { type: "delete"; index: number }
  | { type: "save" }
  | { type: "cancel" };

const configStore = useConfigStore();
const editor = ref<Editor | null>(null);
const localError = ref("");
const connectionStatus = ref<"idle" | "testing" | "success" | "failed">("idle");
let testTimer: ReturnType<typeof setTimeout> | undefined;

const entries = computed(() => configStore.entries);
const selectedProvider = computed(() => configStore.selectedProvider);
const providerIndex = computed(() =>
  entries.value.findIndex(([id]) => id === configStore.activeId),
);
const modelIndex = computed(() => selectedProvider.value?.chosen_model ?? -1);
const actions = ["Edit", "Delete"] as const;
const providerOptions = computed(() =>
  entries.value.map(([, provider]) => ({
    label: provider.name,
    actions: provider.is_custom ? [...actions] : [],
  })),
);
const modelOptions = computed(
  () =>
    selectedProvider.value?.models.map((label) => ({
      label,
      actions: selectedProvider.value?.is_custom ? [...actions] : [],
    })) ?? [],
);
const editorValue = computed({
  get: () => editor.value?.value ?? "",
  set: (value: string) => {
    if (editor.value) editor.value.value = value;
  },
});
const endpoint = computed({
  get: () => selectedProvider.value?.base_url ?? "",
  set: (value: string | number) => {
    if (selectedProvider.value?.is_custom)
      selectedProvider.value.base_url = String(value);
  },
});
const apiKey = computed({
  get: () => selectedProvider.value?.api_key ?? "",
  set: (value: string | number) => {
    if (selectedProvider.value)
      selectedProvider.value.api_key = String(value) || null;
  },
});
const error = computed(() => localError.value || configStore.error);
const statusText = computed(
  () =>
    ({
      idle: "",
      testing: "Testing...",
      success: "Connected",
      failed: "Connection failed",
    })[connectionStatus.value],
);

const startEditor = (kind: Editor["kind"], index: number | null = null) => {
  editor.value = {
    kind,
    index,
    value:
      index === null
        ? ""
        : kind === "provider"
          ? entries.value[index][1].name
          : (selectedProvider.value?.models[index] ?? ""),
  };
};
const saveEditor = () => {
  const draft = editor.value;
  const name = draft?.value.trim();
  if (!draft || !name) return;
  if (draft.kind === "provider") {
    if (draft.index === null) configStore.createProvider(name);
    else configStore.renameProvider(draft.index, name);
  } else configStore.saveModel(draft.index, name);
  editor.value = null;
};
const handleAction = (kind: Editor["kind"], action: SelectorAction) => {
  localError.value = "";
  try {
    switch (action.type) {
      case "create":
        startEditor(kind);
        break;
      case "edit":
        startEditor(kind, action.index);
        break;
      case "delete":
        if (kind === "provider") configStore.deleteProvider(action.index);
        else configStore.deleteModel(action.index);
        editor.value = null;
        break;
      case "save":
        saveEditor();
        break;
      case "cancel":
        editor.value = null;
        break;
    }
  } catch (cause) {
    localError.value = cause instanceof Error ? cause.message : String(cause);
  }
};
const resetTest = () => {
  clearTimeout(testTimer);
  connectionStatus.value = "idle";
};
watch([() => configStore.activeId, endpoint, apiKey, modelIndex], resetTest);
const testConnection = () => {
  clearTimeout(testTimer);
  connectionStatus.value = "testing";
  testTimer = setTimeout(() => {
    connectionStatus.value = /^https?:\/\//.test(endpoint.value)
      ? "success"
      : "failed";
  }, 600);
};
onMounted(() => void configStore.initialize());
onUnmounted(() => clearTimeout(testTimer));
</script>

<template>
  <div class="api-panel">
    <h2 class="title">API</h2>
    <p
      v-if="error"
      class="error-message"
      role="alert"
    >
      {{ error }}
    </p>
    <div
      class="settings-container custom-scrollbar"
      :inert="configStore.busy"
      :aria-busy="configStore.busy"
    >
      <VSelector
        v-model:editor-value="editorValue"
        :model-value="providerIndex"
        class="main-selector"
        label="Provider"
        :options="providerOptions"
        creatable
        show-new-when-empty
        :editing="editor?.kind === 'provider'"
        placeholder="Provider name"
        @update:model-value="configStore.selectProvider"
        @action="handleAction('provider', $event)"
      />
      <VSelector
        v-model:editor-value="editorValue"
        :model-value="modelIndex"
        class="main-selector"
        label="Model"
        :options="modelOptions"
        :creatable="selectedProvider?.is_custom"
        :show-new-when-empty="selectedProvider?.is_custom"
        :editing="editor?.kind === 'model'"
        placeholder="Model identifier"
        @update:model-value="configStore.selectModel"
        @action="handleAction('model', $event)"
      />
      <VInput
        v-model="endpoint"
        label="Endpoint"
        :disabled="!selectedProvider?.is_custom"
        placeholder="https://api.example.com/v1"
        class="field-input"
      />
      <VInput
        v-model="apiKey"
        label="API Key"
        :disabled="!selectedProvider"
        placeholder="Enter API key"
        mask-on-blur
        class="field-input"
      />
      <div class="test-row">
        <VButton
          class="test-btn"
          :label="
            connectionStatus === 'testing' ? 'Testing...' : 'Test Connection'
          "
          size="1.875rem"
          :disabled="connectionStatus === 'testing'"
          @click="testConnection"
        />
        <span
          v-if="statusText"
          class="status-tag"
          :class="`status-${connectionStatus}`"
          >{{ statusText }}</span
        >
      </div>
    </div>
  </div>
</template>

<style scoped>
.api-panel {
  height: 100%;
  flex: 1;
  display: flex;
  flex-direction: column;
}
.title {
  display: flex;
  height: 16%;
  min-height: 2.5rem;
  font-size: 1.5rem;
  align-items: center;
  justify-content: center;
}
.settings-container {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  gap: clamp(0.5rem, 1.5vh, 1rem);
  padding: 0 4% clamp(0.5rem, 1vh, 0.75rem);
  overflow-y: auto;
  overscroll-behavior-y: contain;
}
.main-selector {
  flex: 0 0 auto;
  min-width: 0;
}
.field-input {
  width: 100%;
}
.test-row {
  display: flex;
  align-items: center;
  gap: clamp(0.5rem, 1vw, 0.75rem);
  margin-top: 0.125rem;
  padding-bottom: 0.5rem;
}
.test-btn {
  width: fit-content;
  min-width: 7.5rem;
  flex: 0 0 auto;
}
.status-tag {
  font-size: 0.85rem;
  font-weight: 500;
  letter-spacing: 0.03125rem;
}
.status-testing {
  color: #0d58a4;
}
.status-success {
  color: #17753b;
}
.status-failed,
.error-message {
  color: #a43724;
}
.error-message {
  margin: 0 4% 0.5rem;
  font-size: 0.85rem;
}
:deep(.base-config-select:first-child .select-dropdown-wrapper) {
  z-index: 20;
}
:deep(.base-config-select:nth-child(2) .select-dropdown-wrapper) {
  z-index: 19;
}
</style>
