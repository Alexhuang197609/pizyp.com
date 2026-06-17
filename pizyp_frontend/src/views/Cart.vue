<template>
  <div class="page-cart">
    <!-- 全站统一纯白顶部标题栏 -->
    <div class="header-title">
      <i class="fa fa-angle-left back-icon" @click="$router.back()"></i>
      <i class="fa fa-shopping-cart title-icon"></i>
      <h2 class="page-title">{{ $t("cart.pageTitle") }}</h2>
    </div>

    <div class="page-content">
      <!-- 空状态 -->
      <div v-if="cartList.length === 0" class="empty-card">
        <p>{{ loginTip ? $t("cart.loginEmptyTip") : $t("cart.emptyTip") }}</p>
      </div>

      <!-- 购物车商品列表 -->
      <div v-else>
        <div
          class="cart-item"
          v-for="item in cartList"
          :key="item.id"
          :data-cart-id="item.id"
        >
          <div class="item-info">
            <div class="item-name">{{ item.goods_name }}</div>
            <div class="item-spec">
              {{ item.goods_spec || $t("cart.noSpec") }}
            </div>
            <div class="item-price">{{ item.price_pi }} π</div>
          </div>
          <div class="num-box">
            <div class="num-btn minus" @click="changeNum(item.id, -1)">-</div>
            <div class="num-text">{{ item.num }}</div>
            <div class="num-btn plus" @click="changeNum(item.id, 1)">+</div>
          </div>
        </div>
      </div>
    </div>

    <!-- 底部结算栏 -->
    <div class="cart-bottom-bar" v-if="cartList.length > 0">
      <div class="total-text">
        {{ $t("cart.totalText") }}<span>{{ totalPrice.toFixed(2) }}</span> π
      </div>
      <button class="pay-btn" @click="handleSettle">
        {{ $t("cart.settleBtn") }}
      </button>
    </div>

    <!-- 底部留白避让全局App.vue导航 -->
    <div class="space-bottom"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useI18n } from "@/i18n";
import axios from "axios";
const { t } = useI18n();

// 购物车商品类型定义
interface CartItem {
  id: number;
  goods_name: string;
  goods_spec: string | null;
  price_pi: number;
  num: number;
}

// 响应式数据
const cartList = ref<CartItem[]>([]);
const loginTip = ref(false);
let userUid = "";

// 计算总价
const totalPrice = computed(() => {
  return cartList.value.reduce(
    (sum, item) => sum + item.price_pi * item.num,
    0,
  );
});

// 加载购物车数据（接口升级/api/v1）
const loadCart = async () => {
  const userStr = localStorage.getItem("pi_user");
  if (!userStr) {
    loginTip.value = true;
    cartList.value = [];
    return;
  }
  const userInfo = JSON.parse(userStr);
  userUid = userInfo.uid;

  try {
    const res = await axios.post("/api/v1/cart/list", { uid: userUid });
    if (res.data.code === 0) {
      cartList.value = res.data.data || [];
    } else {
      cartList.value = [];
    }
  } catch (err) {
    console.error("加载购物车失败", err);
    cartList.value = [];
  }
};

// 修改商品数量（接口升级/api/v1）
const changeNum = async (cartId: number, step: number) => {
  try {
    await axios.post("/api/v1/cart/update-num", {
      cart_id: cartId,
      step: step,
    });
    loadCart();
  } catch (err) {
    alert(t("cart.changeNumFailAlert"));
  }
};

// 结算提交订单（接口升级/api/v1）
const handleSettle = async () => {
  if (!userUid) {
    alert(t("cart.needLoginAlert"));
    return;
  }
  if (cartList.value.length === 0) {
    alert(t("cart.emptyCartAlert"));
    return;
  }
  const cartIds = cartList.value.map((item) => item.id);

  try {
    const res = await axios.post("/api/v1/order/settle", {
      uid: userUid,
      cart_ids: cartIds,
    });
    if (res.data.code === 0) {
      alert(
        t("cart.orderSuccessPrefix") +
          res.data.data.order_no +
          t("cart.orderAmountSuffix") +
          res.data.data.total_price +
          " π",
      );
      loadCart();
    } else {
      alert(t("cart.orderFailPrefix") + (res.data.msg || ""));
    }
  } catch (err) {
    alert(t("cart.networkErrAlert"));
  }
};

onMounted(() => loadCart());
</script>

<style scoped>
.page-cart {
  background: #f5f5f5;
  min-height: 100vh;
}
/* 顶部标题栏 和 My/合约/商城完全统一 */
.header-title {
  background: #ffffff;
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 14px 16px;
  border-bottom: 1px solid #eee;
}
.back-icon {
  font-size: 22px;
  color: #333;
  cursor: pointer;
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
/* 空状态卡片 */
.empty-card {
  background: #fff;
  border-radius: 8px;
  padding: 60px 20px;
  text-align: center;
  color: #999;
  font-size: 15px;
}
/* 购物车商品卡片 圆角8px全站标准 */
.cart-item {
  background: #fff;
  border-radius: 8px;
  padding: 12px;
  margin-bottom: 12px;
  display: flex;
  align-items: center;
  gap: 12px;
}
.item-info {
  flex: 1;
}
.item-name {
  font-size: 15px;
  font-weight: bold;
  color: #222;
}
.item-spec {
  font-size: 12px;
  color: #999;
  margin: 4px 0;
}
.item-price {
  font-size: 16px;
  color: #e64340;
  font-weight: bold;
}
.num-box {
  display: flex;
  align-items: center;
  gap: 8px;
}
.num-btn {
  width: 28px;
  height: 28px;
  border-radius: 50%;
  background: #eee;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 16px;
  cursor: pointer;
  user-select: none;
}
.num-text {
  width: 30px;
  text-align: center;
  font-size: 15px;
}
/* 底部结算栏 位置适配全局导航高度60px */
.cart-bottom-bar {
  position: fixed;
  left: 0;
  bottom: 60px;
  width: 100%;
  background: #fff;
  padding: 12px 16px;
  border-top: 1px solid #eee;
  display: flex;
  justify-content: space-between;
  align-items: center;
  z-index: 99;
}
.total-text {
  font-size: 16px;
  font-weight: bold;
  color: #e64340;
}
.pay-btn {
  background: #0066cc;
  color: #fff;
  border: none;
  padding: 8px 18px;
  border-radius: 6px;
  font-size: 14px;
  cursor: pointer;
}
/* 底部避让空白，给App.vue导航留出空间 */
.space-bottom {
  height: 70px;
}
</style>
