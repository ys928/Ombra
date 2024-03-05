<script setup lang="ts">
import PopMenu from './unit/PopMenu.vue'
import Search from './unit/Search.vue';
import Statusbar from './unit/Statusbar.vue';
import Result from './unit/Result.vue';

</script>

<template>
    <div class="FileSearch">
        <Search></Search>
        <Result></Result>
        <Statusbar></Statusbar>
        <PopMenu></PopMenu>
    </div>
</template>

<style scoped lang="less">
.FileSearch {
    height: 100%;
    width: 100%;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    background-color: #191919;
    position: relative;
}
</style>