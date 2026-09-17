<script setup lang="ts">
import {
  computed,
  nextTick,
  onMounted,
  onUnmounted,
  ref,
  useId,
  watch,
} from "vue";
import VLabel from "./VLabel.vue";
import VRollTransition from "./VRollTransition.vue";
import VTextBox from "./VTextBox.vue";

type OptionAction = "Edit" | "Delete";
type SelectorOption = { label: string; actions?: OptionAction[] };
type SelectorAction =
  | { type: "create" }
  | { type: "edit"; index: number }
  | { type: "delete"; index: number }
  | { type: "save" }
  | { type: "cancel" };

const {
  modelValue,
  label,
  options = [],
  id = useId(),
  editing = false,
  editorValue = "",
  placeholder = "",
  creatable = false,
  showNewWhenEmpty = false,
} = defineProps<{
  modelValue: number;
  label: string;
  options?: (string | SelectorOption)[];
  id?: string;
  editing?: boolean;
  editorValue?: string;
  placeholder?: string;
  creatable?: boolean;
  showNewWhenEmpty?: boolean;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: number): void;
  (e: "update:editorValue", value: string): void;
  (e: "action", action: SelectorAction): void;
}>();

const items = computed(() =>
  options.map((option) =>
    typeof option === "string" ? { label: option } : option,
  ),
);

const selectOption = (index: number) => {
  isOpen.value = false;
  emit("update:modelValue", index);
};

const isEmpty = computed(() => items.value.length === 0);
const showDirectNew = computed(() => isEmpty.value && showNewWhenEmpty);
const hasDropdown = computed(
  () => !showDirectNew.value && (!isEmpty.value || creatable),
);
const displayText = computed(() =>
  showDirectNew.value ? "+ New" : (items.value[modelValue]?.label ?? ""),
);

const createOption = () => {
  isOpen.value = false;
  emit("action", { type: "create" });
};

const runAction = (action: OptionAction, index: number) => {
  isOpen.value = false;
  emit("action", {
    type: action === "Edit" ? "edit" : "delete",
    index,
  });
};

const isOpen = ref(false);
const dropdownRef = ref<HTMLElement | null>(null);
const triggerRef = ref<HTMLElement | null>(null);
const editorRef = ref<InstanceType<typeof VTextBox> | null>(null);
const optionHeight = ref("48px");
let triggerResizeObserver: ResizeObserver | undefined;

watch(
  () => editing,
  async (value) => {
    isOpen.value = false;
    await nextTick();
    if (value) editorRef.value?.focus();
    else triggerRef.value?.focus();
  },
);

const toggleDropdown = () => {
  if (editing) return;
  if (showDirectNew.value) createOption();
  else if (hasDropdown.value) isOpen.value = !isOpen.value;
};

const onEditorEnter = (event: KeyboardEvent) => {
  if (!(event.target instanceof HTMLInputElement) || event.isComposing) return;
  event.preventDefault();
  event.stopPropagation();
  emit("action", { type: "save" });
};

const handleClickOutside = (event: MouseEvent) => {
  if (dropdownRef.value && !dropdownRef.value.contains(event.target as Node)) {
    isOpen.value = false;
  }
};

onMounted(() => {
  document.addEventListener("click", handleClickOutside);
  if (!triggerRef.value) return;

  const updateOptionHeight = () => {
    optionHeight.value = `${triggerRef.value?.getBoundingClientRect().height ?? 48}px`;
  };

  triggerResizeObserver = new ResizeObserver(updateOptionHeight);
  triggerResizeObserver.observe(triggerRef.value);
  updateOptionHeight();
});

onUnmounted(() => {
  document.removeEventListener("click", handleClickOutside);
  triggerResizeObserver?.disconnect();
});
</script>

<template>
  <div class="base-config-select">
    <VLabel
      :label="label"
      :for="editing ? `${id}-editor` : id"
    />

    <div
      ref="dropdownRef"
      class="input-section"
    >
      <div
        :id="editing ? undefined : id"
        ref="triggerRef"
        class="select-trigger"
        :class="{ 'is-open': isOpen, 'is-editing': editing }"
        :tabindex="editing ? -1 : 0"
        :role="editing ? undefined : 'button'"
        :aria-expanded="editing || !hasDropdown ? undefined : isOpen"
        :aria-disabled="!editing && !hasDropdown && !showDirectNew"
        @click="toggleDropdown"
        @keydown.space.self.prevent="toggleDropdown"
        @keydown.enter.self.prevent="toggleDropdown"
        @keydown.escape.prevent="isOpen = false"
      >
        <VRollTransition
          v-show="!editing"
          :value="displayText"
          class="selected-value"
        >
          <template #default="{ value }">
            <span class="selected-text">{{ value }}</span>
          </template>
        </VRollTransition>
        <VTextBox
          :id="`${id}-editor`"
          ref="editorRef"
          class="select-editor"
          :inert="!editing"
          :aria-hidden="!editing"
          :model-value="editorValue"
          :placeholder="placeholder"
          pattern=".*"
          @update:model-value="emit('update:editorValue', $event)"
          @keydown.enter="onEditorEnter"
          @keydown.escape.stop.prevent="emit('action', { type: 'cancel' })"
        >
          <template #trailing>
            <div
              v-if="editing"
              class="editor-actions"
            >
              <button
                type="button"
                :disabled="!editorValue.trim()"
                @click.stop="emit('action', { type: 'save' })"
              >
                Save
              </button>
              <button
                type="button"
                @click.stop="emit('action', { type: 'cancel' })"
              >
                Cancel
              </button>
            </div>
          </template>
        </VTextBox>
        <span
          v-show="!editing && hasDropdown"
          class="select-arrow"
          :class="{ 'is-open': isOpen }"
        >
          <svg
            viewBox="0 -960 960 960"
            width="24"
            height="24"
            fill="currentColor"
          >
            <path d="m256-424-56-56 280-280 280 280-56 56-224-223-224 223Z" />
          </svg>
        </span>
        <span
          v-for="edge in ['top', 'right', 'bottom', 'left']"
          :key="edge"
          class="select-edge"
          :class="`edge-${edge}`"
          aria-hidden="true"
        />
      </div>

      <Transition name="dropdown">
        <div
          v-show="isOpen && hasDropdown && !editing"
          class="select-dropdown-wrapper"
          :style="{ '--option-height': optionHeight }"
        >
          <div class="select-dropdown custom-scrollbar">
            <div
              v-for="(option, index) in items"
              :key="index"
              class="select-option"
              :class="{ 'is-selected': index === modelValue }"
            >
              <button
                type="button"
                class="option-label"
                @click="selectOption(index)"
              >
                {{ option.label }}
              </button>
              <button
                v-for="action in option.actions"
                :key="action"
                type="button"
                class="option-action"
                :class="{ 'is-danger': action === 'Delete' }"
                :aria-label="`${action} ${option.label}`"
                @click="runAction(action, index)"
              >
                {{ action }}
              </button>
            </div>
            <div
              v-if="creatable"
              class="select-option"
            >
              <button
                type="button"
                class="option-label"
                @click="createOption"
              >
                + New
              </button>
            </div>
          </div>
        </div>
      </Transition>
    </div>
  </div>
</template>

<style scoped>
.base-config-select {
  display: flex;
  align-items: center;
  width: 100%;
  min-width: 0;
}

.input-section {
  flex: 1 1 0;
  width: 0;
  min-width: 0;
  height: 100%;
  position: relative;
}

.select-trigger,
.select-option {
  height: 100%;
  width: 100%;
  box-sizing: border-box;
  padding: 0 12px;
  display: flex;
  align-items: center;
  cursor: pointer;
  user-select: none;
}

.select-option {
  height: var(--option-height);
  padding: 0;
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

.select-trigger {
  position: relative;
  width: 100%;
  min-height: calc(1.5em + 4px);
  font-size: 1rem;
  border: 0;
  padding: 2px 14px;
  isolation: isolate;
  justify-content: space-between;
}

.select-edge {
  position: absolute;
  z-index: 3;
  background: #0d58a4;
  pointer-events: none;
  transition: transform 0.24s cubic-bezier(0.25, 1, 0.5, 1);
}

.edge-top,
.edge-bottom {
  left: 0;
  right: 0;
  height: 2px;
  transform-origin: right;
}

.edge-left,
.edge-right {
  top: 0;
  bottom: 0;
  width: 2px;
  transform-origin: bottom;
}

.edge-top {
  top: 0;
}
.edge-bottom {
  bottom: 0;
}
.edge-left {
  left: 0;
}
.edge-right {
  right: 0;
}

.is-editing .edge-top,
.is-editing .edge-bottom {
  transform: scaleX(0);
}

.is-editing .edge-left,
.is-editing .edge-right {
  transform: scaleY(0);
}

.select-trigger.is-editing {
  cursor: text;
}

.select-trigger[aria-disabled="true"] {
  cursor: default;
}

.selected-value {
  position: relative;
  z-index: 2;
  flex: 1;
  height: 100%;
  min-height: 1.5em;
}

.select-editor {
  position: absolute;
  inset: 0;
  z-index: 0;
  width: 100%;
  height: 100%;
  user-select: text;
}

.select-editor :deep(.input-field) {
  box-sizing: border-box;
  padding-right: 7rem;
}

.select-trigger:not(.is-editing)::before {
  content: "";
  position: absolute;
  inset: 0;
  z-index: 1;
  background: #ebe2cf;
}

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

.option-action.is-danger {
  color: #a43724;
}

.is-selected .option-action.is-danger {
  color: #ffd1c8;
}

.selected-text {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.select-arrow {
  position: relative;
  z-index: 2;
  display: flex;
  transition: transform 0.2s;
  color: #0d58a4;
  transform: rotate(180deg);
}

.select-arrow.is-open {
  transform: rotate(0deg);
}

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
  transition:
    background-color 0.15s ease-in-out,
    color 0.15s ease-in-out;
}

.select-option:hover {
  background-color: rgba(0, 0, 0, 0.08);
}

.select-option.is-selected {
  background-color: #0d58a4;
  color: #ffffff;
}

/* 下拉菜单缓动动画 */
.dropdown-enter-active,
.dropdown-leave-active {
  transition: max-height 0.25s cubic-bezier(0.25, 0.8, 0.25, 1);
}

.dropdown-enter-from,
.dropdown-leave-to {
  max-height: 0 !important;
}

.dropdown-enter-to,
.dropdown-leave-from {
  max-height: 175px !important;
}
</style>
