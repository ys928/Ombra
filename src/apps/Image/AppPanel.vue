<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import { Window, Url, Notification } from '~/api';

const img_src = ref("");


let uf_file_drag: UnlistenFn | undefined;

onMounted(async () => {
    uf_file_drag = await Window.event_file_drag((files) => {
        if (files[0].endsWith('.jpg') || files[0].endsWith('.jpeg') || files[0].endsWith('.png')) {
            img_src.value = Url.convert(files[0]);
        } else {
            Notification.send('提示', '暂时只支持jpg、png格式图片');
        }
    });
});

onUnmounted(() => {
    if (uf_file_drag) uf_file_drag();
})


</script>

<template>
    <div class="AppPanel">
        <div class="image">
            <img :src="img_src">
        </div>
        <div class="tools">
            <div class="item">图片压缩</div>
            <div class="item">格式转换</div>
        </div>
    </div>
</template>


<style scoped lang="less">
.AppPanel {
    width: 100%;
    height: 100%;
    position: relative;
    user-select: none;

    .image {
        width: 100%;
        height: 100%;
        display: flex;
        padding: 50px 10px;
        justify-content: center;
        align-items: center;
        overflow: hidden;

        img {
            max-width: 100%;
            max-height: 100%;
        }
    }

    .tools {
        height: 30px;
        width: 100%;
        position: absolute;
        left: 0;
        top: 0;
        color: #fff;
        display: flex;

        .item {
            font-size: 14px;
            line-height: 20px;
            margin: 3px 7px;
            padding: 2px 3px;
            border-radius: 5px;
            cursor: pointer;

            &:hover {
                background-color: #333;
            }
        }
    }

}
</style>