<script setup lang="ts">
export interface WebUrlObj {
    name: string,//网站名
    url: string,//网页路径
    id: string,//用网站域名作为id
    icon: string,//网页图标
    features: Array<string>, //可被匹配的特性
}

export const recommand_urls: Array<WebUrlObj> = [
    {
        name: '必应',
        url: 'https://www.bing.com/search?q={query}',
        id: 'www.bing.com',
        icon: 'https://www.bing.com/favicon.ico',
        features: ['text']
    }
]

// //应用启动时读取配置文件、添加网页app
// let items = await read_config_item('web_quick_open') as Array<WebUrlObj>;
// if (items == undefined) return;
// for (let i = 0; i < items.length; i++) {
//     add_app(items[i].name, items[i].id, items[i].icon, items[i].features, true, () => {
//         url_open(items[i].url);
//     });
// }
// for (let i = 0; i < recommand_urls.length; i++) {
//     console.log(recommand_urls[i])
//     App.add(recommand_urls[i].name, recommand_urls[i].id, recommand_urls[i].icon, recommand_urls[i].features, false, null, () => {
//         let feature = Ombra.get_features();
//         let text = Ombra.get_text();
//         if (feature.includes('text')) {
//             let url = recommand_urls[i].url;
//             url = url.replace('{query}', text);
//             Url.open(url);
//         } else {
//             let url = recommand_urls[i].url;
//             url = url.replace('{query}', '');
//             Url.open(url);
//         }
//     });
</script>

<template></template>

<style scoped lang="less"></style>