import { createApp } from "vue";
import App from "./App.vue";
import { createRouter, createWebHistory } from "vue-router"
import { createPinia } from 'pinia'
import MainPanel from "./main/MainPanel.vue";
import AppPanel from "./main/AppPanel.vue";
import Setting from "./main/Setting.vue";


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
        },
        {
            path: '/setting',
            component: Setting
        }
    ]
});

window.router = router

const pinia = createPinia()
createApp(App)
    .use(router)
    .use(pinia)
    .mount("#app");