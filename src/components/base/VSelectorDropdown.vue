<script setup lang="ts">
import type { SelectorItem } from "./VSelector.types";

const { data, methods } = defineProps<{
  data: {
    items: SelectorItem[];
    selectedIndex: number;
    mutable: boolean;
    optionHeight: string;
  };
  methods: {
    select(index: number): void;
    create(): void;
    edit(index: number): void;
    remove(index: number): void;
  };
}>();
</script>

<template>
  <div
    class="select-dropdown-wrapper"
    :style="{ '--option-height': data.optionHeight }"
  >
    <div class="select-dropdown custom-scrollbar">
      <div
        v-for="(item, index) in data.items"
        :key="index"
        class="select-option"
        :class="{ 'is-selected': index === data.selectedIndex }"
      >
        <button
          type="button"
          class="option-label"
          @click="methods.select(index)"
        >
          {{ item.label }}
        </button>
        <template v-if="item.mutable">
          <button
            type="button"
            class="option-action"
            :aria-label="`Edit ${item.label}`"
            @click="methods.edit(index)"
          >
            Edit
          </button>
          <button
            type="button"
            class="option-action is-danger"
            :aria-label="`Delete ${item.label}`"
            @click="methods.remove(index)"
          >
            Delete
          </button>
        </template>
      </div>
      <div
        v-if="data.mutable"
        class="select-option"
      >
        <button
          type="button"
          class="option-label"
          @click="methods.create"
        >
          + New
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.select-dropdown-wrapper {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  width: 100%;
  z-index: 10;
  overflow: hidden;
}

.select-dropdown {
  width: 100%;
  box-sizing: border-box;
  background-color: #ebe2cf;
  border: 2px solid #0d58a4;
  border-top: none;
  max-height: 175px;
  overflow-y: auto;
  overscroll-behavior-y: contain;
  box-shadow: 0 6px 16px rgba(13, 88, 164, 0.15);
}

.select-option {
  height: var(--option-height);
  width: 100%;
  box-sizing: border-box;
  padding: 0;
  display: flex;
  align-items: center;
  cursor: pointer;
  user-select: none;
  transition:
    background-color 0.15s ease-in-out,
    color 0.15s ease-in-out;
}

.select-option:hover {
  background-color: rgba(0, 0, 0, 0.08);
}

.option-label,
.option-action {
  height: 100%;
  padding: 0 12px;
  border: 0;
  background: transparent;
  color: inherit;
  font: inherit;
  text-align: left;
  cursor: pointer;
}

.option-label {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.option-action {
  flex: 0 0 auto;
  padding: 0 6px;
  font-size: 0.75rem;
}

.option-action:hover {
  background: rgba(0, 0, 0, 0.08);
}

.option-action.is-danger {
  color: #a43724;
}

.select-option.is-selected {
  background-color: #0d58a4;
  color: #ffffff;
}

.is-selected .option-action.is-danger {
  color: #ffd1c8;
}
</style>
