<script setup lang="ts">
import { onMounted, reactive, ref } from 'vue';
import { useAppListStore } from '~/stores/appList';
import { useConfigStore } from '~/stores/config';
import { Url } from '~/api';
import { ElDialog, ElButton, ElIcon, ElMessageBox, ElSwitch, ElInput, ElForm, ElFormItem, ElMessage } from 'element-plus'
import { Delete, Plus } from '@element-plus/icons-vue'
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
        applistStore.add_web({
            name: item.name,
            features: item.features,
            url: item.url,
            icon: item.icon.length > 0 ? Url.convert(item.icon) : '/logo.png',
            on: true
        });
    } else {
        applistStore.remove(item.url);
    }
}
async function fun_add_web() {
    if (add_web_name.value.length == 0) {
        ElMessage.warning("未填写名称");
        return;
    }
    if (add_web_url.value.length == 0) {
        ElMessage.warning("未填写网址");
        return;
    }
    ElMessage.info("获取网站图标……");
    let fav_icon = await Url.download_favicon(add_web_url.value, add_web_name.value);
    if (fav_icon.length == 0) {
        ElMessage.warning("获取网站图标失败，将使用默认图标");
    } else {
        ElMessage.success("已添加");
    }
    let is_query = add_web_name.value.indexOf('{query}') != -1;

    let item = {
        name: add_web_name.value,
        url: add_web_url.value,
        features: is_query ? ['text'] : [],
        icon: fav_icon,
        on: true,
    }

    if (fav_icon.length > 0) {
        applistStore.add_web({
            name: add_web_name.value,
            url: add_web_url.value,
            features: is_query ? ['text'] : [],
            icon: Url.convert(fav_icon),
            on: true,
        });
    } else {
        applistStore.add_web(item);
    }

    web_apps.push(item);

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
    let ret = await ElMessageBox.confirm('确定要删除？', '提示', {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
    });
    if (ret != 'confirm') return;
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
                    <el-switch v-model="item.on" @click="fun_change(item)"></el-switch>
                    <el-icon @click="fun_delete(item.url)">
                        <Delete />
                    </el-icon>
                </div>
            </div>
            <el-icon class="plus" @click="is_show_window = true">
                <Plus />
            </el-icon>
        </div>
        <el-dialog v-model="is_show_window">
            <el-form label-position="right" label-width="100px" style="max-width: 460px">
                <el-form-item label="名称">
                    <el-input v-model="add_web_name" placeholder="搜索关键字" style="width: 200px;height: 28px;" />
                </el-form-item>
                <el-form-item label="链接">
                    <el-input v-model="add_web_url" placeholder="https://...." style="width: 200px;height: 28px;" />
                </el-form-item>
            </el-form>
            <template #footer>
                <el-button @click="is_show_window = false">取消</el-button>
                <el-button type="primary" @click="fun_add_web">确定</el-button>
            </template>
        </el-dialog>
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

                .el-icon {
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
}
</style>