import Ombra from "~/api/ombra";
import Url from "~/api/url";
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
        let text = Ombra.get_text();
        let feature = Ombra.get_features();
        if (!feature.includes('url')) return;
        if (text.startsWith('http://') || text.startsWith('https://')) {
            Url.open(text);
        } else {
            Url.open('http://' + text);
        }
    }
}