<template>
  <!-- 页面内容渲染位置 -->
  <router-view />
  <!-- 仅非隐藏页面展示底部导航，后续新增页面直接追加判断条件 -->
  <BottomNav
    v-if="
      $route.path !== '/shop/detail' &&
      $route.path !== '/about' &&
      $route.path !== '/zodiac-game'
    "
  />
</template>

<script setup lang="ts">
// 引入底部导航组件
import BottomNav from "@/components/BottomNav.vue";
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}
/* 有导航的页面自动预留底部间距；无导航页面不受此padding影响
body {
  padding-bottom: 60px;
} */
</style>
