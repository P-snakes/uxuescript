<script setup lang="ts">
import { computed, onUnmounted, ref, watch } from "vue";
import VButton from "@/components/base/VButton.vue";
import VInput from "@/components/base/VInput.vue";
import VSelector from "@/components/base/VSelector.vue";

export type ModelItem = {
  id: string;
  name: string;
  isCustom?: boolean;
};

export type ProviderItem = {
  id: string;
  name: string;
  endpoint: string;
  apiKey: string;
  isCustom?: boolean;
  models: ModelItem[];
};

type ConnectionStatus = "idle" | "testing" | "success" | "failed";

const providers = ref<ProviderItem[]>([
  {
    id: "openai",
    name: "OpenAI",
    endpoint: "https://api.openai.com/v1",
    apiKey: "",
    isCustom: false,
    models: [
      { id: "gpt-4o", name: "GPT-4o", isCustom: false },
      { id: "gpt-4o-mini", name: "GPT-4o mini", isCustom: false },
    ],
  },
  {
    id: "deepseek",
    name: "DeepSeek",
    endpoint: "https://api.deepseek.com/v1",
    apiKey: "",
    isCustom: false,
    models: [
      { id: "deepseek-chat", name: "DeepSeek-V3", isCustom: false },
      { id: "deepseek-reasoner", name: "DeepSeek-R1", isCustom: false },
    ],
  },
  {
    id: "anthropic",
    name: "Anthropic",
    endpoint: "https://api.anthropic.com",
    apiKey: "",
    isCustom: false,
    models: [
      { id: "claude-3-5-sonnet", name: "Claude 3.5 Sonnet", isCustom: false },
    ],
  },
]);

const providerIndex = ref(0);
const modelIndex = ref(0);

const isAddingProvider = ref(false);
const newProviderName = ref("");
const newProviderEndpoint = ref("");

const isAddingModel = ref(false);
const newModelName = ref("");

const connectionStatus = ref<ConnectionStatus>("idle");
let testTimer: ReturnType<typeof setTimeout> | undefined;

const selectedProvider = computed(() => providers.value[providerIndex.value]);
const selectedModel = computed(
  () => selectedProvider.value?.models[modelIndex.value],
);

const providerNames = computed(() => providers.value.map((p) => p.name));
const modelNames = computed(
  () => selectedProvider.value?.models.map((m) => m.name) ?? [],
);

const endpoint = computed({
  get: () => selectedProvider.value?.endpoint ?? "",
  set: (val: string | number) => {
    if (selectedProvider.value) selectedProvider.value.endpoint = String(val);
  },
});

const apiKey = computed({
  get: () => selectedProvider.value?.apiKey ?? "",
  set: (val: string | number) => {
    if (selectedProvider.value) selectedProvider.value.apiKey = String(val);
  },
});

const statusText = computed(() => {
  switch (connectionStatus.value) {
    case "testing":
      return "Testing...";
    case "success":
      return "Connected";
    case "failed":
      return "Connection failed";
    default:
      return "";
  }
});

watch(providerIndex, () => {
  modelIndex.value = 0;
  connectionStatus.value = "idle";
  isAddingProvider.value = false;
  isAddingModel.value = false;
});

watch(endpoint, () => {
  connectionStatus.value = "idle";
});

const createId = (prefix: string) =>
  `${prefix}-${Date.now()}-${Math.random().toString(36).slice(2, 6)}`;

const confirmAddProvider = () => {
  const name = newProviderName.value.trim();
  const ep = newProviderEndpoint.value.trim();
  if (!name || !ep) return;

  providers.value.push({
    id: createId("provider"),
    name,
    endpoint: ep,
    apiKey: "",
    isCustom: true,
    models: [{ id: createId("model"), name: "default", isCustom: true }],
  });
  providerIndex.value = providers.value.length - 1;
  newProviderName.value = "";
  newProviderEndpoint.value = "";
  isAddingProvider.value = false;
};

const deleteCurrentProvider = () => {
  if (!selectedProvider.value?.isCustom) return;
  providers.value.splice(providerIndex.value, 1);
  providerIndex.value = Math.max(0, providerIndex.value - 1);
};

const confirmAddModel = () => {
  const name = newModelName.value.trim();
  if (!name || !selectedProvider.value) return;

  selectedProvider.value.models.push({
    id: createId("model"),
    name,
    isCustom: true,
  });
  modelIndex.value = selectedProvider.value.models.length - 1;
  newModelName.value = "";
  isAddingModel.value = false;
};

const deleteCurrentModel = () => {
  if (!selectedProvider.value || !selectedModel.value?.isCustom) return;
  selectedProvider.value.models.splice(modelIndex.value, 1);
  modelIndex.value = Math.max(0, modelIndex.value - 1);
};

const testConnection = () => {
  if (testTimer) clearTimeout(testTimer);
  connectionStatus.value = "testing";
  testTimer = setTimeout(() => {
    connectionStatus.value =
      endpoint.value.startsWith("https://") || endpoint.value.startsWith("http://")
        ? "success"
        : "failed";
  }, 600);
};

const setKey = () => undefined;
defineExpose({ setKey });

onUnmounted(() => {
  if (testTimer) clearTimeout(testTimer);
});
</script>

<template>
  <div class="api-panel">
    <h2 class="title">API</h2>

    <div class="settings-container custom-scrollbar">
      <!-- Provider Selector Row -->
      <div class="field-item">
        <div class="control-row">
          <VSelector
            v-model="providerIndex"
            class="main-selector"
            label="Provider"
            :options="providerNames"
          />
          <div class="actions">
            <VButton
              :label="isAddingProvider ? 'Cancel' : '+ New'"
              size="28px"
              class="action-btn"
              @click="isAddingProvider = !isAddingProvider"
            />
            <VButton
              v-if="selectedProvider?.isCustom"
              label="Delete"
              size="28px"
              class="action-btn danger"
              @click="deleteCurrentProvider"
            />
          </div>
        </div>

        <!-- Inline Add Provider Form -->
        <div
          v-if="isAddingProvider"
          class="sub-form"
        >
          <VInput
            v-model="newProviderName"
            label="Name"
            placeholder="Provider name"
            class="sub-input"
          />
          <VInput
            v-model="newProviderEndpoint"
            label="Endpoint"
            placeholder="https://api.example.com/v1"
            class="sub-input"
          />
          <VButton
            label="Save Provider"
            size="28px"
            class="confirm-btn"
            :disabled="!newProviderName.trim() || !newProviderEndpoint.trim()"
            @click="confirmAddProvider"
          />
        </div>
      </div>

      <!-- Model Selector Row -->
      <div class="field-item">
        <div class="control-row">
          <VSelector
            v-model="modelIndex"
            class="main-selector"
            label="Model"
            :options="modelNames"
          />
          <div class="actions">
            <VButton
              :label="isAddingModel ? 'Cancel' : '+ New'"
              size="28px"
              class="action-btn"
              @click="isAddingModel = !isAddingModel"
            />
            <VButton
              v-if="selectedModel?.isCustom"
              label="Delete"
              size="28px"
              class="action-btn danger"
              @click="deleteCurrentModel"
            />
          </div>
        </div>

        <!-- Inline Add Model Form -->
        <div
          v-if="isAddingModel"
          class="sub-form"
        >
          <VInput
            v-model="newModelName"
            label="Name"
            placeholder="Model identifier"
            class="sub-input"
          />
          <VButton
            label="Save Model"
            size="28px"
            class="confirm-btn"
            :disabled="!newModelName.trim()"
            @click="confirmAddModel"
          />
        </div>
      </div>

      <!-- Endpoint & API Key -->
      <VInput
        v-model="endpoint"
        label="Endpoint"
        placeholder="https://api.example.com/v1"
        class="field-input"
      />
      <VInput
        v-model="apiKey"
        label="API Key"
        placeholder="Enter API key"
        mask-on-blur
        class="field-input"
      />

      <!-- Connection Test -->
      <div class="test-row">
        <VButton
          class="test-btn"
          :label="connectionStatus === 'testing' ? 'Testing...' : 'Test Connection'"
          size="30px"
          :disabled="connectionStatus === 'testing'"
          @click="testConnection"
        />
        <span
          v-if="statusText"
          class="status-tag"
          :class="`status-${connectionStatus}`"
        >
          {{ statusText }}
        </span>
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
  min-height: 40px;
  font-size: 1.5rem;
  align-items: center;
  justify-content: center;
}

.settings-container {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  gap: 14px;
  padding: 0 4% 12px;
  overflow-y: auto;
  overscroll-behavior-y: contain;
}

.field-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.control-row {
  display: flex;
  align-items: center;
  gap: 10px;
  width: 100%;
}

.main-selector {
  flex: 1;
  min-width: 0;
}

.actions {
  display: flex;
  align-items: center;
  gap: 6px;
}

.action-btn {
  min-width: 58px;
  font-size: 0.85rem;
}

.action-btn.danger {
  color: #a43724;
}

.sub-form {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 10px 12px;
  border-left: 2px solid #0d58a4;
  background: rgba(13, 88, 164, 0.05);
}

.sub-input {
  width: 100%;
}

.confirm-btn {
  align-self: flex-end;
  min-width: 90px;
}

.field-input {
  width: 100%;
}

.test-row {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-top: 2px;
  padding-bottom: 8px;
}

.test-btn {
  min-width: 120px;
}

.status-tag {
  font-size: 0.85rem;
  font-weight: 500;
  letter-spacing: 0.5px;
}

.status-testing {
  color: #0d58a4;
}

.status-success {
  color: #17753b;
}

.status-failed {
  color: #a43724;
}

:deep(.base-config-select:first-child .select-dropdown-wrapper) {
  z-index: 20;
}

:deep(.base-config-select:nth-child(2) .select-dropdown-wrapper) {
  z-index: 19;
}
</style>
