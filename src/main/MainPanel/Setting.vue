<template>
    <div class="Setting">
        <div class="callout">
            <span>快捷键</span> <input v-model="short_key" @keydown="fun_shortkey_input($event)">
        </div>
        <div class="placeholder">
            <span>搜索框占位符</span> <input v-model="search_input_placeholder" @blur="set_placeholder">
        </div>
    </div>
</template>

<script setup lang="ts">

import { onMounted, ref } from 'vue';
const props = defineProps(['set_callout_shortkey', 'set_search_input_placeholder', 'callout_short_key', 'search_input_placeholder']);

const short_key = ref('');
const search_input_placeholder = ref('');

onMounted(async () => {
    short_key.value = props.callout_short_key.replace('CommandOrControl', 'Ctrl');
    search_input_placeholder.value = props.search_input_placeholder;
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
        props.set_callout_shortkey(e.key);
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
    props.set_callout_shortkey(sk);
    return;
}

async function set_placeholder() {
    props.set_search_input_placeholder(search_input_placeholder.value);
}

</script>

<style scoped lang="less">
.Setting {
    height: 100%;
    width: 100%;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    background-color: #252525;
    border-top: #494a4c solid 2px;
    padding-top: 20px;
    padding-left: 150px;

    &>div {
        margin-top: 20px;
        display: flex;
        justify-content: left;

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
            font-size: 20px;
            caret-color: transparent;

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
}
</style>