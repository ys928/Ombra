import { createApp } from "vue";
import App from "./App.vue";
import { createRouter, createWebHistory } from "vue-router"
import MainPanel from "./main/MainPanel.vue";
import { load_user_app } from './user_apps';
import { load_sys_app } from "./sys_apps";
let routes = load_user_app();
routes.push({
    path: '/',
    component: MainPanel
})
const router = createRouter({
    history: createWebHistory(),
    routes: routes
});
load_sys_app();
createApp(App)
    .use(router)
    .mount("#app");
