<script setup lang="ts">
import AppTitlebar from '~/components/AppTitlebar.vue'
import App from './App'
import { recommand_urls } from './App'
import { ref } from 'vue';

const search_content = ref('');


</script>

<template>
    <div class="WebQuickOpen">
        <AppTitlebar :id="App.id" :text="search_content"></AppTitlebar>
        <div class="Content">
            <div class="cate">
                <div class="item">通用</div>
                <div class="item">自定义</div>
            </div>
            <div class="urls">
                <div class="url" v-for="item in recommand_urls">
                    <div> <img :src="item.icon" alt=""> {{ item.name }}</div>
                    <div>{{ item.url }}</div>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped lang="less">
.WebQuickOpen {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;

    .Content {
        height: 100px;
        width: 100%;
        flex-grow: 1;
        display: flex;
        color: #fff;
        user-select: none;

        .cate {
            width: 200px;
            display: flex;
            flex-direction: column;
            justify-content: flex-start;
            font-size: 14px;
            border-right: 1px solid #464646;

            .item {
                width: 100%;
                padding: 10px 20px;
                cursor: pointer;

                &:hover {
                    background-color: #2E2F30;
                }
            }
        }

        .urls {
            flex-grow: 1;

        }
    }
}
</style>