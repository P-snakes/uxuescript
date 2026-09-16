<script setup lang="ts">
import VButton from "@/components/base/VButton.vue";
import TheAPIPanel from "@/components/TheConfigPanel/TheAPIPanel.vue";
import TheCourseConfigPanel from "@/components/panels/TheCourseConfigPanel.vue";
import { commands } from "@/services/cmds";
import { ComponentPublicInstance, reactive, ref, watch } from "vue";
import TheSidebar from "./TheConfigPanel/TheSidebar.vue";

type ConfigItem = {
  name: string;
  ref: ComponentPublicInstance | null;
};

const configPanel = reactive({
  activeIndex: 0,
  items: [
    { name: "API", ref: null },
    { name: "Course", ref: null },
  ] satisfies ConfigItem[],
});

const transitionName = ref("config-content-up");

watch(
  () => configPanel.activeIndex,
  (nextIndex, previousIndex) => {
    transitionName.value =
      nextIndex > previousIndex ? "config-content-down" : "config-content-up";
  },
);

const apiPanelRef = ref<InstanceType<typeof TheAPIPanel> | null>(null);
const courseConfigPanelRef = ref<InstanceType<
  typeof TheCourseConfigPanel
> | null>(null);

const saveConfig = async () => {
  await Promise.all([
    apiPanelRef.value?.setKey(),
    courseConfigPanelRef.value?.setOptions(),
  ]).catch((e) => console.error("设置配置失败: ", e));
  await commands.saveConfig().catch((e) => console.error("保存配置失败: ", e));
};
</script>

<template>
  <div class="panel">
    <div class="workspace">
      <TheSidebar
        v-model="configPanel.activeIndex"
        :items="configPanel.items"
        class="sidebar"
      />
      <div class="container">
        <Transition :name="transitionName">
          <TheAPIPanel
            v-show="configPanel.activeIndex === 0"
            ref="apiPanelRef"
            class="config-panel-content"
          />
        </Transition>
        <Transition :name="transitionName">
          <TheCourseConfigPanel
            v-show="configPanel.activeIndex === 1"
            ref="courseConfigPanelRef"
            class="config-panel-content"
          />
        </Transition>
      </div>
    </div>
    <div
      class="save-button"
      @click="saveConfig"
    >
      <VButton
        label="Save"
        class="button-text"
        style="width: 35%; height: 60%"
      />
    </div>
  </div>
</template>

<style scoped>
.panel {
  display: flex;
  flex-direction: column;
}

.workspace {
  height: 80%;
  width: 100%;
  display: flex;
  flex-direction: row;
}

.sidebar {
  width: 25%;
  height: 100%;
}

.container {
  position: relative;
  width: 75%;
  height: 100%;
  overflow: hidden;
}

.config-panel-content {
  position: absolute;
  inset: 0;
  width: 100%;
  height: 100%;
}

.config-content-up-enter-active,
.config-content-up-leave-active,
.config-content-down-enter-active,
.config-content-down-leave-active {
  transition: transform 0.35s cubic-bezier(0.34, 1.56, 0.64, 1);
  will-change: transform;
}

.config-content-up-enter-from {
  transform: translateY(100%);
}

.config-content-up-leave-to {
  transform: translateY(-100%);
}

.config-content-down-enter-from {
  transform: translateY(-100%);
}

.config-content-down-leave-to {
  transform: translateY(100%);
}

.config-content-up-enter-to,
.config-content-up-leave-from,
.config-content-down-enter-to,
.config-content-down-leave-from {
  transform: translateY(0);
}

.save-button {
  height: 20%;
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
