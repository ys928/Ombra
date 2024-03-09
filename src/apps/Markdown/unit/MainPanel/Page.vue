<script setup lang="ts">
import { EditorState, Transaction, Plugin } from 'prosemirror-state';
import { EditorView } from 'prosemirror-view';
import { onMounted } from 'vue'
import { undo, redo, history } from "prosemirror-history"
import { keymap } from "prosemirror-keymap"
import 'prosemirror-view/style/prosemirror.css'
import { schema, defaultMarkdownParser, defaultMarkdownSerializer } from "prosemirror-markdown"
import { baseKeymap, toggleMark, setBlockType } from 'prosemirror-commands';
import { FS, Dialog } from '~/api'

const props = defineProps(['md_file']);

// 创建一个插件来监视用户的输入行为
const monitor_changes_plugin = new Plugin({
    // 定义插件的state
    state: {
        init() {
        },
        apply(tr, _prevState) {
            // 应用变化时，检查是否有未保存的更改
            if (tr.docChanged) {
                props.md_file.save = false;
            }
        }
    }
});

let state = EditorState.create({
    schema,
    //@ts-ignore
    doc: defaultMarkdownParser.parse(props.md_file.content),
    plugins: [
        history(),
        keymap({ "Mod-z": undo, "Mod-y": redo }),
        keymap(baseKeymap),
        keymap({
            "Ctrl-b": toggleMark(schema.marks.strong),
            "Ctrl-i": toggleMark(schema.marks.em),
            "Ctrl-Shift-~": toggleMark(schema.marks.code),
            "Ctrl-1": setBlockType(schema.nodes.heading, { level: '1' }),
            "Ctrl-2": setBlockType(schema.nodes.heading, { level: '2' }),
            "Ctrl-3": setBlockType(schema.nodes.heading, { level: '3' }),
            "Ctrl-4": setBlockType(schema.nodes.heading, { level: '4' }),
            "Ctrl-5": setBlockType(schema.nodes.heading, { level: '5' }),
            "Ctrl-6": setBlockType(schema.nodes.heading, { level: '6' }),
            "Ctrl-p": setBlockType(schema.nodes.paragraph),
            "Ctrl-s": save_file,
        }),
        monitor_changes_plugin
    ]
})
onMounted(() => {
    let mdedit = document.querySelector('.MdEdit');
    const view = new EditorView(mdedit, {
        state
    });
    view.focus();
    // document.addEventListener('keydown', (e) => {
    //     console.log(e);
    // })
})


function save_file(state: EditorState, _dispatch: ((tr: Transaction) => void) | undefined) {
    let content = defaultMarkdownSerializer.serialize(state.doc);
    if (props.md_file.path.length > 0) {
        FS.write_text(props.md_file.path, content);
        props.md_file.save = true;
        return true;
    }

    Dialog.save({
        title: '保存文件',
        filters: [{ name: 'markdown', extensions: ['md'] }]
    }).then((path) => {
        if (path == null) return;
        FS.write_text(path, content);
        props.md_file.path = path;
        props.md_file.save = true;
    })
    return true
}

</script>

<template>
    <div class="MdEdit"> </div>
</template>

<style lang="less">
.MdEdit {
    width: 100%;
    height: 100%;
    overflow: hidden;
    padding: 20px 0;

    .ProseMirror {
        width: 100%;
        height: 100%;
        outline: none;
        border: none;
        color: #bbb;

        padding: 10px 20px;
        overflow-x: hidden;
        overflow-y: auto;

        &::-webkit-scrollbar {
            width: 6px;
        }

        &::-webkit-scrollbar-thumb {
            background-color: #666;
            border-radius: 3px;
        }

        &::-webkit-scrollbar-track {
            background-color: transparent;
        }

        h1 {
            margin: 25px 0;
        }

        h2 {
            margin: 20px 0;
        }

        h3 {
            margin: 15px 0;
        }

        h4 {
            margin: 10px 0;
        }

        h5 {
            margin: 5px 0;
        }

        h6 {
            margin: 0;
        }

        p {
            margin: 10px 0;
            font-size: 14px;

            img {
                max-width: 90%;
            }

            &>code {
                color: #419af7;
                background-color: #2F2F2F;
                white-space: nowrap;
                border-radius: 4px;
                font-size: 14px;
                padding: 0.15em 0.5em;
                font-weight: 700;
            }
        }

        ol,
        ul {
            margin: 10px 20px;

            li {
                margin: 0.5em 1rem;
                font-size: 15px;

                p {
                    &>code {
                        color: #419af7;
                        background-color: #2F2F2F;
                        white-space: nowrap;
                        border-radius: 4px;
                        font-size: 14px;
                        padding: 0.15em 0.5em;
                        font-weight: 700;
                    }
                }
            }
        }

        pre {
            background-color: #2D2D2D;
            padding: 20px;
        }
    }
}
</style>