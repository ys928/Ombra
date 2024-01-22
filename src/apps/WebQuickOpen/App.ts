import { url_open } from "~/ombra";
import MainPanel from "./MainPanel.vue"
import { add_app, read_config_item } from "~/global";
//导出app注册信息对象
export default {
    name: '网页快开',
    id: 'plugin-web-quick-open',
    icon: '/imgs/webquickopen.png',
    feature: [],
    only_feature: false,
    component: MainPanel,
    self: true,
    setup: () => { },
    preload: async () => {
        //应用启动时读取配置文件、添加网页app
        let items = await read_config_item('web_quick_open');
        /**
         * name：网页名
         * url：网页路径
         * id：用网站域名作为id
         * icon：网页图标
         * features:可被匹配的特性
         */
        if (items == undefined) return;
        for (let i = 0; i < items.length; i++) {
            add_app(items['name'], items['id'], items['icon'], items['features'], true, () => {
                url_open(items['url']);
            });
        }
    }
}