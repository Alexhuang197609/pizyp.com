<template>
  <div class="page-my">
    <!-- 顶部标题栏：纯白背景通栏，和合约/商城页面完全统一 -->
    <div class="header-title">
      <i class="fa fa-user-circle-o title-icon"></i>
      <h2 class="page-title">{{ $t("my.pageTitle") }}</h2>
    </div>

    <div class="page-content">
      <!-- 登录后卡片内容 -->
      <div v-if="userInfo">
        <!-- 基础信息卡片 -->
        <div class="info-card">
          <div class="info-row">
            <label>{{ $t("my.uidLabel") }}</label>
            <span>{{ userInfo.uid }}</span>
          </div>
          <div class="info-row">
            <label>{{ $t("my.piNameLabel") }}</label>
            <span>{{ userInfo.username }}</span>
          </div>
          <div class="info-row">
            <label>{{ $t("my.walletLabel") }}</label>
            <span>{{ userData.wallet_address || "" }}</span>
          </div>
        </div>

        <!-- 微信小程序同步模块 -->
        <div class="mini-sync-card">
          <div class="sync-row">
            <label>{{ $t("my.miniNick") }}</label>
            <span>{{ userData.nickname || $t("my.noSync") }}</span>
          </div>
          <div class="sync-row">
            <label>{{ $t("my.upiNum") }}</label>
            <span>{{ userData.upi_num || $t("my.noSync") }}</span>
          </div>
          <button
            class="sync-btn"
            v-if="!userData.bind_mini"
            @click="openMiniModal"
          >
            {{ $t("my.syncBtn") }}
          </button>
        </div>

        <!-- 功能菜单列表 -->
        <div class="menu-list">
          <div class="menu-item" @click="$router.push('/shop/cart')">
            <i class="fa fa-shopping-cart"></i>
            <span>{{ $t("my.cartMenu") }}</span>
            <i class="fa fa-angle-right"></i>
          </div>
          <div class="menu-item" @click="$router.push('/shop/order')">
            <i class="fa fa-list-alt"></i>
            <span>{{ $t("my.orderMenu") }}</span>
            <i class="fa fa-angle-right"></i>
          </div>
          <div class="menu-item" @click="$router.push('/about')">
            <i class="fa fa-info-circle"></i>
            <span>{{ $t("my.aboutMenu") }}</span>
            <i class="fa fa-angle-right"></i>
          </div>
        </div>
      </div>

      <!-- 未登录区域 包裹在卡片内 -->
      <div v-else class="no-login-card">
        <p>{{ $t("my.noLoginTip") }}</p>
        <button class="login-btn" @click="piLogin">
          {{ $t("my.piLoginBtn") }}
        </button>
      </div>
    </div>

    <!-- 小程序绑定弹窗 -->
    <div
      class="modal-mask"
      v-if="showMiniModal"
      @click.self="showMiniModal = false"
    >
      <div class="modal-box">
        <h4>{{ $t("my.modalTitle") }}</h4>
        <input v-model="miniInput" :placeholder="$t('my.inputPlaceholder')" />
        <div class="modal-btns">
          <button class="cancel" @click="showMiniModal = false">
            {{ $t("my.cancelBtn") }}
          </button>
          <button class="confirm" @click="submitBindMini">
            {{ $t("my.confirmSyncBtn") }}
          </button>
        </div>
      </div>
    </div>

    <!-- 底部留白，避开全局导航 -->
    <div class="space-bottom"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { useI18n } from "@/i18n";
import axios from "axios";

const { t } = useI18n();

// localStorage存储结构（图纸标注：uid、accessToken）
const userInfo = ref<{
  username: string;
  uid: string;
  accessToken: string;
} | null>(null);

// 后端返回数据（wallet字段保留，小程序同步字段）
interface BackendUserData {
  pi_uid: string;
  nickname: string | null;
  upi_num: number | null;
  wallet_address: string | null;
  bind_mini: boolean;
}
const userData = ref<BackendUserData>({
  pi_uid: "",
  nickname: null,
  upi_num: null,
  wallet_address: null,
  bind_mini: false,
});

// 小程序弹窗变量
const showMiniModal = ref(false);
const miniInput = ref("");

// 读取本地存储
const loadUserInfo = () => {
  const cache = localStorage.getItem("pi_user");
  if (!cache) return;
  try {
    userInfo.value = JSON.parse(cache);
  } catch {
    localStorage.removeItem("pi_user");
  }
};

// 请求后端用户数据
const fetchUserData = async () => {
  if (!userInfo.value?.uid) return;
  try {
    const res = await axios.post("/api/v1/my/data", {
      uid: userInfo.value.uid,
    });
    if (res.data.code === 0) {
      userData.value = res.data.data;
    }
  } catch (err) {
    console.error("拉取用户数据失败", err);
  }
};

// 小程序同步提交
const submitBindMini = async () => {
  const key = miniInput.value.trim();
  if (!key) return alert(t("my.emptyKeyAlert"));
  try {
    const res = await axios.post("/api/v1/my/bind-mini", {
      pi_uid: userInfo.value!.uid,
      mini_openid: key,
    });
    if (res.data.code === 0) {
      showMiniModal.value = false;
      miniInput.value = "";
      await fetchUserData();
      alert(t("my.syncSuccessAlert"));
    } else {
      alert(res.data.msg || t("my.syncFailAlert"));
    }
  } catch {
    alert(t("my.networkErrAlert"));
  }
};

// Pi登录逻辑（统一存储pi_user）
const piLogin = async () => {
  let wait = 0;
  while (typeof (window as any).Pi === "undefined" && wait < 20) {
    await new Promise((resolve) => setTimeout(resolve, 100));
    wait++;
  }
  const Pi = (window as any).Pi;
  if (!Pi) return alert(t("my.openInPiAlert"));
  try {
    const auth = await Pi.authenticate(["payments", "username"]);
    const storeData = {
      username: auth.user.username,
      uid: auth.user.uid,
      accessToken: auth.accessToken,
    };
    localStorage.setItem("pi_user", JSON.stringify(storeData));
    userInfo.value = storeData;
    fetchUserData();
    alert(t("my.loginSuccessAlert"));
  } catch (e: any) {
    alert(t("my.loginFailPrefix") + e.message);
  }
};

// 弹窗开关
const openMiniModal = () => {
  miniInput.value = "";
  showMiniModal.value = true;
};

onMounted(() => loadUserInfo());
watch(userInfo, (val) => val?.uid && fetchUserData(), { immediate: true });
</script>

<style scoped>
/* 页面整体灰色背景，和合约/商城统一 */
.page-my {
  background: #f5f5f5;
  min-height: 100vh;
}
/* 顶部通栏纯白标题栏，完全对齐其他页面 */
.header-title {
  background: #ffffff;
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 14px 16px;
}
.title-icon {
  font-size: 22px;
  color: #0066cc;
}
.page-title {
  font-size: 20px;
  margin: 0;
  color: #333;
}
/* 内容区域左右边距 */
.page-content {
  padding: 16px 12px;
}
/* 基础信息卡片 白色圆角 */
.info-card {
  background: #fff;
  border-radius: 8px;
  padding: 14px;
  margin-bottom: 14px;
}
.info-row {
  display: flex;
  padding: 8px 0;
  border-bottom: 1px solid #eee;
}
.info-row:last-child {
  border-bottom: none;
}
.info-row label {
  width: 90px;
  color: #666;
}
/* 小程序同步卡片 */
.mini-sync-card {
  background: #fff;
  border-radius: 8px;
  padding: 14px;
  margin-bottom: 14px;
}
.sync-row {
  display: flex;
  padding: 8px 0;
}
.sync-row label {
  width: 90px;
  color: #666;
}
.sync-btn {
  margin-top: 10px;
  padding: 6px 12px;
  background: #0066cc;
  color: #fff;
  border: none;
  border-radius: 4px;
}
/* 菜单列表 白色卡片 */
.menu-list {
  background: #fff;
  border-radius: 8px;
}
.menu-item {
  display: flex;
  align-items: center;
  padding: 16px 14px;
  border-bottom: 1px solid #eee;
}
.menu-item:last-child {
  border-bottom: none;
}
.menu-item i:first-child {
  font-size: 18px;
  color: #0066cc;
  width: 32px;
}
.menu-item span {
  flex: 1;
}
.menu-item i:last-child {
  color: #ccc;
}
/* 未登录白色卡片 */
.no-login-card {
  background: #fff;
  border-radius: 8px;
  padding: 40px 20px;
  text-align: center;
}
.no-login-card p {
  font-size: 16px;
  color: #666;
  margin: 0 0 16px;
}
.login-btn {
  padding: 10px 24px;
  background: #0066cc;
  color: #fff;
  border: none;
  border-radius: 6px;
}
/* 弹窗样式 */
.modal-mask {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 999;
}
.modal-box {
  width: 85%;
  max-width: 340px;
  background: #fff;
  border-radius: 10px;
  padding: 20px;
}
.modal-box h4 {
  text-align: center;
  margin: 0 0 16px;
}
.modal-box input {
  width: 100%;
  box-sizing: border-box;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 6px;
  margin-bottom: 18px;
}
.modal-btns {
  display: flex;
  gap: 12px;
}
.modal-btns button {
  flex: 1;
  padding: 10px;
  border: none;
  border-radius: 6px;
}
.cancel {
  background: #eee;
}
.confirm {
  background: #0066cc;
  color: #fff;
}
/* 底部留白避开导航 */
.space-bottom {
  height: 70px;
}
</style>
