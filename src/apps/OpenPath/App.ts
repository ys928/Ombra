import Explorer from '~/api/explorer';
import Ombra from '~/api/ombra';
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
        let text = Ombra.get_text();
        let feature = Ombra.get_features();
        if (feature.includes('dir_path')) {
            Explorer.open_file(text);
        } else if (feature.includes('file_path')) {
            Explorer.select_file(text);
        }
    }
}