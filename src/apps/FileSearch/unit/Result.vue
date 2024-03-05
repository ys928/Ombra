<script setup lang="ts">
import { KIDll, KIText, KIFolder, KITypeScript, KIHtml, KIPdf, KIJs, KIJson, KIUnknowFile, KIImage } from '~/icon'
import { computed, ref } from 'vue';
import { Path, Explorer, Tools } from '~/api'
import { useSearchResultStore } from '../stores/searchResult';
import { usePopMenuStore } from '../stores/popMenu';
import { vLoading } from 'element-plus'

const searchResultStore = useSearchResultStore();

const popMenuStore = usePopMenuStore();

const logo_size = ref({
    w: 13,
    h: 13,
})

function fun_file_item_contextmenu(e: MouseEvent, item: FileInfo) {
    popMenuStore.set_show(true);
    popMenuStore.set_pos(e.clientX, e.clientY);
    popMenuStore.set_click_item(item);
}

function handle_scroll(e: Event) {
    if (searchResultStore.scroll_count == -1) return;
    const { scrollTop, clientHeight, scrollHeight } = e.target as HTMLElement;

    if (Math.ceil(scrollTop) + clientHeight >= scrollHeight) {
        searchResultStore.set_scroll_count(searchResultStore.scroll_count + 1);
        searchResultStore.search(searchResultStore.last_search.name, searchResultStore.last_search.ext, searchResultStore.last_search.mode, searchResultStore.scroll_count * 50);
    }
}

function fun_show_file_name(name: string, ext: string, isdir: boolean) {
    if (searchResultStore.last_search.mode == 'normal') {
        let pos = name.toLowerCase().indexOf(searchResultStore.last_search.name.toLowerCase());
        let s = Tools.get_span(name.substring(0, pos), 'normal');
        s += Tools.get_span(name.substring(pos, pos + searchResultStore.last_search.name.length), 'light');
        s += Tools.get_span(name.substring(pos + searchResultStore.last_search.name.length), 'normal');
        if (isdir) {
            return s;
        }
        if (searchResultStore.last_search.ext.length > 0) {
            let pos = ext.toLocaleLowerCase().indexOf(searchResultStore.last_search.ext);
            s += Tools.get_span(".", 'normal');
            s += Tools.get_span(ext.substring(0, pos), 'normal');
            s += Tools.get_span(ext.substring(pos, pos + searchResultStore.last_search.ext.length), 'light');
            s += Tools.get_span(ext.substring(pos + searchResultStore.last_search.ext.length), 'normal');
        } else {
            s += Tools.get_span('.' + ext, 'normal');
        }
        return s;
    } else if (searchResultStore.last_search.mode == 'exact') {
        let show_name = name;
        if (searchResultStore.last_search.ext.length > 0) {
            show_name += '.' + ext;
        }
        return Tools.get_span(show_name, 'light');
    }
    return Tools.get_span(name, 'normal');
}
async function fun_dbclick(item: FileInfo) {
    let p = item.name;
    if (item.ext.length > 0) {
        p += '.' + item.ext;
    }
    if (item.path.length != 0) {
        p = await Path.join(item.path, p);
    }
    Explorer.open_file(p);
}

//得到文件类型的计算属性
const file_type = computed(() => (ext: string) => {
    switch (ext) {
        case 'dll': return 'dll';
        case 'exe': return 'exe';
        case 'txt': return 'text';
        case 'ts': return 'typescript';
        case 'pdf': return 'pdf';
        case 'js': return 'javascript';
        case 'json': return 'json';
        case 'html':
        case 'htm':
            return 'html';
        case 'png':
        case 'jpg':
        case 'jpeg':
        case 'icon':
        case 'bmp':
        case 'gif':
            return 'image';

    }
});

</script>

<template>
    <div class="Result" v-loading="searchResultStore.is_processing || searchResultStore.is_searching"
        @scroll="handle_scroll($event)">
        <div class="table_header">
            <span class="name">名称</span>
            <span class="path">路径</span>
            <span class="time">修改时间</span>
        </div>
        <div v-for="item in searchResultStore.result" class="item"
            @contextmenu="fun_file_item_contextmenu($event, item)" @dblclick="fun_dbclick(item)">
            <span class="name" :title="item.name">
                <KIFolder :w="logo_size.w" :h="logo_size.h" v-if="item.isdir"></KIFolder>
                <KIDll :w="logo_size.w" :h="logo_size.h" v-else-if="file_type(item.ext) == 'dll'"></KIDll>
                <KIText :w="logo_size.w" :h="logo_size.h" v-else-if="file_type(item.ext) == 'text'"></KIText>
                <KITypeScript :w="logo_size.w" :h="logo_size.h" style="color: #4f9aba;"
                    v-else-if="file_type(item.ext) == 'typescript'">
                </KITypeScript>
                <KIHtml :w="logo_size.w" :h="logo_size.h" style="color: #e37933;"
                    v-else-if="file_type(item.ext) == 'html'">
                </KIHtml>
                <KIPdf :w="logo_size.w" :h="logo_size.h" v-else-if="file_type(item.ext) == 'pdf'"></KIPdf>
                <KIJs :w="logo_size.w" :h="logo_size.h" v-else-if="file_type(item.ext) == 'javascript'"></KIJs>
                <KIJson :w="logo_size.w" :h="logo_size.h" style="color: #cbcb41;"
                    v-else-if="file_type(item.ext) == 'json'">
                </KIJson>
                <KIImage :w="logo_size.w" :h="logo_size.h" style="color: #A074C4;"
                    v-else-if="file_type(item.ext) == 'image'">
                </KIImage>
                <KIUnknowFile :w="logo_size.w" :h="logo_size.h" v-else></KIUnknowFile>
                <span v-html="fun_show_file_name(item.name, item.ext, item.isdir)"></span>
            </span>
            <span class="path" :title="item.path">{{ item.path }}</span>
            <span class="time">{{ Tools.time_to_str(item.time) }}</span>
        </div>
    </div>
</template>

<style scoped lang="less">
.Result {
    height: 100px;
    flex-grow: 1;
    color: #aaa;
    overflow-y: auto;
    overflow-x: hidden;
    user-select: none;

    &::-webkit-scrollbar {
        width: 6px;
    }

    &::-webkit-scrollbar-thumb {
        background-color: #666;
        border-radius: 3px;
    }

    &::-webkit-scrollbar-track {
        background-color: #181818;
    }

    .table_header {
        margin-top: 15px;
        display: flex;
        font-size: 13px;
        padding: 0 10px;

        span {
            display: inline-block;
        }

        .name {
            width: 200px;
        }

        .path {
            width: 400px;
            margin: 0 10px;
        }

        .time {
            flex-grow: 1;
        }

    }

    .item {
        font-size: 13px;
        display: flex;
        padding: 0 10px;
        height: 24px;

        &:hover {
            background-color: #454545;
            cursor: pointer;
        }

        span {
            white-space: nowrap;
            display: inline-block;
            overflow: hidden;
            text-overflow: ellipsis;
        }

        .name {
            width: 200px;
            display: flex;
            align-items: center;

            .KIcon {
                margin-right: 3px;
                display: flex;
                align-items: center;
            }

            :deep(.normal) {
                color: #aaa;
            }

            :deep(.light) {
                color: #4fc1ff;
            }

        }

        .path {
            width: 400px;
            margin: 0 10px;
            line-height: 24px;
        }

        .time {
            flex-grow: 1;
            line-height: 24px;
        }
    }
}
</style>