<template>
    <div class="Labels">
        <template v-for="(item, index) in openfilesStore.mdfiles">
            <span class="label" :class="{ 'actived': index == openfilesStore.show_index }" @click="change_file(index)">
                <span class="name">{{ item.name }}</span>
                <span class="save_status" v-if="!item.save"></span>
                <KIClose class="close" w="12" h="12" @click.stop="fun_close_file(index)"></KIClose>
            </span>
        </template>
    </div>
</template>

<script setup lang="ts">
import { KIClose } from '~/icon'
import { useOpenFilesStore } from '../../stores/openfiles'
import { Action, ElMessageBox } from 'element-plus'

const openfilesStore = useOpenFilesStore();

function change_file(index: number) {
    if (index == openfilesStore.show_index) return;

    //当前文件已保存，可直接切换
    if (openfilesStore.mdfiles[openfilesStore.show_index].save) {
        openfilesStore.set_show(index);
        return;
    }
    //否则，需请求保存、或放弃修改
    ElMessageBox.confirm("该文件尚未保存，切换文件将丢失数据！", '警告', {
        confirmButtonText: '切换',
        cancelButtonText: '取消',
        type: 'warning',
        callback: (action: Action) => {
            if (action == "confirm") {
                openfilesStore.set_show(index);
            }
        }
    });

}

async function fun_close_file(index: number) {
    if (openfilesStore.mdfiles[index].save) {
        openfilesStore.close(index)
        return;
    }
    ElMessageBox.confirm("该文件尚未保存，关闭文件将丢失数据！", '警告', {
        confirmButtonText: '关闭',
        cancelButtonText: '取消',
        type: 'warning',
        callback: (action: Action) => {
            if (action == "confirm") {
                openfilesStore.close(index)
            }
        }
    });
}

</script>


<style scoped lang="less">
.Labels {
    border-top: 1px solid #2b2b2b;
    border-bottom: 1px solid #2b2b2b;
    display: flex;
    justify-content: left;
    overflow-y: hidden;
    overflow-x: auto;

    &::-webkit-scrollbar {
        height: 4px;
    }

    &::-webkit-scrollbar-thumb {
        background-color: #4D4D4D;
        border-radius: 2px;
    }

    &::-webkit-scrollbar-track {
        background-color: transparent;
    }

    .label {
        color: #9d9d9d;
        padding-left: 10px;
        padding-right: 5px;
        display: inline-block;
        height: 30px;
        border: 1px solid #2b2b2b;
        font-size: 13px;
        cursor: pointer;
        user-select: none;
        display: flex;
        align-items: center;

        &:hover {
            background-color: #1f1f1f;
        }

        .name {
            max-width: 200px;
            overflow: hidden;
            display: inline-block;
            word-break: break-all;
            white-space: nowrap;
            text-overflow: ellipsis;
            margin: 0 5px;
            text-align: center;
            line-height: 30px;
        }

        .close {
            width: 20px;
            height: 20px;
            line-height: 20px;
            display: inline-block;
            text-align: center;
            border-radius: 5px;

            &:hover {
                background-color: #313232;
            }
        }

        .save_status {
            width: 6px;
            height: 6px;
            border-radius: 3px;
            background-color: #AD946F;
            margin: 0 3px;
        }
    }

    .actived {
        border-top: 1px solid #0078d4;
        border-bottom: none;
    }
}
</style>