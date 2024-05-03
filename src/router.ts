import { createMemoryHistory, createRouter } from "vue-router";
import Index from "./routes/Index.vue";
import Dashboard from "./routes/Dashboard.vue";
import Project from "./routes/Project.vue";

const routes = [
  { path: "/", component: Index },
  { path: "/dashboard", component: Dashboard },
  { path: "/project/:name", component: Project },
];

export const router = createRouter({
  history: createMemoryHistory(),
  routes,
});
