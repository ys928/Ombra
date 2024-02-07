<script setup lang="ts">
import { Ref, h, onMounted, onUnmounted, ref, render } from 'vue';
import Config from './Setting/Config.vue';
import LocalExe from './Setting/LocalExe.vue';
import Web from './Setting/Web.vue';
import { Window } from '~/api';
import KIClose from '~/kui/icon/KIClose.vue';


const ref_div_panel = ref() as Ref<HTMLElement>;

const menu_items = [{
    name: '软件配置',
    item: Config,
}, {
    name: '本地应用',
    item: LocalExe,
},
{
    name: '网页快开',
    item: Web
}];

const show_item = ref(0);

onMounted(() => {
    render(h(menu_items[show_item.value].item), ref_div_panel.value);
    window.addEventListener('keydown', fun_keydown);
});

onUnmounted(() => {
    window.removeEventListener('keydown', fun_keydown);
})

function fun_switch_panel(index: number) {
    show_item.value = index;
    render(h(menu_items[show_item.value].item), ref_div_panel.value);
}

function fun_keydown(e: KeyboardEvent) {
    if (e.key == "Escape") {
        Window.to_main()
    }
}
</script>

<template>
    <div class="Setting">
        <div class="header" data-tauri-drag-region>
            <div class="label">
                <span class="name">设置</span>
                <KIClose :w="17" :h="17" @click="Window.to_main()"></KIClose>
            </div>
            <div class="Icon" @click="Window.to_main()">
                <img src="/logo.png" draggable="false">
            </div>
        </div>
        <div class="content">
            <div class="menu">
                <template v-for="(item, index) in menu_items">
                    <div class="item" :class="{ active: show_item == index }" @click="fun_switch_panel(index)">{{ item.name
                    }}
                    </div>
                </template>
            </div>
            <div class="panel" ref="ref_div_panel"></div>
        </div>
    </div>
</template>

<style scoped lang="less">
.Setting {
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: column;
    user-select: none;

    .header {
        display: flex;
        justify-content: space-between;
        height: 50px;

        .label {
            color: #eee;
            height: 36px;
            margin: 7px 20px;
            display: flex;
            border-radius: 18px;
            padding: 5px;
            align-items: center;
            border: 2px solid #444;
            background-color: #505050;
            cursor: pointer;

            .name {
                font-size: 15px;
                font-weight: bold;
                margin: 0 10px;
            }

            .KIcon {
                width: 24px;
                height: 24px;
                border-radius: 12px;
                color: #7F7F7F;

                &:hover {
                    background-color: #424242;
                    color: #ff0000;
                }
            }
        }

        .Icon {
            width: 50px;
            height: 50px;
            display: flex;
            align-items: center;
            user-select: none;
            cursor: pointer;
            will-change: filter;
            transition: 0.75s;

            img {
                width: 40px;
                height: 40px;
                border-radius: 20px;
            }

            &:hover {
                filter: drop-shadow(0 0 2em #eee);
            }
        }
    }

    .content {
        height: 100px;
        flex-grow: 1;
        display: flex;
        border-top: 2px solid #222;

        .menu {
            height: 100%;
            width: 150px;
            border-right: 2px solid #222;

            .item {
                color: #eee;
                padding: 10px 20px;
                cursor: pointer;

                &:hover {
                    background-color: #343434;
                }
            }

            .active {
                background-color: #37373D;
            }
        }

        .panel {
            width: 100px;
            height: 100%;
            flex-grow: 1;
        }
    }
}
</style>