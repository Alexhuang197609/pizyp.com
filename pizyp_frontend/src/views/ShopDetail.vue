<template>
  <div class="detail-page">
    <!-- 顶部返回头部 对齐旧页面样式 -->
    <div class="page-header">
      <span class="back-btn" @click="$router.back()"
        ><i class="fa fa-arrow-left"></i> {{ $t("shopDetail.backText") }}</span
      >
      <h3>{{ $t("shopDetail.pageTitle") }}</h3>
    </div>

    <div v-if="loading" class="tip">{{ $t("shopDetail.loading") }}</div>
    <div v-else-if="errorText" class="tip error">{{ errorText }}</div>

    <div v-else class="content-wrap">
      <!-- 自动轮播图 -->
      <div class="banner">
        <div class="banner-wrap">
          <div class="banner-item" v-for="img in imgArr" :key="img">
            <img :src="`data:image/jpeg;base64,${img}`" alt="商品图" />
          </div>
        </div>
      </div>

      <!-- 商品基础信息卡片 -->
      <div class="info-card">
        <h2 class="goods-name">{{ detail.goods_name }}</h2>
        <div class="price">{{ detail.price_pi }} π</div>
        <div class="row">
          {{ $t("shopDetail.spec") }}{{ detail.goods_spec }}
        </div>
        <div class="row">
          {{ $t("shopDetail.stock") }}{{ detail.goods_stock }} &nbsp;&nbsp;
          {{ $t("shopDetail.sales") }}：{{ detail.goods_sales }}
        </div>
        <div class="row">
          {{ $t("shopDetail.ship") }}{{ detail.goods_ship_addr }}
        </div>
      </div>

      <!-- 商品简介卡片 -->
      <div class="desc-card">
        <h4 class="card-title">{{ $t("shopDetail.briefTitle") }}</h4>
        <p class="desc-text">{{ detail.goods_desc }}</p>
      </div>

      <!-- 商品详情大图卡片 -->
      <div class="desc-card">
        <h4 class="card-title">{{ $t("shopDetail.detailTitle") }}</h4>
        <div class="img-block" v-for="b64 in detailImgs" :key="b64">
          <img :src="`data:image/jpeg;base64,${b64}`" />
        </div>
      </div>
    </div>

    <!-- 底部留白，防止内容被底部按钮遮挡 -->
    <div class="space-bottom"></div>

    <!-- 底部双操作按钮，页面专属、遮挡全局导航 -->
    <div class="goods-action-bar">
      <div class="btn-cart" @click="addToCart">
        {{ $t("shopDetail.cartBtn") }}
      </div>
      <div class="btn-buy" @click="buyNow">{{ $t("shopDetail.buyBtn") }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { useRoute } from "vue-router";
import { useI18n } from "@/i18n";

const { t } = useI18n();
const route = useRoute();
const loading = ref(true);
const errorText = ref("");
const detail = ref<any>({});

// 商品轮播图数组（完全沿用你稳定逻辑）
const imgArr = computed(() => {
  const list: string[] = [];
  if (detail.value.img1) list.push(detail.value.img1);
  if (detail.value.img2) list.push(detail.value.img2);
  if (detail.value.img3) list.push(detail.value.img3);
  return list;
});

// 商品详情展示图数组
const detailImgs = computed(() => {
  const list: string[] = [];
  if (detail.value.d1) list.push(detail.value.d1);
  if (detail.value.d2) list.push(detail.value.d2);
  if (detail.value.d3) list.push(detail.value.d3);
  if (detail.value.d4) list.push(detail.value.d4);
  return list;
});

// 加载商品详情接口（已跑通，未做任何修改）
const loadData = async () => {
  loading.value = true;
  errorText.value = "";
  try {
    const idStr = route.query.id as string;
    const id = parseInt(idStr);
    if (Number.isNaN(id) || id < 1) {
      errorText.value = t("shopDetail.idError");
      return;
    }

    const res = await fetch("/api/v1/shop/detail", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ id }),
    });

    const json = await res.json();
    if (json.code === 0) {
      detail.value = json.data;
    } else {
      errorText.value = json.msg || t("shopDetail.fetchFail");
    }
  } catch (err) {
    console.error("请求异常", err);
    errorText.value = t("shopDetail.networkErr");
  } finally {
    loading.value = false;
  }
};

// 通用POST请求封装
const httpPost = async (url: string, data: any) => {
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), 10000);
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
  const str = localStorage.getItem("pi_user");
  if (!str) return null;
  return JSON.parse(str);
};

// 加入购物车接口 /api/v1/cart/add
const addToCart = async () => {
  const user = getUserInfo();
  if (!user) {
    alert(t("shopDetail.needLogin"));
    return;
  }
  if (!detail.value?.id) return;
  try {
    await httpPost("/api/v1/cart/add", {
      uid: user.uid,
      goods_id: detail.value.id,
      goods_spec: detail.value.goods_spec,
      num: 1,
    });
    alert(t("shopDetail.cartSuccess"));
  } catch (err: any) {
    alert(t("shopDetail.cartFail") + err.message);
  }
};

// 立即购买下单接口 /api/v1/order/buy-now
const buyNow = async () => {
  const user = getUserInfo();
  if (!user) {
    alert(t("shopDetail.needLogin"));
    return;
  }
  if (!detail.value?.id) return;
  try {
    const res = await httpPost("/api/v1/order/buy-now", {
      uid: user.uid,
      goods_id: detail.value.id,
      goods_spec: detail.value.goods_spec,
      num: 1,
    });
    alert(
      t("shopDetail.orderSuccess") +
        res.data.order_no +
        t("shopDetail.orderAmount") +
        res.data.total_price +
        " π",
    );
  } catch (err: any) {
    alert(t("shopDetail.orderFail") + err.message);
  }
};

onMounted(() => {
  loadData();
});
</script>

<style scoped>
.detail-page {
  background: #f5f5f5;
  min-height: 100vh;
  font-family:
    -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue",
    Arial, sans-serif;
}

.page-header {
  display: flex;
  align-items: center;
  padding: 16px;
  background: #fff;
  border-bottom: 1px solid #eee;
  font-size: 16px;
  font-weight: bold;
  position: sticky;
  top: 0;
  z-index: 99;
}
.back-btn {
  color: #0066cc;
  margin-right: 10px;
  cursor: pointer;
}

.tip {
  padding: 60px 20px;
  text-align: center;
  font-size: 16px;
  color: #666;
}
.tip.error {
  color: #e53e3e;
}

/* 自动轮播动画 */
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
.banner-item {
  width: 33.333%;
  height: 100%;
  flex-shrink: 0;
}
.banner-item img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  display: block;
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

/* 卡片通用样式 */
.info-card,
.desc-card {
  background: #fff;
  margin: 12px;
  border-radius: 12px;
  padding: 12px;
}
.goods-name {
  font-size: 18px;
  font-weight: bold;
  color: #111;
  margin: 0;
}
.price {
  font-size: 22px;
  color: #e64340;
  font-weight: bold;
  margin: 12px 0;
}
.row {
  font-size: 13px;
  color: #999;
  margin-top: 6px;
  line-height: 1.6;
}
.card-title {
  font-size: 16px;
  font-weight: bold;
  margin: 0 0 10px;
}
.desc-text {
  font-size: 14px;
  color: #333;
  line-height: 1.7;
  margin: 0;
}
.img-block img {
  width: 100%;
  margin-top: 8px;
  display: block;
}

/* 内容底部留白，避免被按钮遮挡 */
.space-bottom {
  height: 65px;
}

/* 底部双按钮栏，贴屏幕最底部，无导航遮挡 */
.goods-action-bar {
  position: fixed;
  left: 0;
  bottom: 0;
  width: 100%;
  height: 60px;
  background: #fff;
  border-top: 1px solid #eee;
  display: flex;
  align-items: center;
  padding: 0 15px;
  box-sizing: border-box;
  gap: 10px;
  z-index: 98;
}
.btn-cart {
  flex: 1;
  height: 40px;
  line-height: 40px;
  background: #ffb800;
  color: #fff;
  text-align: center;
  border-radius: 20px;
  font-size: 14px;
  cursor: pointer;
}
.btn-buy {
  flex: 1;
  height: 40px;
  line-height: 40px;
  background: #e64340;
  color: #fff;
  text-align: center;
  border-radius: 20px;
  font-size: 14px;
  cursor: pointer;
}
</style>
