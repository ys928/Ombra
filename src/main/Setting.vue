<script setup lang="ts">
import { onMounted } from 'vue';
import { useRouter } from 'vue-router';
import Window from '~/api/window';

const router = useRouter();

onMounted(() => {
});


function fun_to_main() {

    //跳转到主页面
    Window.set_size(170);
    Window.set_resizable(false);
    router.push(`/`);
}

</script>

<template>
    <div class="Setting">
        <div class="header" data-tauri-drag-region>
            <div class="name">
                <span>设置</span>
            </div>
            <div class="Icon" @click="fun_to_main">
                <img src="/logo.png" draggable="false">
            </div>
        </div>
        <div class="content">
            <div class="menu">
                <div class="item">软件配置</div>
                <div class="item">网页快开</div>
                <div class="item">本地应用</div>
            </div>
            <div class="panel">

            </div>
        </div>
    </div>
</template>

<style scoped lang="less">
.Setting {
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: column;

    .header {
        display: flex;
        justify-content: space-between;
        height: 50px;

        .name {
            color: #eee;
            font-size: 18px;
            line-height: 50px;
            margin: 0 20px;
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
        }

        .panel {
            width: 100px;
            height: 100%;
            flex-grow: 1;
        }
    }
}
</style>