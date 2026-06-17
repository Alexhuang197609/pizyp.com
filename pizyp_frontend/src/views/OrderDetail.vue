<template>
  <div class="page-order-detail">
    <!-- 全站统一纯白顶部标题栏，和My页面结构一致 -->
    <div class="header-title">
      <i class="fa fa-file-text-o title-icon"></i>
      <h2 class="page-title">{{ $t("orderDetail.pageTitle") }}</h2>
    </div>

    <div class="page-content">
      <!-- 订单基础信息卡片 -->
      <div v-if="orderInfo" class="info-card">
        <div class="info-row">
          <label>{{ $t("orderDetail.statusLabel") }}</label>
          <span class="status-value">{{ orderInfo.status_text }}</span>
        </div>
        <div class="info-row">
          <label>{{ $t("orderDetail.orderNoLabel") }}</label>
          <span>{{ orderInfo.order_no }}</span>
        </div>
        <div class="info-row">
          <label>{{ $t("orderDetail.createTimeLabel") }}</label>
          <span>{{ orderInfo.create_time }}</span>
        </div>
      </div>

      <!-- 商品明细卡片 -->
      <div v-if="goodsList.length > 0" class="info-card">
        <div class="goods-title">{{ $t("orderDetail.goodsTitle") }}</div>
        <div
          class="goods-item"
          v-for="item in goodsList"
          :key="item.goods_name + item.num"
        >
          <div class="goods-left">
            <div class="goods-name">{{ item.goods_name }}</div>
            <div class="goods-spec">
              {{ $t("orderDetail.specLabel")
              }}{{ item.goods_spec || $t("orderDetail.noSpec") }}
            </div>
          </div>
          <div class="goods-right">
            <div class="goods-price">{{ item.price_pi }} π</div>
            <div class="goods-num">x{{ item.num }}</div>
          </div>
        </div>
      </div>

      <!-- 金额汇总卡片 -->
      <div v-if="orderInfo" class="info-card">
        <div class="info-row">
          <label>{{ $t("orderDetail.subTotalLabel") }}</label>
          <span>{{ orderInfo.total_price_pi }} π</span>
        </div>
        <div class="info-row">
          <label>{{ $t("orderDetail.actualPayLabel") }}</label>
          <span class="total-price">{{ orderInfo.total_price_pi }} π</span>
        </div>
      </div>

      <!-- 仅待支付展示支付按钮 -->
      <div v-if="orderInfo?.status === 0" class="pay-wrap">
        <button class="pay-btn" :disabled="payLoading" @click="startPiPay">
          {{
            payLoading
              ? $t("orderDetail.payLoadingText")
              : $t("orderDetail.payBtnText")
          }}
        </button>
      </div>

      <!-- 空状态/未登录提示 -->
      <div v-else-if="!loading" class="no-login-card">
        <p>
          {{
            loginTip
              ? $t("orderDetail.loginTipText")
              : $t("orderDetail.emptyTipText")
          }}
        </p>
      </div>
    </div>

    <!-- 底部留白避让全局导航 -->
    <div class="space-bottom"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { useRoute } from "vue-router";
import { useI18n } from "@/i18n";
import axios from "axios";
const { t } = useI18n();

// 商品明细类型
interface GoodsItem {
  goods_name: string;
  goods_spec: string | null;
  price_pi: number;
  num: number;
}
// 订单基础信息类型
interface OrderInfo {
  order_no: string;
  total_price_pi: number;
  status: number;
  status_text: string;
  create_time: string;
}

const route = useRoute();
const loading = ref(true);
const loginTip = ref(false);
const payLoading = ref(false);
const orderInfo = ref<OrderInfo | null>(null);
const goodsList = ref<GoodsItem[]>([]);

// 用户缓存信息，和My页面结构完全统一
let userCache: {
  username: string;
  uid: string;
  accessToken: string;
} | null = null;
let targetOrderId = "";
let payTotalPrice = 0;
let payOrderNo = "";

// 读取本地登录缓存（复刻My页面loadUserInfo逻辑）
const loadUserCache = () => {
  const cacheStr = localStorage.getItem("pi_user");
  if (!cacheStr) return;
  try {
    userCache = JSON.parse(cacheStr);
  } catch {
    localStorage.removeItem("pi_user");
  }
};

// 加载订单详情接口 /api/v1
const loadOrderDetail = async () => {
  loading.value = true;
  orderInfo.value = null;
  goodsList.value = [];
  loadUserCache();

  // 提前拦截空值，TS后续不再报null
  if (!userCache) {
    loginTip.value = true;
    loading.value = false;
    return;
  }
  targetOrderId = String(route.query.id);
  if (!targetOrderId) {
    loading.value = false;
    return;
  }

  try {
    const res = await axios.post("/api/v1/order/detail", {
      uid: userCache.uid,
      order_id: targetOrderId,
    });
    if (res.data.code === 0) {
      const data = res.data.data;
      orderInfo.value = data.order;
      goodsList.value = data.list || [];
      payTotalPrice = data.order.total_price_pi;
      payOrderNo = data.order.order_no;
    }
  } catch (err) {
    console.error("订单详情加载失败", err);
  } finally {
    loading.value = false;
  }
};

// Pi支付函数（完全复刻My页面piLogin的Pi等待逻辑，全局window.Pi）
const startPiPay = async () => {
  if (!orderInfo.value || orderInfo.value.status !== 0 || !userCache) return;
  payLoading.value = true;

  // 复刻My页面20次循环等待全局Pi加载
  let waitPiCount = 0;
  while (typeof (window as any).Pi === "undefined" && waitPiCount < 20) {
    await new Promise((resolve) => setTimeout(resolve, 100));
    waitPiCount++;
  }
  const Pi = (window as any).Pi;
  if (!Pi) {
    alert(t("orderDetail.openInPiAlert"));
    payLoading.value = false;
    return;
  }

  try {
    // Pi授权scope和My页面保持一致
    await Pi.authenticate(["payments", "username"]);
    await Pi.createPayment(
      {
        amount: payTotalPrice,
        memo: `${t("orderDetail.payMemoPrefix")}${payOrderNo}`,
        metadata: {
          type: "order",
          order_id: targetOrderId,
        },
      },
      {
        // 后端审批接口 v1，增加空值判断
        onReadyForServerApproval: async (paymentId: string) => {
          if (!userCache) return;
          await axios.post(
            `/api/v1/pi/payments/${paymentId}/approve`,
            { order_id: targetOrderId },
            {
              headers: {
                Authorization: `Bearer ${userCache.accessToken}`,
              },
            },
          );
        },
        // 支付完成回调，增加空值判断
        onReadyForServerCompletion: async (paymentId: string, txid: string) => {
          if (!userCache) return;
          await axios.post(
            `/api/v1/pi/payments/${paymentId}/complete`,
            { order_id: targetOrderId, txid: txid },
            {
              headers: {
                Authorization: `Bearer ${userCache.accessToken}`,
              },
            },
          );
          alert(t("orderDetail.paySuccessAlert"));
          loadOrderDetail();
        },
        onCancel: () => alert(t("orderDetail.payCancelAlert")),
        onError: (err: any) =>
          alert(`${t("orderDetail.payErrorPrefix")}${err.message}`),
      },
    );
  } catch (e: any) {
    alert(`${t("orderDetail.payStartFailPrefix")}${e.message}`);
    console.error(e);
  } finally {
    payLoading.value = false;
  }
};

onMounted(() => loadOrderDetail());
// 路由ID变化重载详情
watch(
  () => route.query.id,
  () => loadOrderDetail(),
);
</script>

<style scoped>
/* 页面底色与My全站统一 #f5f5f5 */
.page-order-detail {
  background: #f5f5f5;
  min-height: 100vh;
}
/* 顶部标题栏 完全复刻My页面样式 */
.header-title {
  background: #ffffff;
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 14px 16px;
  border-bottom: 1px solid #eee;
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
.page-content {
  padding: 16px 12px;
}
/* 统一卡片样式 8px圆角 与My.info-card一致 */
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
.status-value {
  color: #0066cc;
  font-weight: bold;
}
.total-price {
  color: #e64340;
  font-weight: bold;
}
/* 商品明细样式 */
.goods-title {
  font-size: 15px;
  font-weight: bold;
  margin-bottom: 12px;
}
.goods-item {
  display: flex;
  justify-content: space-between;
  padding: 10px 0;
  border-bottom: 1px solid #f5f5f5;
}
.goods-item:last-child {
  border-bottom: none;
}
.goods-left {
  flex: 1;
}
.goods-name {
  font-size: 14px;
  color: #333;
}
.goods-spec {
  font-size: 12px;
  color: #999;
  margin-top: 4px;
}
.goods-right {
  text-align: right;
}
.goods-price {
  font-size: 14px;
  color: #e64340;
}
.goods-num {
  font-size: 12px;
  color: #999;
  margin-top: 4px;
}
/* 支付按钮区域 */
.pay-wrap {
  margin: 10px 0 20px;
  text-align: right;
}
.pay-btn {
  padding: 10px 22px;
  background: #e64340;
  color: #fff;
  border: none;
  border-radius: 6px;
  font-size: 15px;
  cursor: pointer;
}
.pay-btn:disabled {
  background: #cccccc;
  cursor: not-allowed;
}
/* 未登录空卡片 复刻My.no-login-card */
.no-login-card {
  background: #fff;
  border-radius: 8px;
  padding: 60px 20px;
  text-align: center;
  color: #999;
  font-size: 15px;
}
/* 底部留白避让全局导航 */
.space-bottom {
  height: 70px;
}
</style>
