<script setup lang="ts">
import { onMounted, onUnmounted, reactive, ref } from 'vue';
import { Window, Url, Notification, Img } from '~/api';
import { KLoading } from '~/kui'

let imgs_path = reactive([]) as Array<string>;

let uf_file_drag: UnlistenFn | undefined;

let show_img_path = ref("");

let show_loading = ref(false);

onMounted(async () => {
    uf_file_drag = await Window.event_file_drag((files) => {
        for (let file of files) {
            if (file.endsWith('.jpg') || file.endsWith('.jpeg') || file.endsWith('.png')) {
                imgs_path.push(file);
            } else {
                Notification.send('提示', '暂时只支持jpg、jpeg、png格式图片');
            }
        }
        if (show_img_path.value.length == 0 && imgs_path.length > 0) {
            show_img_path.value = imgs_path[0];
        }
    });
});

onUnmounted(() => {
    if (uf_file_drag) uf_file_drag();
})

async function compress() {
    if (show_loading.value) {
        Notification.send('提示', '正在压缩中……');
        return;
    }
    if (show_img_path.value.length == 0) {
        Notification.send('提示', '还未选择图片');
        return;
    }
    // let name = FS.file_stem(imgs_path[0]);
    // let ext = FS.extension(imgs_path[0]);
    // let parent = FS.parent(imgs_path[0]);
    // let newpath = await Path.join(parent, name + '_om.' + ext);
    show_loading.value = true;
    let ret = await Img.compress(show_img_path.value, show_img_path.value, 70);
    show_loading.value = false;
    if (ret) {
        Notification.send('提示', '压缩成功');
    } else {
        Notification.send('提示', '压缩失败');
    }
}


</script>

<template>
    <div class="AppPanel">
        <div class="image" ref="ref_show_img">
            <img v-show="show_img_path.length > 0" :src="Url.convert(show_img_path)" alt="">
        </div>
        <div class="tools">
            <div class="item" @click="compress">图片压缩</div>
            <div class="item">格式转换</div>
        </div>
        <KLoading v-if="show_loading"></KLoading>
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
            border: 1px solid #383838;

            &:hover {
                background-color: #272727;
                border: 1px solid #C2C2C2;
            }
        }
    }

    .KLoading {
        width: 50px;
        height: 50px;
        position: fixed;
        left: 50%;
        top: 50%;
        transform: translate(-50%, -50%);
    }
}
</style>