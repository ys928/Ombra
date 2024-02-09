import AppPanel from "./AppPanel.vue"
//导出app注册信息对象
export default {
    name: '图片工具',
    id: 'image-tools',
    icon: '/imgs/imagetools.png',
    feature: [],
    only_feature: false,
    component: AppPanel,
    self: true,
    setup: async () => {
    },
    preload: () => {

    }
}