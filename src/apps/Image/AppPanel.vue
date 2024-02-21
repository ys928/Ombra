<script setup lang="ts">
import { Ref, onMounted, onUnmounted, reactive, ref } from 'vue';
import { Window, Url, Img, Dialog, FS } from '~/api';
import { KIPlus } from '~/icon'
import { ElMessage, ElForm, ElFormItem, ElDialog, vLoading, ElButton, ElDropdown, ElDropdownMenu, ElDropdownItem, ElSlider, ElRow, ElCol } from 'element-plus'
let imgs_path = reactive([]) as Array<string>;

let uf_file_drag: UnlistenFn | undefined;

let show_img_path = ref("");

let show_loading = ref(false);

const ref_image = ref() as Ref<HTMLElement>

const is_show_setting = ref(false);

const compress_ratio = ref(70);

//图片属性
const img_attr = reactive({
    x: 0,
    y: 0,
    ow: 0,
    oh: 0,
    scale: 1,
    drag: false,
});

const ref_thumbnail = ref() as Ref<HTMLElement>;

onMounted(async () => {
    uf_file_drag = await Window.event_file_drag((files) => {
        for (let file of files) {
            if (file.endsWith('.jpg') || file.endsWith('.jpeg') || file.endsWith('.png')) {
                imgs_path.push(file);
            } else {
                ElMessage.warning("暂时只支持jpg、jpeg、png格式图片");
            }
        }
        if (show_img_path.value.length == 0 && imgs_path.length > 0) {
            fun_change_img(imgs_path[0]);
        }
    });
});

onUnmounted(() => {
    if (uf_file_drag) uf_file_drag();

})

async function compress(command: string) {
    if (show_loading.value) {
        ElMessage.warning('已有任务处理中……');
        return;
    }

    if (show_img_path.value.length == 0) {
        ElMessage.error('还未选择图片');
        return;
    }

    if (command == 'direct') {
        show_loading.value = true;
        ElMessage.info('开始压缩……');
        let ret = await Img.compress(show_img_path.value, show_img_path.value, compress_ratio.value);
        show_loading.value = false;
        if (ret) {
            ElMessage.success('压缩成功');
        } else {
            ElMessage.error('压缩失败');
        }
        return;
    }

    if (command == 'save') {
        let ext = FS.extension(show_img_path.value);
        let savefile = await Dialog.save({
            title: '保存压缩图像文件',
            filters: [{
                name: 'image',
                extensions: [ext]
            }]
        });
        if (savefile == null) return;
        show_loading.value = true;
        ElMessage.info('开始压缩……');
        let ret = await Img.compress(show_img_path.value, savefile as string, compress_ratio.value);
        show_loading.value = false;
        if (ret) {
            ElMessage.success('压缩成功');
        } else {
            ElMessage.error('压缩失败');
        }
    }
}

async function format_conversion(command: string) {
    if (show_loading.value) {
        ElMessage.warning('已有任务处理中……');
        return;
    }
    if (show_img_path.value.length == 0) {
        ElMessage.error('还未选择图片');
        return;
    }

    if (command == 'jpg') {
        let savefile = await Dialog.save({
            title: '保存压缩图像文件',
            filters: [{
                name: 'image',
                extensions: ['jpg']
            }]
        });
        if (savefile == null) return;
        show_loading.value = true;
        ElMessage.info('开始转换……');
        let ret = await Img.convert(show_img_path.value, savefile as string, "jpg");
        show_loading.value = false;
        if (ret) {
            ElMessage.success('转换成功');
        } else {
            ElMessage.error('转换失败');
        }
    } else if (command == "png") {
        let savefile = await Dialog.save({
            title: '保存压缩图像文件',
            filters: [{
                name: 'image',
                extensions: ['png']
            }]
        });
        if (savefile == null) return;
        show_loading.value = true;
        ElMessage.info('开始转换……');
        let ret = await Img.convert(show_img_path.value, savefile as string, "png");
        show_loading.value = false;
        if (ret) {
            ElMessage.success('转换成功');
        } else {
            ElMessage.error('转换失败');
        }
    } else if (command == "bmp") {
        let savefile = await Dialog.save({
            title: '保存压缩图像文件',
            filters: [{
                name: 'image',
                extensions: ['bmp']
            }]
        });
        if (savefile == null) return;
        show_loading.value = true;
        ElMessage.info('开始转换……');
        let ret = await Img.convert(show_img_path.value, savefile as string, "bmp");
        show_loading.value = false;
        if (ret) {
            ElMessage.success('转换成功');
        } else {
            ElMessage.error('转换失败');
        }
    }
}

async function fun_select_pic() {
    let ret = await Dialog.open({
        title: '选择图片',
        multiple: true,
        filters: [{
            name: '图片',
            extensions: ['jpg', 'jpeg', 'png']
        }]
    }) as null | string[];
    if (ret == null) return;
    for (let i = 0; i < ret.length; i++) {
        imgs_path.push(ret[i]);
    }
    fun_change_img(ret[0]);
}

function fun_wheel(e: WheelEvent) {
    ref_thumbnail.value.scrollLeft += e.deltaY;
    e.preventDefault();
}

function fun_img_mousedown(e: MouseEvent) {
    e.preventDefault();
    img_attr.drag = true;
    document.addEventListener('mouseup', fun_img_mouseup);
}

function fun_img_mouseup(e: MouseEvent) {
    e.preventDefault();
    img_attr.drag = false;
    document.removeEventListener('mouseup', fun_img_mouseup);

    let img = ref_image.value.querySelector('img');
    if (img == null) return;
    img.style.cursor = "grab";
}

function fun_img_move(e: MouseEvent) {
    e.preventDefault();
    if (img_attr.drag) {
        img_attr.x += e.movementX;
        img_attr.y += e.movementY;

        let img = ref_image.value.querySelector('img');
        if (img == null) return;
        img.style.cursor = "grabbing";
    }
}

function fun_change_img(path: string) {
    show_img_path.value = path;
    img_attr.x = 0;
    img_attr.y = 0;
    img_attr.oh = 0;
    img_attr.ow = 0;
    img_attr.scale = 1;
}

function fun_img_wheel(e: WheelEvent) {
    e.preventDefault();
    let img = ref_image.value.querySelector('img');
    if (img == null) return;
    if (img_attr.ow == 0) {
        img_attr.ow = img.clientWidth;
        img_attr.oh = img.clientHeight;
        img.style.width = img_attr.ow + 'px';
        img.style.height = img_attr.oh + 'px';
        img.style.maxWidth = 'none';
        img.style.maxHeight = 'none';
    }
    let ratio = 1.1; //缩放比例
    // 缩小
    if (e.deltaY > 0) {
        ratio = 0.9;
    }
    // 限制缩放倍数
    const scale = img_attr.scale * ratio;
    if (scale >= 32 || scale <= 0.1) return;

    img_attr.scale = scale;

    let target = e.target as HTMLElement | null;
    if (target && target.tagName === "IMG") {
        let r = img.getBoundingClientRect();
        let rx = e.clientX - (r.left - 0.5 * r.width * (ratio - 1)) - (e.clientX - r.left) * ratio;
        let ry = e.clientY - (r.top - 0.5 * r.height * (ratio - 1)) - (e.clientY - r.top) * ratio;
        img_attr.x += rx;
        img_attr.y += ry;
    }
    img.style.width = `${img_attr.scale * img_attr.ow}px`;
    img.style.height = `${img_attr.scale * img_attr.oh}px`;
}

</script>
 
<template>
    <div class="AppPanel">
        <div class="image" ref="ref_image" @wheel="fun_img_wheel" v-loading="show_loading">
            <img v-if="show_img_path.length > 0" :src="Url.convert(show_img_path)" @mousedown="fun_img_mousedown($event)"
                @mouseup="fun_img_mouseup" @mousemove="fun_img_move($event)"
                :style="{ left: img_attr.x + 'px', top: img_attr.y + 'px' }">
            <div v-else class="select" @click="fun_select_pic">
                <KIPlus :w="50" :h="50"></KIPlus>
            </div>
        </div>
        <div class="thumbnail" ref="ref_thumbnail" @wheel="fun_wheel($event)">
            <template v-for="item in imgs_path">
                <div class="item" :class="{ active: item == show_img_path }" @click="fun_change_img(item)">
                    <img :src="Url.convert(item)" alt="">
                </div>
            </template>
        </div>
        <el-row class="tools" justify="space-around">
            <el-col :span="5">
                <el-dropdown trigger="click" @command="compress">
                    <el-button text bg size="small">图片压缩</el-button>
                    <template #dropdown>
                        <el-dropdown-menu>
                            <el-dropdown-item command="direct">原地压缩</el-dropdown-item>
                            <el-dropdown-item command="save">另存压缩</el-dropdown-item>
                            <el-dropdown-item>批量原地压缩</el-dropdown-item>
                            <el-dropdown-item>批量另存压缩</el-dropdown-item>
                        </el-dropdown-menu>
                    </template>
                </el-dropdown>
            </el-col>
            <el-col :span="5">
                <el-dropdown trigger="click" @command="format_conversion">
                    <el-button text bg size="small">格式转换</el-button>
                    <template #dropdown>
                        <el-dropdown-menu>
                            <el-dropdown-item command="jpg">.jpg</el-dropdown-item>
                            <el-dropdown-item command="png">.png</el-dropdown-item>
                            <el-dropdown-item command="bmp">.bmp</el-dropdown-item>
                        </el-dropdown-menu>
                    </template>
                </el-dropdown>
            </el-col>
            <el-col :span="5">
                <el-button text bg size="small" @click="is_show_setting = true">设置</el-button>
            </el-col>
        </el-row>
        <el-dialog class="Setting" v-model="is_show_setting" title="设置" width="600">
            <el-form label-position="right" label-width="100px" style="max-width: 550px">
                <el-form-item label="压缩比例">
                    <el-slider v-model="compress_ratio" :step="5" show-stops />
                </el-form-item>
            </el-form>
        </el-dialog>
    </div>
</template>


<style scoped lang="less">
.AppPanel {
    width: 100%;
    height: 100%;
    position: relative;
    user-select: none;
    display: flex;
    flex-direction: column;
    align-items: center;

    .image {
        width: 100%;
        height: 100px;
        flex-grow: 1;
        display: flex;
        margin: 40px 10px 0;
        justify-content: center;
        align-items: center;
        overflow: hidden;
        position: relative;

        img {
            position: relative;
            max-width: 100%;
            max-height: 100%;
            left: 0;
            top: 0;
            cursor: grab;
        }

        .select {
            width: 100px;
            height: 100px;
            border-radius: 5px;
            border: 1px dashed #373737;
            display: flex;
            justify-content: center;
            align-items: center;
            color: #909399;
            cursor: pointer;

            &:hover {
                border: 1px dashed #409EFF;
            }
        }
    }

    .thumbnail {
        height: 80px;
        margin: 10px 0;
        display: flex;
        justify-content: center;
        width: 80%;
        overflow: auto;
        flex-flow: row;
        align-items: center;

        &::-webkit-scrollbar {
            height: 6px;
        }

        &::-webkit-scrollbar-thumb {
            background-color: #666;
            border-radius: 3px;
        }

        &::-webkit-scrollbar-track {
            background-color: #181818;
        }

        .item {
            width: 60px;
            height: 60px;
            background-color: #292929;
            flex-shrink: 0;
            display: flex;
            justify-content: center;
            align-items: center;
            border-radius: 5px;
            cursor: pointer;
            margin: 0 5px;
            border: 2px solid #292929;
            padding: 5px;

            img {
                max-width: 55px;
                max-height: 55px;
            }

            &:hover {
                border: 2px solid #8C939F;
            }
        }

        .active {
            border: 2px solid #2C71FA;

            &:hover {
                border: 2px solid #2C71FA;
            }
        }

    }

    .tools {
        height: 20px;
        width: 100%;
        position: absolute;
        left: 0;
        top: 5px;
    }
}
</style>