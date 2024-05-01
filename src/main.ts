import "./styles.css";
import "primeflex/primeflex.css";
import "primevue/resources/themes/aura-dark-lime/theme.css";
import { createApp } from "vue";
import App from "./App.vue";
import PrimeVue from "primevue/config";
import { router } from "./router";

const app = createApp(App);
app.use(router);
app.use(PrimeVue);
app.mount("#app");
