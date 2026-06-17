<template>
  <div class="about-page">
    <!-- 返回按钮 -->
    <div class="back-btn" @click="$router.back()">
      <svg
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        stroke="#fff"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <path d="M19 12H5M5 12L12 19M5 12L12 5"></path>
      </svg>
    </div>

    <!-- 原版视频容器 -->
    <div class="video-container">
      <video
        class="brand-video"
        src="/static/about.mp4"
        muted
        autoplay
        playsinline
        loop
        preload="auto"
        @canplay="onVideoReady"
      ></video>
    </div>

    <!-- 半透明加载遮罩 -->
    <div v-if="loading" class="loading-mask">
      <div class="loading-spinner"></div>
      <p class="loading-text">{{ $t("about.loading") }}</p>
    </div>

    <!-- 底部留白避开底部导航 -->
    <div class="space-bottom"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, onUnmounted } from "vue";

const loading = ref(true);
let timer: number | null = null;

const onVideoReady = () => {
  loading.value = false;
  if (timer) clearTimeout(timer);
};

// 10秒超时兜底，防止永久转圈
timer = window.setTimeout(() => {
  loading.value = false;
}, 15000);

// 组件销毁清除定时器
onUnmounted(() => {
  if (timer) clearTimeout(timer);
});
</script>

<style scoped>
.about-page {
  width: 100%;
  min-height: 100vh;
  background-color: #000;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 16px;
  box-sizing: border-box;
  position: relative;
}
/* 退出返回按钮样式 */
.back-btn {
  position: fixed;
  top: 24px;
  left: 16px;
  width: 40px;
  height: 40px;
  background: rgba(0, 0, 0, 0.4);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 99;
  transition: transform 0.15s ease;
}
.back-btn:active {
  transform: scale(0.92);
  background: rgba(0, 0, 0, 0.6);
}
.video-container {
  width: 100%;
  max-width: 720px;
  border-radius: 16px;
  overflow: hidden;
  box-shadow: 0 4px 18px rgba(0, 0, 0, 0.3);
}
.brand-video {
  width: 100%;
  height: auto;
  display: block;
}

/* 半透明加载遮罩，不会完全盖住视频 */
.loading-mask {
  position: absolute;
  inset: 0;
  background: rgba(0, 0, 0, 0.65);
  z-index: 2;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 16px;
}
.loading-spinner {
  width: 40px;
  height: 40px;
  border: 4px solid rgba(255, 255, 255, 0.2);
  border-top-color: #0066cc;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}
.loading-text {
  color: #fff;
  font-size: 14px;
  margin: 0;
}
@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.space-bottom {
  width: 100%;
  height: 70px;
}
</style>
