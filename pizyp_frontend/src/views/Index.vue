<template>
  <div class="home-page">
    <!-- 右上角欢迎区域 + 语言切换下拉 -->
    <h5 id="welcome-text">
      <span v-if="!userInfo"
        >{{ $t("welcome.guestTag") }}{{ $t("welcome.loginTip") }}
        <span @click="piLogin" class="login-btn">{{
          $t("welcome.loginBtn")
        }}</span>
      </span>
      <span v-else
        >🎉{{ userInfo.username }}🎉
        <span @click="createPiPayment" class="tip-btn">{{
          $t("welcome.tipBtn")
        }}</span>
      </span>
      <!-- 语言切换下拉框 -->
      <select
        v-model="currentLang"
        @change="handleChangeLang"
        class="lang-select"
      >
        <option value="zh-CN">中文</option>
        <option value="en-US">English</option>
        <option value="es-ES">Español</option>
        <option value="fr-FR">Français</option>
        <option value="ja-JP">日本語</option>
        <option value="ko-KR">한국어</option>
      </select>
    </h5>

    <!-- 轮播图 -->
    <div class="banner">
      <div class="banner-wrap">
        <img src="/static/banner1.jpg" alt="轮播1" />
        <img src="/static/banner2.jpg" alt="轮播2" />
        <img src="/static/banner3.jpg" alt="轮播3" />
      </div>
    </div>

    <!-- 朋友圈动态列表 -->
    <div class="post-list">
      <!-- 加载中 -->
      <div v-if="loading" class="loading-box">
        <div class="loader"></div>
        <div>{{ $t("loading.community") }}</div>
      </div>

      <!-- 加载失败 -->
      <div v-else-if="errorMsg" class="error-tip">{{ errorMsg }}</div>

      <!-- 列表内容 -->
      <div v-else>
        <div class="post-item" v-for="item in listData" :key="item.id">
          <img v-if="item.faceSrc" class="post-avatar" :src="item.faceSrc" />
          <div class="post-content">
            <span class="post-nick">{{ item.nick_name || "" }}</span>
            <span class="post-time">{{ item.time || "" }}</span>
            <div class="post-text">{{ item.text || "" }}</div>

            <img v-if="item.imgSrc" class="post-img" :src="item.imgSrc" />

            <video v-if="item.videoSrc" class="post-video" controls>
              <source :src="item.videoSrc" type="video/mp4" />
              {{ $t("videoTip") }}
            </video>

            <div class="post-actions">
              <span>👍 {{ $t("action.like") }} {{ item.prize_num ?? 0 }}</span>
              <span
                >💬 {{ $t("action.comment") }}
                {{ item.comments_num ?? 0 }}</span
              >
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useI18n } from "@/i18n";

// i18n 语言切换逻辑
const { t, locale } = useI18n();
const currentLang = ref(locale.value);
const handleChangeLang = () => {
  locale.value = currentLang.value;
  localStorage.setItem("site_lang", currentLang.value);
};

// 状态定义
const loading = ref(true);
const errorMsg = ref("");
const listData = ref<any[]>([]);
const userInfo = ref<{
  username: string;
  uid: string;
  accessToken: string;
} | null>(null);

// 统一请求封装
const httpPost = async (url: string, data?: any, headers?: any) => {
  const timeoutMs = 20000;
  const abortCtrl = new AbortController();
  const timer = setTimeout(() => abortCtrl.abort(), timeoutMs);

  try {
    const h: any = { "Content-Type": "application/json" };
    if (headers) Object.assign(h, headers);

    const res = await fetch(url, {
      method: "POST",
      headers: h,
      body: data ? JSON.stringify(data) : null,
      signal: abortCtrl.signal,
    });

    clearTimeout(timer);
    const json = await res.json();
    if (json.code !== 0) throw new Error(json.msg || t("error.loadFail"));
    return json;
  } catch (e: any) {
    clearTimeout(timer);
    let msg = e.message || t("error.loadFail");
    if (e.name === "AbortError") msg = "请求超时，请检查网络";
    throw new Error(msg);
  }
};

// 加载首页轻量化动态列表
const loadHomeData = async () => {
  try {
    loading.value = true;
    errorMsg.value = "";
    const json = await httpPost("/api/v1/action/list");
    console.log("【列表接口完整返回】", json);
    // 后端外层是data数组，修复之前json.list错误
    const rawList = json.data || [];
    listData.value = rawList.map((item: any) => ({
      ...item,
      mediaLoaded: false,
      faceSrc: "",
      imgSrc: "",
      videoSrc: "",
    }));
    // 列表渲染完成立刻批量加载所有媒体资源
    listData.value.forEach((item) => loadItemMedia(item));
  } catch (e: any) {
    errorMsg.value = t("error.loadFail");
    console.error("加载首页动态失败", e);
  } finally {
    loading.value = false;
  }
};

// 加载单条媒体Base64
const loadItemMedia = async (item: any) => {
  if (item.mediaLoaded) return;
  console.log("开始请求媒体ID:", item.id);
  try {
    const res = await httpPost("/api/v1/action/media", { id: item.id });
    console.log("【媒体接口返回data】", res.data);
    const mediaData = res.data;
    // 拼接dataURL
    item.faceSrc = `data:image/jpeg;base64,${mediaData.face_base64}`;
    if (mediaData.img_base64) {
      item.imgSrc = `data:image/jpeg;base64,${mediaData.img_base64}`;
    }
    if (mediaData.video_base64) {
      item.videoSrc = `data:video/mp4;base64,${mediaData.video_base64}`;
    }
    item.mediaLoaded = true;
    console.log("赋值头像src前缀:", item.faceSrc.substring(0, 80));
  } catch (err) {
    console.error("媒体加载失败ID=" + item.id, err);
  }
};

// Pi 登录函数（alert替换多语言）
const piLogin = async () => {
  let waitCount = 0;
  while (typeof Pi === "undefined" && waitCount < 20) {
    await new Promise((resolve) => setTimeout(resolve, 100));
    waitCount++;
  }

  if (typeof Pi === "undefined") {
    alert(t("loginAlert.piBrowser"));
    return;
  }

  try {
    const auth = await Pi.authenticate(["payments", "username"]);
    const user = {
      username: auth.user.username,
      uid: auth.user.uid,
      accessToken: auth.accessToken,
    };
    localStorage.setItem("pi_user", JSON.stringify(user));
    userInfo.value = user;
    alert(t("loginAlert.success"));
    window.location.reload();
  } catch (err: any) {
    alert(t("loginAlert.fail") + err.message);
    console.error(err);
  }
};

// Pi 支付打赏（alert全部替换多语言）
const createPiPayment = async () => {
  if (!userInfo.value) {
    alert(t("loginAlert.needLogin"));
    return;
  }
  let paying = false;
  if (paying) return;
  paying = true;
  const orderId = "test_" + Date.now();
  try {
    await Pi.authenticate(["payments", "username"]);
    await Pi.createPayment(
      {
        amount: 0.01,
        memo: "测试打赏",
        metadata: { type: "tip", order_id: orderId },
      },
      {
        onReadyForServerApproval: async (paymentId: string) => {
          try {
            const res = await fetch(
              `/api/v1/pi/payments/${paymentId}/approve`,
              {
                method: "POST",
                headers: {
                  Authorization: "Bearer " + userInfo.value!.accessToken,
                  "Content-Type": "application/json",
                },
                body: JSON.stringify({ order_id: orderId }),
              },
            );
            if (!res.ok) alert(t("payAlert.approveFail") + res.status);
          } catch (e: any) {
            alert(t("payAlert.approveErr") + e.message);
          }
        },
        onReadyForServerCompletion: async (paymentId: string, txid: string) => {
          try {
            const res = await fetch(
              `/api/v1/pi/payments/${paymentId}/complete`,
              {
                method: "POST",
                headers: {
                  Authorization: "Bearer " + userInfo.value!.accessToken,
                  "Content-Type": "application/json",
                },
                body: JSON.stringify({ order_id: orderId, txid: txid }),
              },
            );
            if (!res.ok) alert(t("payAlert.completeFail") + res.status);
            else alert(t("payAlert.paySuccess"));
          } catch (e: any) {
            alert(t("payAlert.completeErr") + e.message);
          }
        },
        onCancel: () => {
          alert(t("payAlert.cancelPay"));
          paying = false;
        },
        onError: (e: any) => {
          alert(t("payAlert.payErr") + e.message);
          paying = false;
        },
      },
    );
  } catch (err: any) {
    alert(t("payAlert.payFail") + err.message);
  } finally {
    paying = false;
  }
};

// 读取本地登录缓存
const loadUserInfo = () => {
  const userStr = localStorage.getItem("pi_user");
  if (userStr) {
    try {
      userInfo.value = JSON.parse(userStr);
    } catch {
      localStorage.removeItem("pi_user");
    }
  }
};

// 页面挂载初始化
onMounted(() => {
  loadUserInfo();
  loadHomeData();
});
</script>

<style scoped>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}
body {
  font-family: Arial, sans-serif;
  padding-bottom: 70px;
}
h5 {
  color: #333;
  margin: 5px 10px 5px;
  font-size: 14px;
  text-align: right;
  white-space: nowrap;
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 12px;
}
.login-btn,
.tip-btn {
  margin-left: 15px;
  color: #0066cc;
  cursor: pointer;
}
/* 语言下拉样式，匹配项目紫黄品牌色 */
.lang-select {
  padding: 2px 6px;
  background: #621fa3;
  color: #ffd700;
  border: none;
  border-radius: 4px;
  font-size: 13px;
}
/* 轮播 */
.banner {
  width: calc(100% - 20px);
  height: 160px;
  overflow: hidden;
  position: relative;
  margin: 0 10px 15px;
}
.banner-wrap {
  display: flex;
  width: 300%;
  height: 100%;
  animation: bannerSlide 12s infinite;
}
.banner-wrap img {
  width: 33.333%;
  height: 100%;
  object-fit: cover;
  flex-shrink: 0;
}
@keyframes bannerSlide {
  0%,
  30% {
    transform: translateX(0);
  }
  33%,
  63% {
    transform: translateX(-33.333%);
  }
  66%,
  96% {
    transform: translateX(-66.666%);
  }
  100% {
    transform: translateX(0);
  }
}
/* 列表 */
.post-list {
  padding: 0 10px;
}
.post-item {
  background: #fff;
  border-bottom: 1px solid #f0f0f0;
  padding: 12px 0;
  display: flex;
  gap: 10px;
}
.post-avatar {
  width: 48px;
  height: 48px;
  border-radius: 50%;
  object-fit: cover;
  flex-shrink: 0;
}
.post-content {
  flex: 1;
}
.post-nick {
  font-size: 15px;
  font-weight: bold;
  color: #333;
}
.post-time {
  font-size: 12px;
  color: #999;
  margin-left: 8px;
}
.post-text {
  font-size: 14px;
  color: #333;
  line-height: 1.5;
  margin: 6px 0;
}
.post-img {
  max-width: 180px;
  border-radius: 6px;
  margin: 4px 0;
}
.post-video {
  max-width: 220px;
  border-radius: 6px;
  margin: 4px 0;
}
.post-actions {
  font-size: 12px;
  color: #999;
  margin-top: 6px;
  display: flex;
  gap: 16px;
}
/* 加载动画 */
.loading-box {
  text-align: center;
  padding: 40px 0;
  color: #666;
}
.loader {
  width: 40px;
  height: 40px;
  border: 3px solid #eee;
  border-top: 3px solid #666;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 10px;
}
@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}
.error-tip {
  text-align: center;
  padding: 40px 0;
  color: #999;
}
</style>
