<template>
  <div class="contract-page">
    <!-- 顶部标题栏 -->
    <div class="head-title">{{ $t("contract.title") }}</div>

    <div class="list-wrap">
      <!-- 加载动画 -->
      <div v-if="loading" class="loading-box">
        <div class="loader"></div>
        <div>{{ $t("contract.loading") }}</div>
      </div>
      <!-- 错误提示 -->
      <div v-else-if="errorText" class="loading-box error">{{ errorText }}</div>
      <!-- 合约列表卡片 -->
      <div v-else>
        <div
          class="content-card"
          v-for="item in contractList"
          :key="item.contract_addr"
          @click="handleCardClick(item)"
        >
          <img
            v-if="item.logo_base64"
            class="contract-logo"
            :src="`data:image/jpeg;base64,${item.logo_base64}`"
          />
          <div class="card-text">
            <h3>{{ item.name }}</h3>
            <p>{{ item.desc }}</p>
            <p>
              <strong>{{ $t("contract.statusLabel") }}</strong
              >{{ item.status }}
            </p>
            <p v-if="item.contract_addr">
              <strong>{{ $t("contract.addrLabel") }}</strong
              >{{ item.contract_addr }}
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();
const loading = ref(true);
const errorText = ref("");
const contractList = ref<any[]>([]);

// 统一请求封装（超时20s，完全对齐原始JS逻辑）
const httpPost = async (url: string, data: any) => {
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), 20000);
  try {
    const res = await fetch(url, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(data),
      signal: controller.signal,
    });
    clearTimeout(timer);
    const json = await res.json();
    if (json.code !== 0) throw new Error(json.msg || "请求失败");
    return json;
  } catch (e: any) {
    clearTimeout(timer);
    let msg = e.message || "网络异常";
    if (e.name === "AbortError") msg = "请求超时，请检查网络";
    throw new Error(msg);
  }
};

// 读取本地登录用户信息
const getUserInfo = () => {
  const userStr = localStorage.getItem("pi_user");
  return userStr ? JSON.parse(userStr) : null;
};

// 加载合约列表（接口升级v1前缀）
const loadContractData = async () => {
  loading.value = true;
  errorText.value = "";
  try {
    const resData = await httpPost("/api/v1/contract/data", null);
    contractList.value = resData.data || [];
  } catch (err: any) {
    errorText.value = err.message || "加载失败，请刷新重试";
    console.error("合约列表加载异常", err);
  } finally {
    loading.value = false;
  }
};

// 卡片点击跳转 1:1还原旧页面分支逻辑
const handleCardClick = (item: any) => {
  if (item.name === "生肖猜猜乐") {
    router.push("/zodiac-game");
  } else if (item.name === "SUPI" && item.contract_addr) {
    router.push(`/browser?addr=${item.contract_addr}`);
  }
};

onMounted(async () => {
  await loadContractData();
  getUserInfo();
});
</script>

<style scoped>
.contract-page {
  background: #f5f5f5;
  min-height: 100vh;
  font-family:
    -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue",
    Arial, sans-serif;
}
.head-title {
  padding: 16px;
  background: #fff;
  font-size: 18px;
  font-weight: bold;
  border-bottom: 1px solid #eee;
  position: sticky;
  top: 0;
  z-index: 99;
}

.content-card {
  background: #fff;
  margin: 12px;
  border-radius: 12px;
  padding: 16px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  display: flex;
  align-items: flex-start;
  gap: 16px;
  cursor: pointer;
}
.contract-logo {
  width: 80px;
  height: 80px;
  border-radius: 8px;
  object-fit: cover;
  flex-shrink: 0;
}
.card-text {
  flex: 1;
  min-width: 0;
}
.card-text h3 {
  margin: 0 0 8px 0;
  font-size: 16px;
  color: #333;
}
.card-text p {
  margin: 0 0 6px 0;
  line-height: 1.6;
  color: #333;
  font-size: 14px;
  word-break: break-all;
}
.loading-box {
  text-align: center;
  padding: 40px 0;
  color: #666;
}
.loading-box.error {
  color: #999;
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
</style>
