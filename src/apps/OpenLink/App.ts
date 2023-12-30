import { om_get_features, om_get_text, url_open } from "~/ombra";
//导出app注册信息对象
export default {
    name: '打开网址',
    id: '',
    icon: '/imgs/link.png',
    feature: ['url'],
    only_feature: true,
    component: undefined,
    self: false,
    setup: () => {
        let text = om_get_text();
        let feature = om_get_features();
        if (!feature.includes('url')) return;
        if (text.startsWith('http://') || text.startsWith('https://')) {
            url_open(text);
        } else {
            url_open('http://' + text);
        }
    }
}