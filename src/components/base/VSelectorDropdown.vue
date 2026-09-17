<script setup lang="ts">
import DeleteIcon from "@/assets/delete.svg?component";
import EditIcon from "@/assets/edit.svg?component";
import NewIcon from "@/assets/new.svg?component";
import { useScaleFeedback } from "@/composables/useScaleFeedback";
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

const scaleFeedback = useScaleFeedback({ hoverScale: 1.25 });

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
            @pointerenter="scaleFeedback.handlePointerEnter"
            @pointerleave="scaleFeedback.handlePointerLeave"
          >
            <EditIcon
              class="action-icon action-icon-compact"
              aria-hidden="true"
            />
          </button>
          <button
            type="button"
            class="option-action is-danger"
            :aria-label="`Delete ${item.label}`"
            @click="methods.remove(index)"
            @pointerenter="scaleFeedback.handlePointerEnter"
            @pointerleave="scaleFeedback.handlePointerLeave"
          >
            <DeleteIcon
              class="action-icon action-icon-compact"
              aria-hidden="true"
            />
          </button>
        </template>
      </div>
      <div
        v-if="data.mutable"
        class="select-option"
      >
        <button
          type="button"
          class="option-label option-create"
          aria-label="New"
          @click="methods.create"
        >
          <NewIcon
            class="action-icon create-icon"
            aria-hidden="true"
          />
          <span>New</span>
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
  overflow-x: hidden;
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
  align-self: center;
  width: var(--option-height);
  height: 75%;
  padding: 0;
  color: #0d58a4;
  display: flex;
  align-items: center;
  justify-content: center;
}

.option-create {
  flex: 1;
  align-self: center;
  height: 75%;
  padding: 0 12px;
  display: flex;
  align-items: center;
  gap: 0.375rem;
  justify-content: flex-start;
  color: #0d58a4;
}

.action-icon {
  width: 75%;
  height: 75%;
  fill: currentColor;
}

.create-icon {
  width: auto;
  height: 60%;
}

.action-icon-compact {
  width: 60%;
  height: 60%;
}

.select-option.is-selected {
  background-color: #0d58a4;
  color: #ffffff;
}

.is-selected .option-action {
  color: #ffffff;
}
</style>
