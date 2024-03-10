<script setup lang="ts">
import { onMounted } from 'vue';
import { Window } from '~/api'
import { load_apps } from '~/apps';

onMounted(() => {
  if (Window.is_main()) {
    Window.focus();
    load_apps();
  }

  document.oncontextmenu = function (e) {
    e.preventDefault();
  };

  document.addEventListener('keydown', fun_keydown)
});

function fun_keydown(e: KeyboardEvent) {
  if (e.key == 'F5') e.preventDefault();
  if (e.key == 'r' && e.ctrlKey) {
    e.preventDefault();
  }
  if (e.key == 'R' && e.shiftKey && e.ctrlKey) {
    e.preventDefault();
  }
  if (e.key == "g" && e.ctrlKey) {
    e.preventDefault();
  }
  if (e.key == "f" && e.ctrlKey) {
    e.preventDefault();
  }
  if (e.key == "p" && e.ctrlKey) {
    e.preventDefault();
  }
  if (e.key == "j" && e.ctrlKey) {
    e.preventDefault();
  }
}

</script>

<template>
  <div class="RootApp">
    <RouterView></RouterView>
  </div>
</template>

<style scoped lang="less">
.RootApp {
  width: 100vw;
  height: 100vh;
  overflow: hidden;
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