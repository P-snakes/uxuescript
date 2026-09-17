<script setup lang="ts">
import { ref } from "vue";
import VTextBox from "./VTextBox.vue";

const { data, methods } = defineProps<{
  data: { active: boolean; draft: string; id: string };
  methods: {
    updateDraft(value: string): void;
    save(): void;
    cancel(): void;
  };
}>();

const inputRef = ref<InstanceType<typeof VTextBox> | null>(null);
defineExpose({ focus: () => inputRef.value?.focus() });

const onEnter = (event: KeyboardEvent) => {
  if (!(event.target instanceof HTMLInputElement) || event.isComposing) return;
  event.preventDefault();
  event.stopPropagation();
  methods.save();
};
</script>

<template>
  <VTextBox
    :id="data.id"
    ref="inputRef"
    class="select-editor"
    :inert="!data.active"
    :aria-hidden="!data.active"
    :model-value="data.draft"
    placeholder=""
    pattern=".*"
    @update:model-value="methods.updateDraft(String($event))"
    @keydown.enter="onEnter"
    @keydown.escape.stop.prevent="methods.cancel"
  >
    <template #trailing>
      <div
        v-if="data.active"
        class="editor-actions"
      >
        <button
          type="button"
          :disabled="!data.draft.trim()"
          @click.stop="methods.save"
        >
          Save
        </button>
        <button
          type="button"
          @click.stop="methods.cancel"
        >
          Cancel
        </button>
      </div>
    </template>
  </VTextBox>
</template>

<style scoped>
.editor-actions {
  display: flex;
  gap: 0.5rem;
  margin-left: 0.5rem;
}

.editor-actions button {
  padding: 0;
  border: 0;
  background: transparent;
  color: #0d58a4;
  font: inherit;
  font-size: 0.75rem;
  cursor: pointer;
}

.editor-actions button:disabled {
  opacity: 0.4;
  cursor: default;
}
</style>
