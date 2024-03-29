<script setup lang="ts">
import { onMounted, onUnmounted, reactive } from 'vue';
import { App, CLI, Url, Window, Dialog, FS } from '~/api';
import { useAppListStore } from '~/stores/appList';
import { useConfigStore } from '~/stores/config';
import { Delete, Plus } from '@element-plus/icons-vue'
import { ElIcon, ElAlert, ElMessage } from 'element-plus'
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
                ElMessage.error('暂不支持非可执行文件');
                continue;
            }
            let name = FS.file_stem(f);
            let icon = await App.get_icon(f);
            if (icon.length == 0) {
                ElMessage.error('获取程序图标失败');
                continue;
            }
            local_apps.push({
                name: name,
                path: f,
                icon: icon
            });
            let status = applistStore.add(name, f, Url.convert(icon), [], null, () => {
                CLI.exec(['start', '', f]);
            });
            if (status != 'success') {
                ElMessage.error('添加失败，请勿重复添加！');
            }
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
    let name = FS.file_stem(f);
    let icon = await App.get_icon(f);
    if (icon.length == 0) {
        ElMessage.error('获取程序图标失败');
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
        ElMessage.error('添加失败，请勿重复添加！');
    }
}

</script>

<template>
    <div class="LocalExe">
        <div class="tip">
            <el-alert title="拖拽可执行文件到本页面可快速添加" center type="info" />
        </div>
        <div class="content">
            <div class="item" v-for="item in local_apps">
                <div class="info">
                    <img :src="Url.convert(item.icon)" alt="" srcset="">
                    <div>
                        <span class="name">{{ item.name }}</span>
                        <span class="path">{{ item.path }}</span>
                    </div>
                </div>
                <el-icon class="delete" @click="fun_delete(item.path)">
                    <Delete />
                </el-icon>
            </div>
        </div>
        <el-icon class="plus" @click="fun_add">
            <Plus />
        </el-icon>
    </div>
</template>

<style scoped lang="less">
.LocalExe {
    width: 100%;
    height: 100%;
    position: relative;
    display: flex;
    flex-direction: column;

    .tip {
        text-align: center;
    }

    .content {
        display: flex;
        flex-direction: column;
        flex-grow: 1;

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
                background-color: #383838;
                color: white;
                width: 26px;
                height: 26px;
                border-radius: 13px;
                cursor: pointer;
                font-size: 12px;

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
</style>