<template>
    <div class="AppMenu" @click="props.main_input.focus()">
        <div class="SearchResult" v-if="search_result_list.length != 0">
            <div class="title" @click="fun_expand">
                <span>查找工具</span>
                <span>
                    <span>{{ search_result_is_expand ? '折叠' : '展开' }}</span>
                    <span>({{ search_count }})</span>
                </span>
            </div>
            <div class="items">
                <div v-for="(item, index) in search_result_list" class="item" :key="item.name"
                    :class="{ 'active': index == props.cur_focus_app }" @click="fun_open_app(item, true)">
                    <img :src="item.icon" draggable="false">
                    <div class="name" v-html="get_show_name(item.name)"></div>
                </div>
            </div>
        </div>
        <div class="RecommandResult" v-if="recommend_list.length != 0">
            <div class="title">
                <span>推荐工具</span>
            </div>
            <div class="items">
                <div v-for="(item, index) in recommend_list" class="item" :key="item.name"
                    :class="{ 'active': recommand_item_is_active(index) }" @click="fun_open_app(item, false)">
                    <img :src="item.icon" draggable="false">
                    <div class="name"> <span>{{ item.name }}</span> </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">

import { nextTick, onMounted, reactive, ref, watch } from 'vue';
import PinyinMatch from 'pinyin-match';
import { get_app_list, write_config_item, type AppInfo, read_config_item, get_span } from '~/global'
import { om_set_appid, om_set_features, om_set_plugin_index, om_set_text, path_judge, win_set_size, win_to_app } from '~/ombra';
const props = defineProps(['main_input', 'search_content', 'cur_focus_app']);
const emit = defineEmits(['update:cur_focus_app']);

const app_list = get_app_list(); //所有应用列表

const search_result_list = reactive([]) as Array<AppInfo>; //每次的搜索结果

const recommend_list = reactive([]) as Array<AppInfo>; //最多8个推荐应用

const search_count = ref(0); //搜索结果的数量

let features_list = [] as Array<string>; //为每次搜索内容所匹配到的特点
let back_features = [] as Array<string>; //features_list的备份

let search_result_is_expand = ref(false); //记录搜索结果是否为展开状态

let app_setup_content = ''; //点击app时向其中传入的文本内容

onMounted(async () => {
    let appinfo = await read_config_item('appinfo');
    if (appinfo == undefined) {
        appinfo = [];
        for (let i = 0; i < app_list.length; i++) {
            appinfo.push({
                name: app_list[i].name,
                weight: 0,
            });
        }
    } else {
        for (let i = 0; i < app_list.length; i++) {
            let f = true;
            for (let j = 0; j < appinfo.length; j++) {
                if (appinfo[j].name == app_list[i].name) {
                    app_list[i].weight = appinfo[j].weight;
                    f = false;
                    break;
                }
            }
            if (f) {
                app_list[i].weight = 0;
                appinfo.push({
                    name: app_list[i].name,
                    weight: 0
                })
            }
        }
    }
    write_config_item('appinfo', appinfo);
    app_list.sort((a, b) => {
        return a.weight - b.weight;
    });
    search(); //初始化操作
});

//解决本地应用延迟加载未显示的问题
let app_load_timer: string | number | NodeJS.Timeout | undefined;
watch(() => app_list.length, () => {
    if (app_load_timer) clearTimeout(app_load_timer);
    app_load_timer = setTimeout(search, 100);
})

function click_app() {
    if (search_result_list.length == 0) {
        fun_open_app(recommend_list[props.cur_focus_app], false);
    } else {
        if (props.cur_focus_app >= search_result_list.length) {
            fun_open_app(recommend_list[props.cur_focus_app % search_result_list.length], false);
            return;
        }
        fun_open_app(search_result_list[props.cur_focus_app], true);
    }
}
async function move(dire: String) {
    let all_length = search_result_list.length + recommend_list.length;

    //没有任何可用应用，直接返回
    if (all_length == 0) return;

    switch (dire) {
        case 'up':
            {
                //如果两者结果都存在
                if (search_result_list.length && recommend_list.length) {
                    //如果当前在搜索结果内
                    if (props.cur_focus_app < search_result_list.length) {
                        //如果在最前面，则直接跳到最后
                        if (props.cur_focus_app == 0) {
                            emit('update:cur_focus_app', all_length - 1);
                        } else if (props.cur_focus_app < 8) { //如果在第一行，则每次前进一个
                            emit('update:cur_focus_app', props.cur_focus_app - 1);
                        } else { //否则每次前进一行
                            emit('update:cur_focus_app', props.cur_focus_app - 8);
                        }
                    } else {//如果当前在推荐结果内
                        //如果在推荐结果第一行
                        if (props.cur_focus_app < search_result_list.length + 8) {
                            let offset = props.cur_focus_app - search_result_list.length;
                            let pos = (Math.floor(search_result_list.length / 8) - 1) * 8 + offset;
                            emit('update:cur_focus_app', pos);
                        } else { //否则直接每次-8
                            emit('update:cur_focus_app', props.cur_focus_app - 8);
                        }
                    }
                } else {//如果搜索应用、推荐应用只有一个有结果
                    //如果在最前面，则直接跳到最后
                    if (props.cur_focus_app == 0) {
                        emit('update:cur_focus_app', all_length - 1);
                    } else if (props.cur_focus_app < 8) { //如果在第一行，则每次前进一个
                        emit('update:cur_focus_app', props.cur_focus_app - 1);
                    } else { //否则每次前进一行
                        emit('update:cur_focus_app', props.cur_focus_app - 8);
                    }
                }
            }
            break;
        case 'down':
            {
                //如果两者结果都存在
                if (search_result_list.length && recommend_list.length) {
                    //如果当前在搜索结果内
                    if (props.cur_focus_app < search_result_list.length) {
                        let pos = Math.floor((search_result_list.length - 1) / 8) * 8; //最后一行的起始数字(下标)
                        if (props.cur_focus_app >= pos) { //如果在最后一行，则根据偏移值调准到推荐结果
                            let offset = props.cur_focus_app - pos;
                            if (search_result_list.length + offset >= all_length) {
                                emit('update:cur_focus_app', all_length - 1);
                            } else {
                                emit('update:cur_focus_app', search_result_list.length + offset);
                            }
                        } else { //不在最后一行，每次直接-8
                            emit('update:cur_focus_app', all_length + 8);
                        }
                    } else {//如果当前在推荐结果内
                        //如果在推荐结果最后一行
                        if (props.cur_focus_app >= all_length - 8) {
                            //如果在最后一个，则直接跳到第一个
                            if (props.cur_focus_app == all_length - 1) {
                                emit('update:cur_focus_app', 0);
                            } else { //否则每次+1
                                emit('update:cur_focus_app', props.cur_focus_app + 1);
                            }
                        } else { //否则直接每次+8
                            emit('update:cur_focus_app', props.cur_focus_app + 8);
                        }
                    }
                } else {//如果搜索应用、推荐应用只有一个有结果
                    //如果在推荐结果最后一行
                    if (props.cur_focus_app >= all_length - 8) {
                        //如果在最后一个，则直接跳到第一个
                        if (props.cur_focus_app == all_length - 1) {
                            emit('update:cur_focus_app', 0);
                        } else { //否则每次+1
                            emit('update:cur_focus_app', props.cur_focus_app + 1);
                        }
                    } else { //否则直接每次+8
                        emit('update:cur_focus_app', props.cur_focus_app + 8);
                    }
                }
            }
            break;
        case 'left':
            {
                if (props.cur_focus_app > 0) {
                    emit('update:cur_focus_app', props.cur_focus_app - 1);
                } else {
                    emit('update:cur_focus_app', all_length - 1);
                }
            }
            break;
        case 'right':
            {
                if (props.cur_focus_app < all_length - 1) {
                    emit('update:cur_focus_app', props.cur_focus_app + 1);
                } else {
                    emit('update:cur_focus_app', 0);
                }
            }
            break;
    }
    await nextTick();
    let act_app = document.querySelector('.active');
    act_app?.scrollIntoView({
        block: 'nearest',
        inline: 'nearest',
        behavior: 'smooth'
    });
    return false;
}

function fun_expand() {
    if (search_result_is_expand.value) {
        search_result_is_expand.value = false;
    } else {
        search_result_is_expand.value = true;
    }
    search();
    adjust_height();
}

function adjust_height() {
    //主搜索框高度
    let search_box_height = 50;
    //推荐栏高度
    let recommand_height = recommend_list.length == 0 ? 0 : 120;
    //搜索结果高度
    let search_resule_height = 120;
    if (search_result_is_expand.value) {
        search_resule_height += 90 * Math.floor((search_result_list.length - 1) / 8);
    }
    if (search_result_list.length == 0) {
        search_resule_height = 0
    }

    if (search_resule_height + recommand_height > 550) {
        search_resule_height = 430;
    }
    // console.log(search_box_height + search_resule_height + recommand_height);
    win_set_size(search_box_height + search_resule_height + recommand_height);
}

async function fun_open_app(app: AppInfo, sea_of_rec: boolean) {
    for (let i = 0; i < app_list.length; i++) {
        if (app_list[i].name == app.name) {
            app_list[i].weight += 1;
            break;
        }
    }
    let appinfo = [];
    for (let i = 0; i < app_list.length; i++) {
        appinfo.push({
            name: app_list[i].name,
            weight: app_list[i].weight,
        });
    }
    //最多领先后一个程序3次，防止被某个程序始终占据最前面的位置
    appinfo.sort((a, b) => {
        return b.weight - a.weight;
    })
    for (let i = appinfo.length - 1; i > 0; i--) {
        if (appinfo[i - 1].weight - appinfo[i].weight > 3) {
            appinfo[i - 1].weight = appinfo[i].weight + 3;
        }
    }
    write_config_item('appinfo', appinfo);

    if (sea_of_rec) {
        om_set_text('');
        om_set_features([]);
    } else {
        om_set_text(app_setup_content);
        om_set_features(back_features);
    }

    om_set_appid(app.id);
    //如果是插件应用，保存插件代码文件位置
    if (app.plugin_index.length == 0) {
        om_set_plugin_index('');
    } else {
        om_set_plugin_index(app.plugin_index);
    }
    app.setup();
    if (!app.self) return;
    win_to_app(app.id);
}
let old_search_content = "";
//由父组件触发搜索事件
async function search() {
    //先根据权重进行排序
    app_list.sort((a, b) => {
        return b.weight - a.weight;
    })
    //清空
    search_result_list.length = 0;
    recommend_list.length = 0;
    //如果搜索内容变化，则重新折叠面板
    if (old_search_content != props.search_content) {
        old_search_content = props.search_content;
        search_result_is_expand.value = false;
    }
    //没有任何输入的情况下
    if (props.search_content.length == 0) {
        for (let i = 0; i < app_list.length; i++) {
            for (let f of app_list[i].feature) {
                if (features_list.includes(f) && recommend_list.length < 8) {
                    recommend_list.push(app_list[i])
                    // console.log(recommend_list);
                }
            }
            if (app_list[i].only_feature) continue;

            if (search_result_is_expand.value) {
                search_result_list.push(app_list[i]);
            } else if (search_result_list.length < 8) { //在未展开模式下，最多显示8个应用
                search_result_list.push(app_list[i]);
            }
        }
        search_count.value = app_list.length;
        adjust_height();
        features_list.length = 0; //最后清理掉features
        return;
    }
    //有输入的情况下
    app_setup_content = props.search_content;
    //首先匹配特性
    let fe = await match_feature(props.search_content);
    features_list.push(...fe);
    back_features.length = 0;
    back_features.push(...fe);//备份
    //如果是展开模式下
    if (search_result_is_expand.value) {
        for (let i = 0; i < app_list.length; i++) {
            if (test_name_metch(app_list[i].name, props.search_content)) {
                search_result_list.push(app_list[i]);
            }
            //推荐应用最多8个
            if (recommend_list.length >= 8) continue;
            for (let f of features_list) {
                if (app_list[i].feature.includes(f)) {
                    recommend_list.push(app_list[i]);
                    break;
                }
            }
        }
        adjust_height();
        search_count.value = search_result_list.length;
        features_list.length = 0; //最后清理掉features
        return;
    }

    //如果处于未展开状态，那么最多显示8个
    search_count.value = 0;
    for (let i = 0; i < app_list.length; i++) {
        if (app_list[i].only_feature == false) {
            let ret = test_name_metch(app_list[i].name, props.search_content);
            if (ret) {
                search_count.value += 1;
                if (search_result_list.length < 8) {
                    search_result_list.push(app_list[i]);
                }
            }
        }
        //推荐应用最多8个
        if (recommend_list.length >= 8) continue;
        for (let f of features_list) {
            if (app_list[i].feature.includes(f)) {
                recommend_list.push(app_list[i]);
                break;
            }
        }
    }
    adjust_height();
    features_list.length = 0; //最后清理掉features
    return;
}

function test_name_metch(name: string, m: string) {
    //尝试拼音匹配
    let ret = PinyinMatch.match(name, m);
    if (typeof ret != "boolean") {
        return true;
    }

    //尝试首字母匹配

    //不是由英文单词组成的名字，直接返回false
    if (!/^[a-zA-Z\s]+$/.test(name)) {
        return false;
    }
    //要匹配的内容不是英语字母组成，返回false
    if (!/^[a-zA-Z]+$/.test(m)) {
        return false;
    }
    // 将搜索内容拆分为单词
    const words = name.split(/\s+/).filter(word => word.length > 0);
    //获取所有单词首字母组成的序列
    const initials_str = words.map(word => word[0]).join('').toLowerCase();
    if (initials_str.includes(m)) {
        return true;
    }
    return false;
}

//根据程序名称构造html标签
function get_show_name(name: string) {
    if (!test_name_metch(name, props.search_content)) {
        return get_span(name, 'normal');
    }

    //先根据拼音匹配构造
    let ret = PinyinMatch.match(name, props.search_content);
    if (typeof ret != "boolean") {
        let s = get_span(name.substring(0, ret[0]), 'normal');
        s += get_span(name.substring(ret[0], ret[1] + 1), 'match');
        s += get_span(name.substring(ret[1] + 1, name.length), 'normal');
        return s;
    }

    //否则根据首字母匹配进行构造

    // 将搜索内容拆分为单词
    const words = name.split(/\s+/).filter(word => word.length > 0);
    //获取所有单词首字母组成的序列
    const initials_str = words.map(word => word[0]).join('').toLowerCase();
    let pos = initials_str.search(props.search_content.toLowerCase());
    let s = '';
    for (let i = 0; i < words.length; i++) {
        if (i >= pos && i < pos + props.search_content.length) {
            s += get_span(words[i][0], 'match');
            s += get_span(words[i].substring(1) + ' ', 'normal');
        } else {
            s += get_span(words[i] + ' ', 'normal');
        }
    }
    return s;
}

//根据搜索内容返回可能的特性
async function match_feature(cnt: string) {
    let tmp_feature = []
    let re_sep = /^[^\/\\:\*\?"<>|]*$/g;
    if (re_sep.test(cnt)) {
        tmp_feature.push('filename');
    }
    var re = /^(?:http(s)?:\/\/)?[\w.-]+(?:\.[\w\.-]+)+[\w\-\._~:/?#[\]@!\$&'\*\+,;=.]+$/g;
    if (re.test(cnt)) {
        tmp_feature.push('url');
    }

    let r_path = /^[a-zA-Z]:\\([^\/\\:\*\?"<>|]+\\)*[^\/\\:\*\?"<>|]*\\?$/
    if (r_path.test(cnt)) {
        let t = await path_judge(cnt);
        if (t == 'dir') {
            tmp_feature.push('dir_path');
        } else if (t == 'file') {
            tmp_feature.push('file_path');
        }
    }
    return tmp_feature;
}

function recommand_item_is_active(index: number) {
    if (search_result_list.length != 0) {
        return index == props.cur_focus_app - search_result_list.length;
    } else {
        return index == props.cur_focus_app;
    }
}
//由父组件调用，用于初始化feature
function init_feature(feature: string[], data: string) {
    features_list.length = 0;
    features_list.push(...feature);
    back_features.length = 0;
    back_features.push(...feature);
    app_setup_content = data;
    search();
}


defineExpose({
    click_app,
    move,
    search,
    init_feature
});

</script>

<style scoped lang="less">
.AppMenu {
    background-color: #252525;
    display: flex;
    flex-direction: column;

    .SearchResult {
        height: 120px;
        flex-grow: 1;
        display: flex;
        flex-direction: column;

        .title {
            color: #cbcbcc;
            font-weight: bold;
            padding: 0 10px;
            font-size: 15px;
            height: 30px;
            line-height: 30px;
            user-select: none;
            cursor: pointer;

            &:hover {
                background: #3b3c3d;
            }

            display: flex;
            justify-content: space-between;
        }

        .items {
            display: flex;
            flex-wrap: wrap;
            background-color: #252525;
            overflow: auto;
            height: 90px;
            flex-grow: 1;

            &::-webkit-scrollbar {
                width: 5px;
            }

            &::-webkit-scrollbar-thumb {
                background-color: #434343;
                border-radius: 3px;
            }

            &::-webkit-scrollbar-track {
                background-color: #252525;
            }

            .item {
                height: 90px;
                width: 99px;
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                cursor: pointer;
                user-select: none;
                padding: 10px;
                text-align: center;

                &:hover {
                    background-color: #3c3c3e;
                }

                img {
                    width: 30px;
                    height: 30px;
                    border-radius: 5px;
                    user-select: none;
                }

                .name {
                    font-size: 12px;
                    user-select: none;
                    height: 35px;
                    margin: 7px 4px 0;

                    display: -webkit-box;
                    -webkit-box-orient: vertical;
                    -webkit-line-clamp: 2;
                    overflow: hidden;
                    color: #d2d2d2;
                    word-break: break-all;

                    &:deep(.normal) {
                        color: #d2d2d2;
                    }

                    &:deep(.match) {
                        color: #f18325;
                    }
                }
            }

            .active {
                background-color: #565656;
            }
        }
    }

    .RecommandResult {
        height: 120px;

        .title {
            color: #cbcbcc;
            font-weight: bold;
            padding: 0 10px;
            font-size: 15px;
            height: 30px;
            line-height: 30px;
            user-select: none;
        }

        .items {
            display: flex;
            flex-wrap: wrap;
            background-color: #252525;
            overflow: hidden;
            height: 90px;
            flex-grow: 1;

            .item {
                height: 90px;
                width: 99px;
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                cursor: pointer;
                user-select: none;
                padding: 10px;

                &:hover {
                    background-color: #3c3c3e;
                }

                img {
                    width: 30px;
                    height: 30px;
                    border-radius: 5px;
                    user-select: none;
                }

                .name {
                    font-size: 12px;
                    user-select: none;
                    height: 35px;
                    margin: 7px 4px 0;
                    text-align: center;

                    word-break: break-all;
                    text-overflow: ellipsis;
                    display: -webkit-box;
                    -webkit-box-orient: vertical;
                    -webkit-line-clamp: 2;
                    /* 这里是超出几行省略 */
                    overflow: hidden;
                    color: #d2d2d2;
                }
            }

            .active {
                background-color: #565656;
            }
        }
    }

    .plugin {
        height: 200px;
    }
}
</style>