<template>
  <div class="shop-page">
    <!-- 顶部标题栏 -->
    <div class="head-title">{{ $t("shop.title") }}</div>

    <!-- 商品网格容器 -->
    <div class="goods-grid">
      <!-- 加载状态 -->
      <div v-if="loading" class="loading-box">
        <div class="loader"></div>
        <div>{{ $t("shop.loading") }}</div>
      </div>
      <!-- 加载失败 -->
      <div v-else-if="errorMsg" class="loading-box">{{ errorMsg }}</div>
      <!-- 商品列表 -->
      <template v-else>
        <div
          class="goods-card"
          v-for="item in goodsList"
          :key="item.id"
          @click="goDetail(item.id)"
        >
          <img
            :src="getImgUrl(item.img_base64)"
            alt="商品图"
            class="goods-img"
          />
          <div class="goods-info">
            <div class="goods-name">{{ item.goods_name }}</div>
            <div class="goods-spec">{{ item.goods_spec }}</div>
            <div class="goods-price">{{ item.price_pi }} π</div>
            <div class="goods-stock">
              {{ $t("shop.stock") }}{{ item.goods_stock }}
            </div>
          </div>
        </div>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";

// 路由实例
const router = useRouter();

// 商品类型定义
interface GoodsItem {
  id: number;
  goods_name: string;
  goods_spec: string;
  price_pi: string;
  goods_stock: number;
  img_base64: string | null;
}

// 状态变量
const loading = ref(true);
const errorMsg = ref("");
const goodsList = ref<GoodsItem[]>([]);

// 统一POST请求封装（对齐原JS httpPost逻辑）
async function httpPost(
  url: string,
  data: Record<string, any>,
  headers?: Record<string, string>,
) {
  const timeoutMs = 10000;
  const abortCtrl = new AbortController();
  const timer = setTimeout(() => abortCtrl.abort(), timeoutMs);

  try {
    const h: Record<string, string> = { "Content-Type": "application/json" };
    if (headers) Object.assign(h, headers);

    const res = await fetch(url, {
      method: "POST",
      headers: h,
      body: JSON.stringify(data),
      signal: abortCtrl.signal,
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
}

// 拼接图片Base64地址
const getImgUrl = (b64: string | null) => {
  if (b64) return `data:image/jpeg;base64,${b64}`;
  return "https://via.placeholder.com/400x400.png?text=商品图片";
};

// 点击跳详情
const goDetail = (goodsId: number) => {
  router.push({
    path: "/shop/detail",
    query: { id: goodsId },
  });
};

const loadShopData = async () => {
  console.log("=== 开始加载商城商品 ===");
  try {
    loading.value = true;
    errorMsg.value = "";
    const res = await httpPost("/api/v1/shop/data", {});
    // 后端真实商品数组在data层级
    goodsList.value = res.data || [];
    console.log("商品列表赋值完成，数量：", goodsList.value.length);
  } catch (err: any) {
    errorMsg.value = err.message || "加载失败，请刷新重试";
    console.error("商城加载捕获错误：", err);
  } finally {
    loading.value = false;
    console.log("=== 加载结束，loading置为false ===");
  }
};

// 页面挂载加载数据
onMounted(() => {
  loadShopData();
});
</script>

<style scoped>
.shop-page {
  background: #f5f5f5;
  min-height: calc(100vh - 60px);
  padding-bottom: 60px;
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
.goods-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
  padding: 12px;
}
.loading-box {
  text-align: center;
  padding: 40px 0;
  color: #666;
  grid-column: 1 / 3;
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
.goods-card {
  background: #fff;
  border-radius: 12px;
  padding: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  cursor: pointer;
}
.goods-img {
  width: 100%;
  border-radius: 8px;
  display: block;
}
.goods-info {
  margin-top: 10px;
}
.goods-name {
  font-size: 15px;
  font-weight: bold;
  color: #111;
  line-height: 1.4;
}
.goods-spec {
  font-size: 13px;
  color: #666;
  margin: 6px 0;
  line-height: 1.3;
}
.goods-price {
  font-size: 18px;
  color: #e64340;
  font-weight: bold;
  margin: 8px 0;
}
.goods-stock {
  font-size: 12px;
  color: #999;
}
</style>
