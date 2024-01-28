<script setup lang="ts">
import { onMounted, reactive } from 'vue';
import App, { type WebUrlApp } from '~/api/app';
import Config from '~/api/config';
import { KSwitch } from '~/kui'
import { useAppListStore } from '~/stores/appList';

const web_apps = reactive([]) as Array<WebUrlApp>;

const applistStore = useAppListStore();

onMounted(async () => {
    web_apps.push(... (await App.get_web()));
});

async function fun_change(item: WebUrlApp) {
    Config.write_web_apps(web_apps);
    if (item.on) {
        applistStore.add_web(item);
    } else {
        applistStore.remove(item.id);
    }
}

</script>

<template>
    <div class="Web">
        <div class="recommand">
            <div class="url_item" v-for="item in web_apps">
                <div class="web_info">
                    <img class="logo" :src="item.icon" alt="网站图标">
                    <div class="info">
                        <div class="name">{{ item.name }}</div>
                        <div class="url">{{ item.url }} </div>
                    </div>
                </div>
                <div class="opt">
                    <KSwitch v-model:value="item.on" @click="fun_change(item)"></KSwitch>
                </div>
            </div>
        </div>
        <div>

        </div>
    </div>
</template>

<style scoped lang="less">
.Web {

    width: 100%;
    height: 100%;

    .recommand {
        width: 100%;
        display: flex;
        flex-direction: column;

        .url_item {
            display: flex;
            justify-content: space-between;
            padding: 10px 20px;

            .web_info {
                display: flex;
                justify-content: left;

                .logo {
                    width: 30px;
                    height: 30px;
                }

                .info {
                    display: flex;
                    flex-direction: column;
                    color: #BDBDBE;
                    margin-left: 10px;

                    .name {
                        font-size: 14px;
                    }

                    .url {
                        font-size: 12px;
                    }

                }

            }

            .opt {
                display: flex;
                align-items: center;
            }
        }

    }
}
</style>