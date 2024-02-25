<template>
    <div class="Labels">
        <template v-for="(item, index) in md_files">
            <span class="label" :class="{ 'actived': index == cur_show_file }" @click="change_file(index)">
                <span class="name">{{ item.name }}</span>
                <KIClose class="close" w="12" h="12" @click.stop="fun_close_file(index)"></KIClose>
            </span>
        </template>
    </div>
</template>

<script setup lang="ts">
import { Ref, inject } from 'vue';
import { KIClose } from '~/icon'
type md_file = {
    name: string,
    path: string,
    content: string
};
const md_files = inject('md_files') as Array<md_file>;
const cur_show_file = inject('cur_show_file') as Ref<number>;
const next_show_file = inject('next_show_file') as Ref<number>;

function change_file(index: number) {
    cur_show_file.value = index;
    next_show_file.value = index;
}

async function fun_close_file(index: number) {
    if (index < cur_show_file.value) {
        next_show_file.value = cur_show_file.value - 1;
    } else if (index == cur_show_file.value) {
        if (index == md_files.length - 1) {
            next_show_file.value = index - 1;
        }
    }
    md_files.splice(index, 1);
}

</script>


<style scoped lang="less">
.Labels {
    border-top: 1px solid #2b2b2b;
    border-bottom: 1px solid #2b2b2b;
    display: flex;
    justify-content: left;

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
    }

    .actived {
        border-top: 1px solid #0078d4;
        border-bottom: none;
    }
}
</style>