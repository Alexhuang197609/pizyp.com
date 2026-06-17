<template>
  <div class="page-shop-order">
    <!-- 全站统一顶部标题栏 -->
    <div class="header-title">
      <i class="fa fa-list-alt title-icon"></i>
      <h2 class="page-title">{{ $t("orderList.pageTitle") }}</h2>
    </div>

    <div class="page-content">
      <!-- 未登录提示卡片 -->
      <div v-if="!userCache" class="no-login-card" @click="$router.push('/my')">
        <p>{{ $t("orderList.noLoginTip") }}</p>
      </div>

      <!-- 订单列表区域 -->
      <div v-else-if="orderList.length > 0" class="order-list-wrap">
        <div
          class="order-item"
          v-for="item in orderList"
          :key="item.id"
          @click="
            $router.push({
              path: '/order/detail',
              query: { id: item.id },
            })
          "
        >
          <div class="order-top-row">
            <div class="order-no-text">
              {{ $t("orderList.orderNoLabel") }}{{ item.order_no }}
            </div>
            <span class="status-tag">{{ item.status_text }}</span>
          </div>
          <div class="order-time-row">
            {{ $t("orderList.createTimeLabel") }}{{ item.create_time }}
          </div>
          <div class="order-price-row">
            {{ $t("orderList.totalLabel")
            }}<span class="price-text">{{ item.total_price_pi }}</span> π
          </div>
          <div class="btn-row">
            <!-- stop阻止冒泡，点击按钮不触发卡片跳转 -->
            <button class="del-btn" @click.stop="handleDeleteOrder(item.id)">
              {{ $t("orderList.delBtn") }}
            </button>
          </div>
        </div>
      </div>

      <!-- 无订单空白提示 -->
      <div v-else class="empty-card">
        <p>{{ $t("orderList.emptyTip") }}</p>
      </div>
    </div>

    <!-- 底部留白避开全局底部导航 -->
    <div class="space-bottom"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useI18n } from "@/i18n";
import axios from "axios";
const { t } = useI18n();

// 订单单项类型
interface OrderItem {
  id: number;
  order_no: string;
  total_price_pi: number;
  status: number;
  status_text: string;
  create_time: string;
}

const loading = ref(true);
const orderList = ref<OrderItem[]>([]);
// 修复核心：使用ref响应式变量，缓存读取后页面自动更新（旧版正常逻辑）
const userCache = ref<{
  username: string;
  uid: string;
  accessToken: string;
} | null>(null);

// 读取本地登录缓存
const loadUserCache = () => {
  const cacheStr = localStorage.getItem("pi_user");
  if (!cacheStr) {
    userCache.value = null;
    return;
  }
  try {
    userCache.value = JSON.parse(cacheStr);
  } catch {
    localStorage.removeItem("pi_user");
    userCache.value = null;
  }
};

// 加载订单列表接口
const loadOrderList = async () => {
  loading.value = true;
  orderList.value = [];

  if (!userCache.value) {
    loading.value = false;
    return;
  }

  try {
    const res = await axios.post("/api/v1/order/list", {
      uid: userCache.value.uid,
    });
    if (res.data.code === 0) {
      orderList.value = res.data.data;
    }
  } catch (err) {
    console.error("加载订单列表失败", err);
  } finally {
    loading.value = false;
  }
};

// 删除订单核心方法（已修复传参，对齐后端接口）
const handleDeleteOrder = async (orderId: number) => {
  if (!userCache.value) {
    alert(t("orderList.needLoginAlert"));
    return;
  }
  const confirmRes = confirm(t("orderList.delConfirmTip"));
  if (!confirmRes) return;

  try {
    const res = await axios.post("/api/v1/order/delete", {
      uid: userCache.value.uid,
      order_id: String(orderId),
    });
    if (res.data.code === 0) {
      alert(t("orderList.delSuccessAlert"));
      loadOrderList(); // 刷新列表清除已删除条目
    } else {
      alert(res.data.msg);
    }
  } catch (err) {
    alert(t("orderList.networkFailAlert"));
  }
};

// 页面挂载先读取缓存，再加载订单
onMounted(() => {
  loadUserCache();
  loadOrderList();
});
</script>

<style scoped>
/* 页面底色和全站My页面统一 */
.page-shop-order {
  background: #f5f5f5;
  min-height: 100vh;
}
/* 顶部标题栏 */
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
/* 无登录空白卡片 */
.no-login-card,
.empty-card {
  background: #fff;
  border-radius: 8px;
  padding: 60px 20px;
  text-align: center;
  color: #999;
  font-size: 15px;
}
.no-login-card {
  cursor: pointer;
}
/* 订单列表容器 */
.order-list-wrap {
  display: flex;
  flex-direction: column;
  gap: 14px;
}
/* 单个订单卡片 */
.order-item {
  background: #fff;
  border-radius: 8px;
  padding: 14px;
}
.order-top-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}
.order-no-text {
  font-size: 16px;
  color: #333;
}
.status-tag {
  padding: 4px 8px;
  background: #e6f0ff;
  color: #0066cc;
  border-radius: 4px;
  font-size: 14px;
}
.order-time-row {
  font-size: 14px;
  color: #666;
  margin-bottom: 8px;
}
.order-price-row {
  font-size: 18px;
  margin-bottom: 12px;
}
.price-text {
  color: #e64340;
  font-weight: bold;
}
.btn-row {
  display: flex;
  justify-content: flex-end;
}
.del-btn {
  padding: 6px 16px;
  background: #e64340;
  color: #fff;
  border: none;
  border-radius: 4px;
  font-size: 14px;
}
/* 底部留白避让导航 */
.space-bottom {
  height: 70px;
}
</style>
