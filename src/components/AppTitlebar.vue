<template>
    <div class="AppTitlebar" data-tauri-drag-region>
        <div class="Icon">
            <img src="/logo.png">
        </div>
        <div class="mmc">
            <KISeparate class="sep" :w="12" :h="12" @click="winSeparate" v-if="app_is_embed()" title="分离窗口">
            </KISeparate>
            <KIMinus class="min" w="12" h="12" @click="WinMin"></KIMinus>
            <KIFullScreen class="max" w="12" h="12" @click="WinTogMax"></KIFullScreen>
            <KIClose class="close" w="12" h="12" @click="WinClose"></KIClose>
        </div>
    </div>
</template>

<script setup lang="ts">
import { KIMinus, KIFullScreen, KIClose, KISeparate } from '~/kui'
import { onMounted } from 'vue';
import { win_close, win_min, win_new_app, win_shadow, win_toggle_max, app_is_embed, om_get_appid } from '~/ombra';

async function winSeparate() {
    let appid = om_get_appid();
    if (appid.length > 0) {
        win_new_app(appid);
    }
}

onMounted(() => {
    win_shadow();
});

//处理程序退出时的情况
async function WinClose() {
    win_close();
}
async function WinMin() {
    win_min();
}
async function WinTogMax() {
    win_toggle_max();
}
</script>

<style scoped lang="less">
.AppTitlebar {
    height: 35px;
    line-height: 35px;
    display: flex;
    justify-content: space-between;

    .Icon {
        height: 35px;
        display: flex;
        align-items: center;
        margin-left: 10px;
        user-select: none;

        img {
            width: 16px;
            height: 16px;
        }
    }

    .search {
        margin: 6px 0;
        height: 23px;
        width: 250px;
        outline: none;
        color: #c2c2c2;
        border: 1px solid #454545;
        background-color: #242424;
        border-radius: 5px;
        padding: 0 10px;
        user-select: none;
    }

    .mmc {
        height: 35px;
        line-height: 35px;
        display: flex;
        align-items: center;

        .sep,
        .max,
        .min {
            width: 40px;
            text-align: center;
            color: #cccccc;
            cursor: pointer;

            &:hover {
                background-color: #373737;
            }
        }

        .close {
            width: 40px;
            text-align: center;
            color: #cccccc;
            cursor: pointer;

            &:hover {
                background-color: #e81123;
            }
        }
    }
}
</style>