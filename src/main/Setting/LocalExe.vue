<script setup lang="ts">
import { UnlistenFn } from '@tauri-apps/api/event';
import { onMounted, onUnmounted, reactive } from 'vue';
import { App, CLI, Notification, Path, Url, Window, Dialog } from '~/api';
import { useAppListStore } from '~/stores/appList';
import { useConfigStore } from '~/stores/config';
import { KIDelete } from '~/kui';
import { KIPlus } from '~/kui'
const local_apps = reactive([]) as Array<LocalApp>;

const applistStore = useAppListStore();

const configStore = useConfigStore();

let uf_file_drag: UnlistenFn | undefined;

onMounted(async () => {
    let ret = await configStore.read_local_app();
    if (ret == undefined) {
        configStore.write_local_app([]);
    } else {
        local_apps.push(...ret);
    }
    uf_file_drag = await Window.event_file_drag(async (files) => {
        for (let f of files) {
            if (!f.endsWith('.exe')) {
                Notification.send('提示', '暂不支持非exe可执行文件');
                continue;
            }
            let name = await Path.file_stem(f);
            let icon = await App.get_icon(f);
            if (icon.length == 0) {
                Notification.send('错误', '获取程序图标失败');
                continue;
            }
            local_apps.push({
                name: name,
                path: f,
                icon: icon
            });
            applistStore.add(name, f, Url.convert(icon), [], null, () => {
                CLI.exec(['start', '', f]);
            })
        }
        configStore.write_local_app(local_apps);
    });
});

onUnmounted(() => {
    if (uf_file_drag) uf_file_drag();
});

async function fun_delete(path: string) {
    let ret = await Dialog.confirm('确定要删除？', '提示', 'warning');
    if (!ret) return;
    for (let i = 0; i < local_apps.length; i++) {
        if (local_apps[i].path == path) {
            local_apps.splice(i, 1);
            configStore.write_local_app(local_apps);
            applistStore.remove(path);
            break;
        }
    }
}

async function fun_add() {
    const ret = await Dialog.open({ title: '选择文件', filters: [{ name: '可执行文件', extensions: ['exe'] }] });
    if (ret == null) return;
    let f = ret as string;
    let name = await Path.file_stem(f);
    let icon = await App.get_icon(f);
    if (icon.length == 0) {
        Notification.send('错误', '获取程序图标失败');
        return;
    }
    local_apps.push({
        name: name,
        path: f,
        icon: icon
    });
    let status = applistStore.add(name, f, Url.convert(icon), [], null, () => {
        CLI.exec(['start', '', f]);
    });
    if (status == 'success') {
        configStore.write_local_app(local_apps);
    } else {
        Notification.send('错误', '请勿重复添加');
    }
}

</script>

<template>
    <div class="LocalExe">
        <div class="item" v-for="item in local_apps">
            <div class="info">
                <img :src="Url.convert(item.icon)" alt="" srcset="">
                <div>
                    <span class="name">{{ item.name }}</span>
                    <span class="path">{{ item.path }}</span>
                </div>
            </div>
            <div class="delete" @click="fun_delete(item.path)">
                <KIDelete w="12" h="12"></KIDelete>
            </div>
        </div>
        <span class="plus" @click="fun_add">
            <KIPlus :w="20" :h="20"></KIPlus>
        </span>
    </div>
</template>

<style scoped lang="less">
.LocalExe {
    width: 100%;
    height: 100%;
    position: relative;
    display: flex;
    flex-direction: column;

    .item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0 20px;

        .info {
            display: flex;
            align-items: center;
            margin: 10px 0;

            img {
                width: 30px;
                height: 30px;
                margin: 0 10px;
            }

            div {
                height: 30px;
                display: flex;
                flex-direction: column;
                justify-content: space-between;

                .name {
                    color: #eee;
                    font-size: 13px;
                }

                .path {
                    color: #8E8E8E;
                    font-size: 12px;
                }
            }

        }

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
</style>