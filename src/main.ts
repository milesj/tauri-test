import "./styles.css";
import "primeflex/primeflex.css";
import "primevue/resources/themes/aura-dark-lime/theme.css";
import { createApp } from "vue";
import App from "./App.vue";
import PrimeVue from "primevue/config";
import { router } from "./router";
import { createPinia } from "pinia";

const pinia = createPinia();

const app = createApp(App);
app.use(pinia);
app.use(router);
app.use(PrimeVue);

app.mount("#app");
