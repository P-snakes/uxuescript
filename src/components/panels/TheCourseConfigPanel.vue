<script setup lang="ts">
import { computed, onMounted } from "vue";
import VInput from "@/components/base/VInput.vue";
import VToggle from "@/components/base/VToggle.vue";
import { useConfigStore } from "@/stores/config";

const configStore = useConfigStore();
const options = computed(() => configStore.options);
const speedValue = computed<number>({
  get: () => options.value?.speedValue ?? 1,
  set: (value) => {
    if (options.value) options.value.speedValue = value;
  },
});
onMounted(() => void configStore.initialize());
</script>

<template>
  <div class="course-config-panel">
    <h2 class="title">Course</h2>
    <div
      v-if="options"
      class="settings-container"
    >
      <VToggle
        v-model="options.persistSession"
        label="Perisist Session"
        class="option"
      />
      <VToggle
        v-model="options.muteWebview"
        label="Mute Course"
        class="option"
      />
      <VToggle
        v-model="options.speedLock"
        label="Lock Playspeed"
        class="option"
      />
      <VInput
        id="playing-speed-input"
        v-model.number="speedValue"
        placeholder="input number here"
        label="Playing Speed"
        aria-label=""
        pattern="\d+(?:\.\d*)?"
        class="option speed-input"
      />
    </div>
  </div>
</template>

<style scoped>
.course-config-panel {
  height: 100%;
  flex: 1;
  flex-direction: column;
  display: flex;
}
.settings-container {
  flex: 1;
  display: flex;
  gap: 4%;
  flex-direction: column;
}
.title {
  display: flex;
  height: 20%;
  font-size: 1.5rem;
  align-items: center;
  justify-content: center;
}
.option {
  height: 22%;
}
.speed-input :deep(.input-field) {
  text-align: center;
}
</style>
