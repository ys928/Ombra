import { createApp } from "vue";
import App from "./App.vue";
import { createRouter, createWebHistory } from "vue-router"
import { createPinia } from 'pinia'
import MainPanel from "./main/MainPanel.vue";
import AppPanel from "./main/AppPanel.vue";
import Setting from "./main/Setting.vue";
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import 'element-plus/theme-chalk/dark/css-vars.css'
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
let app = createApp(App);
app.use(router);
app.use(pinia);
app.use(ElementPlus);
app.mount("#app");