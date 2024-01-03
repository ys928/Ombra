import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import { createRouter, createWebHistory } from "vue-router"
import MainPanel from "./main/MainPanel.vue";
import { load_user_app } from './user_apps';
import { load_sys_app } from "./sys_apps";
import Window from "./plugin/Window.vue";
let routes = load_user_app();
routes.push({
    path: '/',
    component: MainPanel
})
routes.push({
    path: '/plugin',
    component: Window
})
const router = createRouter({
    history: createWebHistory(),
    routes: routes
});
load_sys_app();
createApp(App)
    .use(router)
    .mount("#app");