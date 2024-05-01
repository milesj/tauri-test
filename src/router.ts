import { createMemoryHistory, createRouter } from "vue-router";
import Index from "./routes/Index.vue";
import Dashboard from "./routes/Dashboard.vue";

const routes = [
  { path: "/", component: Index },
  { path: "/dashboard", component: Dashboard },
];

export const router = createRouter({
  history: createMemoryHistory(),
  routes,
});
