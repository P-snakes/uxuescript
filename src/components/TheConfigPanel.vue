<script setup lang="ts">
import VButton from "@/components/base/VButton.vue";
import TheAPIPanel from "@/components/panels/TheAPIPanel.vue";
import TheCourseConfigPanel from "@/components/panels/TheCourseConfigPanel.vue";
import { commands } from "@/services/cmds";
import { ComponentPublicInstance, reactive, ref } from "vue";
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
        <TheAPIPanel
          v-show="configPanel.activeIndex === 0"
          ref="apiPanelRef"
        />
        <TheCourseConfigPanel
          v-show="configPanel.activeIndex === 1"
          ref="courseConfigPanelRef"
        />
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
  width: 75%;
  height: 100%;
}

.save-button {
  height: 20%;
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
