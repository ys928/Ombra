import { defineStore } from "pinia";
import { onMounted, onUnmounted, ref } from "vue";

export const usePopMenuStore = defineStore('file_search_pop_menu', () => {
    const x = ref(0);
    const y = ref(0);
    const show = ref(false);

    const click_item = ref({
        name: "",
        ext: "",
        path: "",
        time: 0,
        isdir: false,
        index: -1
    });

    let fun_click_document = () => {
        show.value = false;
        click_item.value.index = -1;
    }

    onMounted(() => {
        document.addEventListener('click', fun_click_document)
    });

    onUnmounted(() => {
        document.removeEventListener('click', fun_click_document)
    });

    function set_pos(_x: number, _y: number) {
        x.value = _x;
        y.value = _y;
        show.value = true;
    }

    function set_show(is_show: boolean) {
        show.value = is_show;
        if (is_show == false) {
            click_item.value.index = -1;
        }
    }

    function set_click_item(item: FileInfo, index: number) {
        click_item.value.name = item.name;
        click_item.value.ext = item.ext;
        click_item.value.path = item.path;
        click_item.value.time = item.time;
        click_item.value.isdir = item.isdir;
        click_item.value.index = index;
    }

    return { x, y, show, click_item, set_pos, set_show, set_click_item };
});