import { exp_open_file, exp_select_file, om_get_features, om_get_text } from '~/ombra'
//导出app注册信息对象
export default {
    name: '前往文件夹',
    id: '',
    icon: '/imgs/fileexplorer.png',
    feature: ['dir_path', 'file_path'],
    only_feature: true,
    component: undefined,
    self: false,
    setup: () => {
        let text = om_get_text();
        let feature = om_get_features();
        if (feature.includes('dir_path')) {
            exp_open_file(text);
        } else if (feature.includes('file_path')) {
            exp_select_file(text);
        }
    }
}