import { exp_open_file, om_get_features, om_get_text } from "~/ombra";
//导出app注册信息对象
export default {
    name: '打开文件',
    id: '',
    icon: '/imgs/openfile.png',
    feature: ['file_path'],
    only_feature: true,
    component: undefined,
    self: false,
    setup: () => {
        let text = om_get_text();
        let feature = om_get_features();
        if (feature.includes('file_path')) {
            exp_open_file(text)
        }
    }
}