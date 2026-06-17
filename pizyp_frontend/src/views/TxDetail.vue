<template>
  <div class="tx-detail-page">
    <!-- 顶部紫色导航栏 -->
    <div class="top-nav">
      <div class="back-btn" @click="router.back()">
        <i class="fa fa-arrow-left"></i> {{ $t("txDetail.backText") }}
      </div>
      <h1>{{ $t("txDetail.pageTitle") }}</h1>
      <div></div>
    </div>

    <!-- 详情卡片容器 -->
    <div class="detail-card" v-if="!loading && !errorText">
      <div class="detail-title">{{ $t("txDetail.infoTitle") }}</div>
      <div class="detail-item">
        <span class="detail-label">{{ $t("txDetail.hashLabel") }}</span>
        <span class="detail-value">{{ txData.tx_hash }}</span>
      </div>
      <div class="detail-item">
        <span class="detail-label">{{ $t("txDetail.ledgerLabel") }}</span>
        <span class="detail-value">{{ txData.ledger }}</span>
      </div>
      <div class="detail-item">
        <span class="detail-label">{{ $t("txDetail.timeLabel") }}</span>
        <span class="detail-value">{{ txData.tx_time }}</span>
      </div>
      <div class="detail-item">
        <span class="detail-label">{{ $t("txDetail.fromLabel") }}</span>
        <span class="detail-value">{{ txData.from_addr }}</span>
      </div>
      <div class="detail-item">
        <span class="detail-label">{{ $t("txDetail.toLabel") }}</span>
        <span class="detail-value">{{ txData.to_addr }}</span>
      </div>
      <div class="detail-item">
        <span class="detail-label">{{ $t("txDetail.amountLabel") }}</span>
        <span class="detail-value"
          >{{ formatAmount(txData.amount) }} {{ txData.token_type }}</span
        >
      </div>
      <div class="detail-item">
        <span class="detail-label">{{ $t("txDetail.typeLabel") }}</span>
        <span class="detail-value">{{ txData.event_type }}</span>
      </div>
    </div>

    <!-- 加载/错误占位 -->
    <div v-if="loading" class="detail-card loading-box">
      <div class="loader"></div>
      <div>{{ $t("txDetail.loading") }}</div>
    </div>
    <div v-if="errorText" class="detail-card error-box">{{ errorText }}</div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";

const route = useRoute();
const router = useRouter();
const loading = ref(true);
const errorText = ref("");
const txData = ref<any>({});

// 统一20s超时POST请求（全局规范：code===0，取res.data）
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
    console.error("请求报错：", e);
    throw new Error(msg);
  }
};

// 金额格式化 除以1e6保留两位小数
const formatAmount = (rawAmt: string | number) => {
  const num = Number(rawAmt) / 1000000;
  return num.toFixed(2);
};

// 加载交易详情数据
const loadTxDetail = async () => {
  loading.value = true;
  errorText.value = "";
  try {
    const txHash = route.query.hash as string;
    if (!txHash) {
      errorText.value = "无效交易哈希";
      return;
    }
    // 接口升级v1路径
    const json = await httpPost("/api/v1/browser/tx-detail", {
      tx_hash: txHash,
    });
    txData.value = json.data;
  } catch (e: any) {
    console.error("加载异常：", e);
    errorText.value = e.message || "加载失败";
  } finally {
    loading.value = false;
  }
};

onMounted(() => loadTxDetail());
</script>

<style scoped>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}
.tx-detail-page {
  font-family:
    -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  background: #ffffff;
  min-height: 100vh;
  padding-bottom: 70px;
}
.top-nav {
  background: linear-gradient(135deg, #7928ca, #5a189a);
  color: #fff;
  padding: 18px 16px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.top-nav h1 {
  font-size: 20px;
  font-weight: 600;
}
.back-btn {
  font-size: 16px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 4px;
}
/* 详情卡片样式 */
.detail-card {
  margin: 16px;
  padding: 16px;
  background: #fff;
  border-radius: 12px;
  box-shadow: 0 2px 10px rgba(121, 40, 202, 0.1);
}
.detail-title {
  color: #222;
  font-size: 18px;
  margin-bottom: 10px;
  font-weight: 600;
}
.detail-item {
  font-size: 14px;
  color: #444;
  line-height: 1.6;
  margin: 6px 0;
  word-break: break-all;
  display: flex;
}
.detail-label {
  font-weight: 500;
  color: #222;
  min-width: 90px;
}
.detail-value {
  color: #5a189a;
  font-weight: 500;
  word-break: break-all;
  flex: 1;
}
/* 加载动画 */
.loading-box {
  text-align: center;
  padding: 40px 0;
  color: #666;
}
.error-box {
  text-align: center;
  padding: 40px 0;
  color: #999;
}
.loader {
  width: 40px;
  height: 40px;
  border: 3px solid #eee;
  border-top: 3px solid #7b2cbf;
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
