import AppPanel from "./AppPanel.vue"
//导出app注册信息对象
export default {
    name: '插件开发',
    id: 'plugin-develop',
    icon: '/imgs/plugin_develop.png',
    feature: [],
    only_feature: false,
    component: AppPanel,
    self: true,
    setup: () => {
    },
    preload: () => { }
}