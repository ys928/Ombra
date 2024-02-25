
<script setup lang="ts">
import { EditorState, TextSelection, Transaction } from 'prosemirror-state';
import { EditorView } from 'prosemirror-view';
import { onMounted } from 'vue'
import { undo, redo, history } from "prosemirror-history"
import { keymap } from "prosemirror-keymap"
import 'prosemirror-view/style/prosemirror.css'
import { schema } from "prosemirror-markdown"
import { baseKeymap } from 'prosemirror-commands';

let state = EditorState.create({
    schema, plugins: [
        history(),
        keymap({ "Mod-z": undo, "Mod-y": redo }),
        keymap(baseKeymap),
        keymap({
            "Ctrl-b": blod_font,
            "Ctrl-i": italic_font
        })
    ]
})
onMounted(() => {
    let mdedit = document.querySelector('.MdEdit');
    new EditorView(mdedit, {
        state
    });

})

function blod_font(state: EditorState, dispatch: ((tr: Transaction) => void) | undefined) {
    if (!dispatch) return false;
    let { empty, ranges } = state.selection as TextSelection
    let { $from, $to } = ranges[0];
    if (empty) return false
    let strong = state.schema.marks.strong;
    let isActive = true;
    state.tr.doc.nodesBetween($from.pos, $to.pos, (node) => {
        if (!isActive) return false;
        if (node.isInline) {
            const mark = strong.isInSet(node.marks)
            if (!mark) {
                isActive = false;
            }
        }
    })
    if (isActive) {
        let tr = state.tr.removeMark($from.pos, $to.pos, strong.create());
        dispatch(tr);
    } else {
        let tr = state.tr.addMark($from.pos, $to.pos, strong.create());
        dispatch(tr);
    }
    return true
}

function italic_font(state: EditorState, dispatch: ((tr: Transaction) => void) | undefined) {
    if (!dispatch) return false;
    let { empty, ranges } = state.selection as TextSelection
    let { $from, $to } = ranges[0];
    if (empty) return false
    let em = state.schema.marks.em;
    let isActive = true;
    state.tr.doc.nodesBetween($from.pos, $to.pos, (node) => {
        if (!isActive) return false;
        if (node.isInline) {
            const mark = em.isInSet(node.marks)
            if (!mark) {
                isActive = false;
            }
        }
    })
    if (isActive) {
        let tr = state.tr.removeMark($from.pos, $to.pos, em.create());
        dispatch(tr);
    } else {
        let tr = state.tr.addMark($from.pos, $to.pos, em.create());
        dispatch(tr);
    }
    return true
}

</script>
<template>
    <div class="MdEdit"> </div>
</template>
  
<style>
.MdEdit {
    width: 100%;
    height: 100%;
}

.ProseMirror {
    width: 100vw;
    height: 100vh;
    outline: none;
    border: none;
}
</style>
  