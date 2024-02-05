<script setup lang="ts">
const props = defineProps(['isshow']);
const emits = defineEmits(['update:isshow', 'cancel', 'sure'])


function fun_cancel() {
    emits('cancel')
}

function fun_sure() {
    emits('sure');
}

</script>

<template>
    <div class="KWindow" v-if="props.isshow">
        <div class="panel">
            <div class="header">
            </div>
            <div class="content">
                <slot></slot>
            </div>
            <div class="footer">
                <button class="cancel" @click="fun_cancel">取消</button>
                <button class="sure" @click="fun_sure">保存</button>
            </div>
        </div>
    </div>
</template>

<style scoped lang="less">
.KWindow {
    width: 100vw;
    height: 100vh;
    position: fixed;
    z-index: 999;
    left: 0;
    top: 0;

    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;

    .panel {
        background-color: #2E2E2E;
        width: 600px;
        height: 400px;
        display: flex;
        flex-direction: column;
        border-radius: 10px;

        .header {
            height: 40px;
        }

        .content {
            flex-grow: 1;
        }

        .footer {
            height: 40px;
            width: 100%;
            display: flex;
            justify-content: space-around;

            button {
                color: #2e2e2e;
                width: 60px;
                height: 30px;
                border-radius: 5px;
                border: none;
                cursor: pointer;
                font-size: 15x;
            }

            .sure {
                background-color: #90CAF9;
                color: #111;

                &:hover {
                    background-color: #648DAE;
                }
            }

            .cancel {
                background-color: transparent;
                color: #aa8390;
                border: 1px solid #aa8390;

                &:hover {
                    background-color: #473E41;
                }
            }
        }
    }


}
</style>