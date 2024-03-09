<script setup lang="ts">



</script>

<template>
    <div class="Blank">
        <div class="action">
            <div class="item">新建文件</div>
            <div class="item">切换全屏</div>
        </div>
        <div class="shortcut">
            <div class="item">
                <span class="short">Ctrl</span>+<span class="short">N</span>
            </div>
            <div class="item">
                <span class="short">F11</span>
            </div>
        </div>

    </div>
</template>

<style scoped lang="less">
.Blank {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    user-select: none;

    .action {
        text-align: right;
        margin: 0 10px;
        font-size: 14px;
        color: #727272;

        .item {
            height: 25px;
            margin: 20px 0;
        }
    }

    .shortcut {
        text-align: left;
        font-size: 12px;

        .item {
            height: 25px;
            margin: 20px 0;

            .short {
                font-size: 12px;
                padding: 2px 5px;
                font-weight: 500;
                background-color: #2C2C2C;
                border-radius: 5px;
                margin: 0 5px;
                color: #A2A2A2;
            }
        }
    }
}
</style>