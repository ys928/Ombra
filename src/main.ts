import { createApp } from "vue";
import App from "./App.vue";
import { createRouter, createWebHistory } from "vue-router"
import MainPanel from "./main/MainPanel.vue";
import AppPanel from "./main/AppPanel.vue";
import { listen } from "@tauri-apps/api/event";

const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: '/',
            component: MainPanel
        },
        {
            path: '/app',
            component: AppPanel
        }
    ]
});

createApp(App)
    .use(router)
    .mount("#app");

//监视应用退出、清理缓存数据
listen('exit_app', (e) => {
    if (e.windowLabel == "MainWindow") {
        localStorage.clear();
    }
})