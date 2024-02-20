<template>
    <i class="KIcon">
        <svg viewBox="0 0 200 200" version="1.1" :width="prop.w" :height="prop.h" xmlns="http://www.w3.org/2000/svg">
            <g>
                <text font-weight="bold" xml:space="preserve" text-anchor="start" font-size="100" id="svg_2" y="142.33348"
                    x="37.33497" stroke-width="0" stroke="currentColor" fill="currentColor">ab</text>
                <line stroke="currentColor" stroke-width="8" id="svg_3" y2="160" x2="176.33497" y1="160" x1="22.33498"
                    fill="none" />
                <line id="svg_4" y2="163" x2="22.33498" y1="120" x1="22" stroke-width="8" stroke="currentColor"
                    fill="none" />
                <line id="svg_5" y2="164" x2="172.33497" y1="120" x1="172.33497" stroke-width="8" stroke="currentColor"
                    fill="none" />
            </g>

        </svg>
    </i>
</template>

<script setup lang="ts">
const prop = defineProps(['w', 'h']);

</script>

<style scoped lang="less">
.KIcon {
    font-style: normal;
}
</style>