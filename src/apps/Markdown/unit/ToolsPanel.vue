<script setup lang="ts">
import { ref } from 'vue';
import FileTree from './ToolsPanel/FileTree.vue';
import { Dialog, Dir, Path } from '~/api';

type FileInfo = {
    path: string,
    name: string,
};
type FileChildren = {
    dir: Array<FileInfo>,
    file: Array<FileInfo>
};

const root_path = ref('');

async function fun_opendir(f: string) {
    const entries = await Dir.walk(f);
    let children: FileChildren = {
        dir: [],
        file: []
    };
    for (const entry of entries) {
        if (entry.isdir) {
            children.dir.push({
                path: await Path.join(entry.path, entry.name),
                name: entry.name,
            });
        } else {
            if (entry.ext == 'md') {
                children.file.push({
                    path: await Path.join(entry.path, entry.name + ".md"),
                    name: entry.name + ".md",
                });
            }
        }
    }
    children.dir.sort(compareFileNames);
    children.file.sort(compareFileNames);

    return children;
}

// 文件名排序函数
function compareFileNames(a: FileInfo, b: FileInfo) {
    // 提取文件名中的数字部分
    function extractNumber(filename: string) {
        var match = filename.match(/\d+/);
        return match ? parseInt(match[0], 10) : NaN;
    }

    // 提取文件名中的非数字部分
    function extractNonNumber(filename: string) {
        return filename.replace(/\d+/g, '');
    }

    var numA = extractNumber(a.name);
    var numB = extractNumber(b.name);
    var nonNumA = extractNonNumber(a.name);
    var nonNumB = extractNonNumber(b.name);

    // 如果文件名中的数字相同，则按非数字部分的字母顺序排序
    if (numA === numB) {
        return nonNumA.localeCompare(nonNumB);
    }

    // 否则，按照数字的大小排序
    return numA - numB;
}

async function select_dir() {
    const selected = await Dialog.open({
        directory: true,
    })
    if (selected === null) return;
    root_path.value = selected as string;
}

</script>

<template>
    <div class="ToolsPanel">
        <FileTree :root="root_path" :fun_opendir="fun_opendir" :select_dir="select_dir">
        </FileTree>
    </div>
</template>

<style scoped lang="less">
</style>