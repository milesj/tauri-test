import { RouteRecordRaw, createMemoryHistory, createRouter } from "vue-router";
import Index from "./routes/Index.vue";
import Dashboard from "./routes/Dashboard.vue";
import Project from "./routes/Project.vue";

const routes: RouteRecordRaw[] = [
  { path: "/", component: Index },
  { path: "/dashboard", component: Dashboard },
  { path: "/project/:id", component: Project, name: "project" },
];

export const router = createRouter({
  history: createMemoryHistory(),
  routes,
});
