import { defineStore } from "pinia";
import { Ref, ref } from "vue";
import { FS } from '~/api'
type MdFile = {
    name: string,
    path: string,
    content: string,
    save: boolean,
};

export const useOpenFilesStore = defineStore('markdown_openfiles', () => {
    //当前已打开的所有md文件
    const mdfiles = ref([]) as Ref<Array<MdFile>>;
    //当前显示的文件索引
    const show_index = ref(0);

    function set_show(index: number) {
        show_index.value = index;
    }

    async function open(path: string) {
        //防止重复打开相同文件
        for (let i = 0; i < mdfiles.value.length; i++) {
            if (mdfiles.value[i].path == path) {
                show_index.value = i;
                return;
            }
        }
        let name = FS.file_name(path);
        mdfiles.value.push({
            name: name,
            path: path,
            content: await FS.read_text(path),
            save: true
        });
        show_index.value = mdfiles.value.length - 1;
    }
    //打开一个默认空白的页面
    function open_default() {
        mdfiles.value.push({
            name: '未命名.md',
            path: '',
            content: '',
            save: true
        });
        show_index.value = mdfiles.value.length - 1;
    }

    function close(index: number) {
        if (index < show_index.value) {
            show_index.value = show_index.value - 1;
        } else if (index == show_index.value && index == mdfiles.value.length - 1) {
            show_index.value = show_index.value - 1;
        }
        mdfiles.value.splice(index, 1);
    }

    return { mdfiles, show_index, set_show, close, open, open_default }
})