<template>
  <div class="bottom-nav">
    <!-- 优社区 -->
    <router-link
      to="/"
      class="nav-item"
      :class="{ active: route.path === '/' }"
    >
      <i class="fa-solid fa-users"></i>
      <span>{{ $t("tab.community") }}</span>
    </router-link>
    <!-- 优商城 -->
    <router-link
      to="/shop"
      class="nav-item"
      :class="{ active: route.path === '/shop' }"
    >
      <i class="fa-solid fa-bag-shopping"></i>
      <span>{{ $t("tab.mall") }}</span>
    </router-link>
    <!-- 智能合约 -->
    <router-link
      to="/contract"
      class="nav-item"
      :class="{ active: route.path === '/contract' }"
    >
      <i class="fa-solid fa-file-lines"></i>
      <span>{{ $t("tab.contract") }}</span>
    </router-link>
    <!-- 我的 -->
    <router-link
      :to="myPath"
      class="nav-item"
      :class="{ active: route.path.startsWith('/my') }"
    >
      <i class="fa-solid fa-user"></i>
      <span>{{ $t("tab.mine") }}</span>
    </router-link>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { useRoute } from "vue-router";
// 仅引入路由，无需额外i18n导入，main.ts已全局注入$t
const route = useRoute();
const myPath = ref("/my");

function setMyRoute() {
  const userStr = localStorage.getItem("pi_user");
  if (userStr) {
    try {
      const user = JSON.parse(userStr);
      if (user.uid) {
        myPath.value = `/my?uid=${user.uid}`;
      }
    } catch {
      myPath.value = "/my";
    }
  }
}

onMounted(() => setMyRoute());
watch(route, () => setMyRoute());
</script>

<style scoped>
.bottom-nav {
  position: fixed;
  bottom: 0;
  left: 0;
  width: 100%;
  height: 60px;
  background: #fff;
  display: flex;
  border-top: 1px solid #eee;
  z-index: 99;
}
.nav-item {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  color: #999;
  text-decoration: none;
}
.nav-item.active {
  color: #0066cc;
}
.fa-solid {
  font-size: 20px;
  margin-bottom: 4px;
}
.nav-item span {
  font-size: 11px;
}
</style>
