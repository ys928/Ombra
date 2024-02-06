<script setup lang="ts">
import { onMounted, reactive, ref } from 'vue';
import { type WebUrlApp } from '~/api/app';
import { KSwitch } from '~/kui'
import { useAppListStore } from '~/stores/appList';
import { useConfigStore } from '~/stores/config';
import { KIPlus, KIDelete, KWindow } from '~/kui'
import Url from '~/api/url';
import Notification from '~/api/notification';
import Dialog from '~/api/dialog';

const web_apps = reactive([]) as Array<WebUrlApp>;

const applistStore = useAppListStore();

const configStore = useConfigStore();

const is_show_window = ref(false);

const add_web_name = ref('');
const add_web_url = ref('');

// download_file
onMounted(async () => {
    web_apps.push(... (await configStore.read_web_apps()));
});

async function fun_change(item: WebUrlApp) {
    configStore.write_web_apps(web_apps);
    if (item.on) {
        applistStore.add_web(item);
    } else {
        applistStore.remove(item.url);
    }
}
async function fun_add_web() {
    if (add_web_name.value.length == 0) {
        Notification.send('提示', '未填写名称');
        return;
    }
    if (add_web_url.value.length == 0) {
        Notification.send('提示', '未填写网址');
        return;
    }
    let fav_icon = await Url.download_favicon(add_web_url.value, add_web_name.value);
    if (fav_icon.length == 0) {
        Notification.send('提示', '获取网站图标失败，将使用默认图标');
    }
    let is_query = add_web_name.value.indexOf('{query}') != -1;
    let item = {
        name: add_web_name.value,
        url: add_web_url.value,
        id: add_web_url.value,
        features: is_query ? ['text'] : [],
        icon: fav_icon,
        on: true
    }
    let show_item = {
        name: item.name,
        url: item.url,
        id: item.id,
        features: item.features,
        icon: item.icon.length > 0 ? Url.convert(item.icon) : '/logo.png',
        on: true
    }
    applistStore.add_web(show_item);
    web_apps.push(show_item);

    let cfg_web_apps = await configStore.read_web_apps();
    cfg_web_apps.push(item);
    configStore.write_web_apps(cfg_web_apps);
    is_show_window.value = false;
}

function get_icon_url(icon: string) {
    if (icon.length > 0) {
        return Url.convert(icon);
    }
    return '/logo.png';
}
async function fun_delete(url: string) {
    let ret = await Dialog.confirm('确定要删除？', '提示', 'warning');
    if (!ret) return;
    for (let i = 0; i < web_apps.length; i++) {
        if (web_apps[i].url == url) {
            web_apps.splice(i, 1);
            break;
        }
    }
    applistStore.remove(url);

    let cfg_web_apps = await configStore.read_web_apps();
    for (let i = 0; i < cfg_web_apps.length; i++) {
        if (cfg_web_apps[i].url == url) {
            cfg_web_apps.splice(i, 1);
            break;
        }
    }
    configStore.write_web_apps(cfg_web_apps);
}
</script>

<template>
    <div class="Web">
        <div class="recommand">
            <div class="url_item" v-for="item in web_apps">
                <div class="web_info">
                    <img class="logo" :src="get_icon_url(item.icon)" alt="网站图标">
                    <div class="info">
                        <div class="name">{{ item.name }}</div>
                        <div class="url">{{ item.url }} </div>
                    </div>
                </div>
                <div class="opt">
                    <KSwitch v-model:value="item.on" @click="fun_change(item)"></KSwitch>
                    <div class="delete" @click="fun_delete(item.url)">
                        <KIDelete w="12" h="12"></KIDelete>
                    </div>
                </div>
            </div>
            <span class="plus" @click="is_show_window = true">
                <KIPlus :w="20" :h="20"></KIPlus>
            </span>
        </div>
        <KWindow v-model:isshow="is_show_window" @sure="fun_add_web" @cancel="is_show_window = false">
            <div class="content">
                <div class="name">
                    <span>名称：</span>
                    <input placeholder="搜索关键字" v-model="add_web_name" />
                </div>
                <div class="url">
                    <span>链接：</span>
                    <input placeholder="https://...." v-model="add_web_url" />
                </div>
            </div>
        </KWindow>
    </div>
</template>

<style scoped lang="less">
.Web {

    width: 100%;
    height: 100%;

    .recommand {
        width: 100%;
        display: flex;
        flex-direction: column;

        .url_item {
            display: flex;
            justify-content: space-between;
            padding: 10px 20px;

            .web_info {
                display: flex;
                justify-content: left;

                .logo {
                    width: 30px;
                    height: 30px;
                }

                .info {
                    display: flex;
                    flex-direction: column;
                    color: #BDBDBE;
                    margin-left: 10px;

                    .name {
                        font-size: 14px;
                    }

                    .url {
                        font-size: 12px;
                    }

                }

            }

            .opt {
                display: flex;
                width: 100px;
                align-items: center;
                justify-content: space-around;

                .delete {
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    background-color: #383838;
                    color: white;
                    width: 26px;
                    height: 26px;
                    border-radius: 13px;
                    cursor: pointer;

                    &:hover {
                        background-color: #F56C6C;
                    }
                }
            }
        }

        .plus {
            position: absolute;
            right: 20px;
            bottom: 20px;
            color: #A2A29B;
            width: 30px;
            height: 30px;
            display: flex;
            align-items: center;
            justify-content: center;
            cursor: pointer;
            border-radius: 15px;
            background-color: #3c3c3c;

            &:hover {
                background-color: #5c5c5c;
            }
        }
    }

    .content {
        height: 100%;
        width: 100%;

        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        color: #eee;


        .name,
        .url {
            margin: 10px 0;

            input {
                outline: none;
                border: none;
                background-color: #4A4A4A;
                height: 30px;
                border-radius: 5px;
                font-size: 16px;
                width: 300px;
                padding: 0 10px;
                color: #eee;
                border: 1px solid #4A4A4A;

                &:focus {
                    border: 1px solid #eee;
                }
            }
        }
    }
}
</style>