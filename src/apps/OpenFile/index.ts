import { Explorer, Ombra } from '~/api'
//导出app注册信息对象
export default {
    name: '打开文件',
    id: '',
    icon: '/imgs/openfile.png',
    feature: ['file_path'],
    only_feature: true,
    component: null,
    setup: () => {
        let text = Ombra.get_text();
        let feature = Ombra.get_features();
        if (feature.includes('file_path')) {
            Explorer.open_file(text)
        }
    },
    preload: () => { }
}