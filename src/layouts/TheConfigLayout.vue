<script setup lang="ts">
import VButton from "@/components/base/VButton.vue";
import TheAPIPanel from "@/components/panels/TheAPIPanel.vue";
import TheCourseConfigPanel from "@/components/panels/TheCourseConfigPanel.vue";
import { useConfigStore } from "@/stores/config";

const configStore = useConfigStore();
</script>

<template>
  <div class="setting-panel">
    <h1 class="title">Configuration</h1>
    <div class="info-panel">
      <TheAPIPanel class="api-panel" />
      <TheCourseConfigPanel class="course-config-panel" />
    </div>
    <div class="save-button">
      <VButton
        label="Save"
        class="button-text"
        style="width: 35%; height: 60%"
        :disabled="configStore.busy"
        @click="configStore.save"
      />
    </div>
  </div>
</template>

<style scoped>
.setting-panel {
  display: flex;
  flex-direction: column;
}
.title {
  height: 10%;
  width: 100%;
  font-size: 2rem;
  display: flex;
  align-items: center;
  justify-content: center;
}
.info-panel {
  height: 70%;
  width: 100%;
  display: flex;
  flex-direction: row;
}
.api-panel {
  width: 50%;
  height: 100%;
}
.save-button {
  height: 20%;
  display: flex;
  align-items: center;
  justify-content: center;
}
.button-text {
  font-size: 1.75rem;
  letter-spacing: 1px;
}
</style>
