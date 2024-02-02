<script setup lang="ts">
import { Ref, ref, watch } from 'vue';

const props = defineProps(['value', 'placeholder']);
const emits = defineEmits(['update:value', 'change', 'keydown']);

const search_input = ref() as Ref<HTMLElement>;

let is_ompositioning = false;

watch(() => props.value, () => {
    search_input.value.textContent = props.value;
});

function fun_ompositionstart() {
    is_ompositioning = true;
}
function fun_ompositionend() {
    is_ompositioning = false;
    emits('update:value', search_input.value.textContent);
    emits('change')
}
async function fun_input() {
    if (is_ompositioning) return;
    emits('update:value', search_input.value.textContent);
    emits('change');
}

function fun_keydown(e: KeyboardEvent) {
    emits('keydown', e);
}

function focus() {
    if (search_input.value) {
        search_input.value.focus();
    }
}
function selected() {
    const selection = window.getSelection() as Selection;
    const range = document.createRange();
    range.selectNodeContents(search_input.value);
    selection.removeAllRanges();
    selection.addRange(range);
    search_input.value.focus();
}
defineExpose({
    focus,
    selected
})

</script>

<template>
    <div class="Search">
        <div ref="search_input" contenteditable @input="fun_input" @compositionstart="fun_ompositionstart"
            @compositionend="fun_ompositionend" @keydown="fun_keydown($event)">
        </div>
        <span class="placeholder" v-if="props.value.length == 0">{{ props.placeholder }}</span>
    </div>
</template>

<style scoped lang="less">
.Search {
    position: relative;

    div {
        min-width: 20px;
        max-width: 730px;
        font-family: 'Arial', sans-serif;
        height: 30px;
        margin: 10px 0;
        line-height: 30px;
        border-radius: 5px;
        font-size: 25px;
        outline: none;
        border: none;
        background-color: #252525;
        color: #f1f2f3;
        padding: 0 10px;
        white-space: nowrap;
        overflow-x: auto;
        overflow-y: hidden;

        &::selection {
            background-color: #263C58;
        }

        &::-webkit-scrollbar {
            height: 0;
        }
    }

    .placeholder {
        position: absolute;
        left: 10px;
        top: 10px;
        white-space: nowrap;
        color: #949494;
        font-size: 25px;
    }
}
</style>