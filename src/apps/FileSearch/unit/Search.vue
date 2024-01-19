<template>
    <div class="Search">
        <input v-model="search_content" ref="search_input" @input="fun_input" @compositionstart="fun_compositionstart"
            @compositionend="fun_compositionend" @keydown="fun_keydown($event)">
        <span class="option">
            <KIWholeWord title="精确匹配" :w="18" :h="18" :class="{ active: use_whole_word }" @click="fun_switch_whole_word">
            </KIWholeWord>
        </span>
    </div>
</template>

<script setup lang="ts">
import { Ref, onMounted, ref } from 'vue';
import { KIWholeWord } from '~/kui';
import { om_get_text } from '~/ombra';

const emits = defineEmits(['fun_search', 'fun_exit']);

const use_whole_word = ref(false) as Ref<Boolean>;

const search_content = ref('');
const mode = ref('normal');

let is_inputing = false;
const search_input = ref() as Ref<HTMLElement>;
onMounted(() => {
    search_input.value.focus();
    document.addEventListener('click', () => {
        if (search_input.value && document.activeElement != search_input.value) {
            search_input.value.focus();
        }
    });
    search_content.value = om_get_text();
    let cnt = get_name_ext();
    emits('fun_search', cnt.name, cnt.ext, mode.value, 0);
});

function fun_keydown(e: KeyboardEvent) {
    if (e.key == 'Escape') {
        if (search_content.value.length == 0) {
            emits('fun_exit');
        } else {
            search_content.value = "";
            let cnt = get_name_ext();
            emits('fun_search', cnt.name, cnt.ext, mode.value, 0);
        }
    }
}

function fun_switch_whole_word() {
    use_whole_word.value = !use_whole_word.value;
    if (use_whole_word.value) {
        mode.value = 'exact';
    } else {
        mode.value = 'normal';
    }
    let cnt = get_name_ext();
    emits('fun_search', cnt.name, cnt.ext, mode.value, 0);
}

function fun_compositionstart() {
    is_inputing = true;
}
function fun_compositionend() {
    is_inputing = false;
    let cnt = get_name_ext();
    emits('fun_search', cnt.name, cnt.ext, mode.value, 0);
}

function fun_input() {
    if (is_inputing) return;
    let cnt = get_name_ext();
    emits('fun_search', cnt.name, cnt.ext, mode.value, 0);
}

function get_name_ext() {
    let pos = search_content.value.lastIndexOf('.');
    let name = search_content.value;
    let ext = "";
    if (pos != -1) {
        name = search_content.value.substring(0, pos);
        ext = search_content.value.substring(pos + 1);
    }
    return { name, ext };
}

</script>

<style scoped lang="less">
.Search {
    position: relative;

    input {
        width: 100%;
        flex-grow: 1;
        height: 25px;
        outline: none;
        background-color: #242424;
        border: none;
        color: #eee;
        padding-left: 20px;
        padding-right: 100px;
        user-select: none;
    }

    .option {
        position: absolute;
        right: 5px;
        top: 1px;
        display: flex;
    }

    .KIcon {
        width: 22px;
        height: 22px;
        color: #ccc;
        cursor: pointer;
        display: flex;
        align-items: center;
        border-radius: 5px;
        font-style: normal;
        display: flex;
        justify-content: center;
        align-items: center;

        &:hover {
            background-color: #454747;
        }
    }

    .active {
        background-color: #2a5e88;
        border: 1px solid #2488db;

        &:hover {
            background-color: #2a5e88;
        }
    }
}
</style>