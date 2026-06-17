<template>
  <div class="browser-page">
    <!-- 顶部紫色导航 -->
    <div class="top-nav">
      <i class="fa fa-arrow-left back-btn" @click="goBackContract"></i>
      <h1><i class="fa fa-chain"></i> SUPI Block Explorer</h1>
      <div class="nav-icon-group">
        <i class="fa fa-qrcode"></i>
        <i class="fa fa-bars"></i>
      </div>
    </div>

    <!-- 容器区域 -->
    <div id="browser-container">
      <div v-if="loading" class="loading-box">
        <div class="loader"></div>
        <div>{{ $t("browser.loading") }}</div>
      </div>
      <div v-else-if="errorText" class="loading-box error">{{ errorText }}</div>
      <div v-else>
        <!-- 合约基础信息卡片 -->
        <div class="contract-info">
          <h2>{{ contract.name }} {{ $t("browser.tokenSuffix") }}</h2>
          <p>
            <strong>{{ $t("browser.addrLabel") }}</strong
            >{{ contract.contract_addr }}
          </p>
          <p>
            <strong>{{ $t("browser.descLabel") }}</strong
            >{{ contract.desc }}
          </p>
          <p>
            <strong>{{ $t("browser.statusLabel") }}</strong
            >{{ contract.status }}
          </p>
        </div>

        <!-- 列表表头 -->
        <div class="table-header">
          <span>{{ $t("browser.colAccount") }}</span>
          <span>{{ $t("browser.colOperation") }}</span>
          <span>{{ $t("browser.colLedgerTime") }}</span>
        </div>

        <!-- 交易列表 -->
        <div
          class="tx-item"
          v-for="tx in tx_list"
          :key="tx.tx_hash"
          @click="goTxDetail(tx.tx_hash)"
        >
          <div class="addr-tag" @click.stop="goAddressTx(tx.from_addr)">
            {{ shortAddr(tx.from_addr) }}
          </div>
          <div class="tx-op">
            <div v-if="tx.event_type === 'transfer'">
              Pay {{ Number(tx.amount) / 1000000 }} {{ tx.token_type }} to
              <span class="to-addr" @click.stop="goAddressTx(tx.to_addr)">
                {{ tx.to_addr }}
              </span>
            </div>
            <div v-else>{{ tx.event_type }}_claimable_balance</div>
          </div>
          <div class="tx-time">
            {{ tx.ledger }}
            <br />
            {{ formatBJTime(tx.tx_time) }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";

const route = useRoute();
const router = useRouter();
const loading = ref(true);
const errorText = ref("");
const contract = ref<any>({});
const tx_list = ref<any[]>([]);

// 统一20s超时POST请求（全局规范：code===0，取data）
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

// 地址截断 前4后4
const shortAddr = (addr: string) => {
  if (!addr) return "-";
  return addr.slice(0, 4) + "..." + addr.slice(-4);
};

// 北京时间格式化（无手动时区偏移）
const formatBJTime = (rawTimeStr: string) => {
  const targetDate = new Date(rawTimeStr);
  const pad = (n: number) => String(n).padStart(2, "0");
  const year = targetDate.getFullYear();
  const month = pad(targetDate.getMonth() + 1);
  const day = pad(targetDate.getDate());
  const hour = pad(targetDate.getHours());
  const minute = pad(targetDate.getMinutes());
  const second = pad(targetDate.getSeconds());
  return `${year}-${month}-${day}\n${hour}:${minute}:${second}`;
};
const goBackContract = () => {
  router.push("/contract");
};
// 跳转方法
const goTxDetail = (hash: string) => {
  router.push(`/tx-detail?hash=${hash}`);
};
const goAddressTx = (addr: string) => {
  router.push(`/address-txs?address=${addr}`);
};

// 加载区块浏览器数据
const loadBrowserData = async () => {
  loading.value = true;
  errorText.value = "";
  try {
    const addr = route.query.addr as string;
    if (!addr) {
      errorText.value = "无效合约地址";
      return;
    }
    // 接口升级v1
    const json = await httpPost("/api/v1/browser/overview", {
      contract_addr: addr,
    });
    const data = json.data;
    contract.value = data.contract;
    tx_list.value = data.tx_list || [];
  } catch (e: any) {
    console.error("页面加载异常：", e);
    errorText.value = e.message || "加载失败，请刷新重试";
  } finally {
    loading.value = false;
  }
};

onMounted(() => loadBrowserData());
</script>

<style scoped>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}
.browser-page {
  font-family:
    -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  background: #ffffff;
  min-height: 100vh;
  padding-bottom: 70px;
}
/* 顶部紫色渐变导航栏 */
.top-nav {
  background: linear-gradient(135deg, #7928ca, #5a189a);
  color: #fff;
  padding: 18px 16px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.top-nav .back-btn {
  font-size: 22px;
  cursor: pointer;
  margin-right: 10px;
}
.top-nav h1 {
  font-size: 20px;
  font-weight: 600;
}
.nav-icon-group i {
  font-size: 22px;
  margin-left: 16px;
  cursor: pointer;
}
/* 合约信息卡片 */
.contract-info {
  margin: 16px;
  padding: 16px;
  background: #fff;
  border-radius: 12px;
  box-shadow: 0 2px 10px rgba(121, 40, 202, 0.1);
}
.contract-info h2 {
  color: #222;
  font-size: 18px;
  margin-bottom: 10px;
}
.contract-info p {
  font-size: 14px;
  color: #444;
  line-height: 1.6;
  margin: 6px 0;
  word-break: break-all;
}
/* 列表表头三栏布局 加宽Account列 */
.table-header {
  display: grid;
  grid-template-columns: 110px 1fr 110px;
  padding: 12px 16px;
  border-bottom: 1px solid #eee;
  color: #666;
  font-size: 15px;
  font-weight: 500;
  gap: 12px;
}
/* 单条交易条目 */
.tx-item {
  display: grid;
  grid-template-columns: 110px 1fr 110px;
  padding: 8px 10px;
  border-bottom: 1px solid #f5f5f5;
  align-items: center;
  gap: 12px;
  cursor: pointer;
}
/* 左侧紫色圆形头像标签 */
.addr-tag {
  background: #7b2cbf;
  color: #fff;
  width: 98px;
  height: 28px;
  border-radius: 6px;
  font-size: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  padding: 0 6px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  cursor: pointer;
}
/* 中间操作文本 缩小字号 */
.tx-op {
  font-size: 13px;
  color: #222;
  line-height: 1.5;
  word-break: break-all;
}
.tx-op .to-addr {
  color: #5a189a;
  font-weight: 500;
  display: block;
  margin-top: 4px;
  cursor: pointer;
}
/* 右侧时间：小字号 + 自动换行 */
.tx-time {
  font-size: 12px;
  color: #777;
  text-align: right;
  white-space: pre-line;
  flex-shrink: 0;
}
/* 加载动画 */
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
