import AppPanel from "./AppPanel.vue"
//导出app注册信息对象
export default {
    name: 'MD',
    id: 'markdown',
    icon: '/imgs/markdown.png',
    feature: [],
    only_feature: false,
    component: AppPanel,
    setup: async () => {
    },
    preload: () => {
    }
}