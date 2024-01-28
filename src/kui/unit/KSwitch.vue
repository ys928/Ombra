<template>
    <div class="KSwitch" :class="{ 'on': prop.value }" @click="toggle">
        <span class="icon" :style="{ left: prop.value ? '20px' : '0px' }"></span>
    </div>
</template>
   
<script setup lang="ts">

const prop = defineProps(['value']);
const emit = defineEmits(['update:value']);

function toggle() {
    emit('update:value', !prop.value);
}
</script>
   
<style scoped lang="less">
.KSwitch {
    background-color: #6D6E6F;
    width: 40px;
    height: 14px;
    border-radius: 7px;
    cursor: pointer;
    position: relative;
    transition: left 0.5s;

    .icon {
        position: absolute;
        top: -3px;
        width: 20px;
        height: 20px;
        border-radius: 10px;
        display: inline-block;
        background-color: #fff;
        transition: left 0.5s, background-color 0.5s;
    }
}

.on {
    background-color: #607D95;

    .icon {
        background-color: #90CAF9;
    }
}
</style>