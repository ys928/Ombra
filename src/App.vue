<template>
  <div class="RootApp" @keydown="fun_keydown($event)" @contextmenu="fun_contextmenu($event)">
    <RouterView></RouterView>
  </div>
</template>
<script setup lang="ts">
import { onMounted } from 'vue';
import { Window } from '~/api'
import { load_apps } from './apps';

onMounted(() => {
  if (Window.is_main()) {
    Window.focus();
    load_apps();
  }
  document.oncontextmenu = function (e) {
    e.preventDefault();
  };
});

function fun_keydown(e: KeyboardEvent) {
  if (e.key == 'F5') e.preventDefault();
  if (e.key == 'r' && e.ctrlKey) {
    e.preventDefault();
    e.stopPropagation();
  }
  if (e.key == 'R' && e.shiftKey && e.ctrlKey) {
    e.preventDefault();
    e.stopPropagation();
  }
}

function fun_contextmenu(e: MouseEvent) {
  e.preventDefault();
}

</script>

<style scoped lang="less">
.RootApp {
  width: 100vw;
  height: 100vh;
  background-color: #252525;
}
</style>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}
</style>