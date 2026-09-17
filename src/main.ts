import { createApp } from "vue";
import { createPinia } from "pinia";
import TheMainPage from "./TheMainPage.vue";

createApp(TheMainPage).use(createPinia()).mount("#main");
