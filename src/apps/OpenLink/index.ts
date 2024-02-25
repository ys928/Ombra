import { Ombra, Url } from '~/api'
//导出app注册信息对象
export default {
    name: '打开网址',
    id: '',
    icon: '/imgs/link.png',
    feature: ['url'],
    only_feature: true,
    component: null,
    setup: () => {
        let text = Ombra.get_text();
        let feature = Ombra.get_features();
        if (!feature.includes('url')) return;
        if (text.startsWith('http://') || text.startsWith('https://')) {
            Url.open(text);
        } else {
            Url.open('http://' + text);
        }
    },
    preload: () => { }
}