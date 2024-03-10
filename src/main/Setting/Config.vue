<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { AutoStart } from '~/api';
import { useConfigStore } from '~/stores/config';
import { ElSwitch, ElMessage, ElInput, ElForm, ElFormItem, ElCheckTag, ElRow } from 'element-plus';
const short_key = ref('');
const search_input_placeholder = ref('');
const auto_start = ref(false);

const configStore = useConfigStore();

onMounted(async () => {
    //读取唤出面板的快捷键
    let callout_shortkey = await configStore.read_callout();
    short_key.value = callout_shortkey.replace('CommandOrControl', 'Ctrl');
    //读取搜索栏占位符
    search_input_placeholder.value = await configStore.read_placeholder();
    auto_start.value = await AutoStart.is_set();
});

async function fun_shortkey_input(event: KeyboardEvent | Event) {
    let e = event as KeyboardEvent;
    if (e.key == 'Control' || e.key == 'Shift' || e.key == 'Alt' || e.key == 'Meta') {
        console.log(e);
        return;
    }
    console.log(e);
    //使用F1-F12注册快捷键
    let rf = /F[1-9][0-2]/g
    if (rf.test(e.key)) {
        configStore.write_callout(e.key);
        return;
    }
    //使用Ctrl+shift+Alt注册快捷键
    short_key.value = '';
    short_key.value += e.ctrlKey ? 'Ctrl+' : '';
    short_key.value += e.shiftKey ? 'Shift+' : '';
    short_key.value += e.altKey ? 'Alt+' : '';
    if (e.key == ' ' || e.key == "Process") {
        short_key.value += 'Space';
    } else {
        short_key.value += e.key.toUpperCase();
    }
    let sk = short_key.value.replace('Ctrl', 'CommandOrControl');
    configStore.write_callout(sk);
    return;
}

async function set_placeholder() {
    configStore.write_placeholder(search_input_placeholder.value);
}

async function fun_set_auto_start() {
    let ret = await AutoStart.set(auto_start.value);
    if (ret) {
        if (auto_start.value) {
            ElMessage.success('启用开启自启动');
        } else {
            ElMessage.warning('取消开机自启动');
        }
    } else {
        ElMessage.error('更改配置失败');
    }
}

function fun_set_hotkey(shortkey: string) {
    short_key.value = shortkey.replace("CommandOrControl", "Ctrl");
    configStore.write_callout(shortkey);
}

</script>

<template>
    <div class="Setting">
        <el-form label-position="right" label-width="100px" style="max-width: 460px">
            <el-form-item label="快捷键">
                <el-row>
                    <el-input v-model="short_key" placeholder="请输入热键"
                        input-style="color: transparent; text-shadow: 0 0 0 #90CAF9;text-align: center;"
                        style="width: 200px;height: 28px;" @keydown="fun_shortkey_input($event)" />
                </el-row>
                <el-row>
                    <el-check-tag
                        style="width: 80px;height: 25px;display: flex;justify-content: center; align-items: center; font-size: 10px; margin: 5px 5px 0;"
                        checked @click="fun_set_hotkey('CommandOrControl+Space')">
                        Ctrl+Space
                    </el-check-tag>
                    <el-check-tag
                        style="width: 80px;height: 25px;display: flex;justify-content: center; align-items: center; font-size: 10px; margin: 5px 5px 0;"
                        checked @click="fun_set_hotkey('Alt+Space')">
                        Alt+Space
                    </el-check-tag>
                </el-row>
            </el-form-item>
            <el-form-item label="占位符">
                <el-input v-model="search_input_placeholder" placeholder="请输入占位符"
                    input-style="text-align: center;color:#90CAF9;" style="width: 200px;height: 28px;"
                    @blur="set_placeholder" />
            </el-form-item>
            <el-form-item label="自启动">
                <el-switch v-model="auto_start" @change="fun_set_auto_start" />
            </el-form-item>
        </el-form>
    </div>
</template>


<style scoped lang="less">
.Setting {
    height: 100%;
    width: 100%;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    background-color: #252525;
    padding-top: 20px;
    padding-left: 150px;
}
</style>