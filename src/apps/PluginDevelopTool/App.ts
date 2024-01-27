import DevelopTool from "./DevelopTool.vue"
//导出app注册信息对象
export default {
    name: '插件开发工具',
    id: 'plugin-develop-tool',
    icon: '/imgs/plugin_develop.png',
    feature: [],
    only_feature: false,
    component: DevelopTool,
    self: true,
    setup: () => {
    },
    preload: () => { }
}