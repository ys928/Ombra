<script setup lang="ts">
import { onMounted, ref } from 'vue';
import Config from '~/api/config';
import AutoStart from '~/api/autostart';
import Notification from '~/api/notification';

const short_key = ref('');
const search_input_placeholder = ref('');
const auto_start = ref(false);


onMounted(async () => {
    //读取唤出面板的快捷键
    let callout_shortkey = await Config.read_item('callout');
    short_key.value = callout_shortkey.replace('CommandOrControl', 'Ctrl');
    //读取搜索栏占位符
    search_input_placeholder.value = await Config.read_item('placeholder');
    auto_start.value = await AutoStart.is_set();
});




async function fun_shortkey_input(e: KeyboardEvent) {
    e.preventDefault();
    e.stopPropagation();
    console.log(e);
    if (e.key == 'Control' || e.key == 'Shift' || e.key == 'Alt' || e.key == 'Meta') {
        return;
    }
    //使用F1-F12注册快捷键
    let rf = /F[1-9][0-2]/g
    if (rf.test(e.key)) {
        Config.write_item('callout', e.key);
        return;
    }
    //使用Ctrl+shift+Alt注册快捷键
    short_key.value = '';
    short_key.value += e.ctrlKey ? 'Ctrl+' : '';
    short_key.value += e.shiftKey ? 'Shift+' : '';
    short_key.value += e.altKey ? 'Alt+' : '';
    if (e.key == ' ') {
        short_key.value += 'Space';
    } else {
        short_key.value += e.key.toUpperCase();
    }
    let sk = short_key.value.replace('Ctrl', 'CommandOrControl');
    Config.write_item('callout', sk);
    return;
}

async function set_placeholder() {
    Config.write_item('placeholder', search_input_placeholder.value);
}

async function fun_set_auto_start() {
    if (auto_start.value) {
        let ret = await AutoStart.set(false);
        if (ret) {
            auto_start.value = false;
        } else {
            Notification.send('错误', "设置开机自启动失败");
        }
    } else {
        let ret = await AutoStart.set(true);
        if (ret) {
            auto_start.value = true;
        } else {
            Notification.send('错误', "关闭开机自启动失败");
        }
    }
}

</script>

<template>
    <div class="Setting">
        <div class="callout">
            <span>快捷键</span> <input v-model="short_key" @keydown="fun_shortkey_input($event)">
        </div>
        <div class="placeholder">
            <span>搜索框占位符</span> <input v-model="search_input_placeholder" @blur="set_placeholder">
        </div>
        <div class="autostart">
            <span>开机自启动</span> <input v-model="auto_start" type="checkbox" @click="fun_set_auto_start">
        </div>
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

    &>div {
        margin-top: 20px;
        display: flex;
        justify-content: left;
        align-items: center;

        span {
            color: #e9e9e9;
            width: 150px;
            text-align: right;
            margin-right: 20px;
        }
    }

    .callout {

        input {
            outline: none;
            background-color: #2b2b2b;
            border: #616264 solid 1px;
            width: 200px;
            height: 25px;
            border-radius: 5px;
            color: #90caf9;
            text-align: center;
            font-size: 16px;
            caret-color: transparent;
            cursor: pointer;

            &:hover {
                border: #ffffff solid 1px;
            }

            &:focus {
                border: #90caf9 solid 1px;
            }
        }
    }

    .placeholder {
        input {
            outline: none;
            background-color: #2b2b2b;
            border: #616264 solid 1px;
            width: 200px;
            height: 25px;
            border-radius: 5px;
            color: #979798;
            text-align: left;
            font-size: 16px;
            padding: 0 20px;

            &:hover {
                border: #ffffff solid 1px;
            }

            &:focus {
                border: #90caf9 solid 1px;
            }
        }
    }

    .autostart {
        input {
            width: 16px;
            height: 16px;
            background-color: #252525;
            border: 1px solid #616264;
            border-radius: 4px;
            color: #fff;
            text-align: center;
            line-height: 15px;
            -webkit-appearance: none;
            appearance: none;
            outline: none;
            cursor: pointer;

            &:hover {
                border: 1px solid #ffffff;
            }

            &:checked {
                color: #fff;
                background-color: #4e8cf2;
                border: 1px solid #4e8cf2;
            }

            &:after {
                content: "\2713";
                color: #252525;
            }
        }
    }
}
</style>