<template>
  <div class="game-wrap">
    <!-- 返回关闭按钮 -->
    <div class="close-btn" @click="$router.push('/contract')">×</div>

    <!-- 标题区域 -->
    <div class="page-title">
      <h1>{{ $t("zodiacGame.title") }}</h1>
      <p>{{ $t("zodiacGame.desc") }}</p>
    </div>

    <!-- 12生肖网格 -->
    <div class="zodiac-grid">
      <div
        v-for="(zod, idx) in zodiacList"
        :key="idx"
        class="zodiac-item"
        :class="{ roll: rollIndex === idx, active: selectedIdx === idx }"
        @click="selectZod(idx, zod.name)"
      >
        <div style="font-size: 24px; margin-bottom: 4px">{{ zod.emoji }}</div>
        <div style="font-size: 13px">{{ zod.name }}</div>
        <div
          :id="`num${idx}`"
          style="font-size: 11px; margin-top: 4px; color: #ffd768"
        >
          {{ betNum[idx] }} {{ $t("zodiacGame.personUnit") }}
        </div>
      </div>
    </div>

    <!-- 开奖提示框 -->
    <div class="info-box">
      <div id="needText" class="info-box-text">
        {{ $t("zodiacGame.needDrawText", { num: needPeople }) }}
      </div>
    </div>

    <!-- 投注按钮 -->
    <button
      id="betBtn"
      class="btn-bet"
      :disabled="selectedIdx === -1"
      @click="startZodiacBet"
    >
      {{
        selectedIdx === -1
          ? $t("zodiacGame.btnSelectTip")
          : $t("zodiacGame.btnBetFormat", { name: selectedName })
      }}
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { useI18n } from "@/i18n";
const { t } = useI18n();

// 生肖基础数据
const zodiacList = ref([
  { emoji: "🐭", name: "子鼠" },
  { emoji: "🐂", name: "丑牛" },
  { emoji: "🐅", name: "寅虎" },
  { emoji: "🐇", name: "卯兔" },
  { emoji: "🐉", name: "辰龙" },
  { emoji: "🐍", name: "巳蛇" },
  { emoji: "🐎", name: "午马" },
  { emoji: "🐑", name: "未羊" },
  { emoji: "🐒", name: "申猴" },
  { emoji: "🐓", name: "酉鸡" },
  { emoji: "🐕", name: "戌狗" },
  { emoji: "🐖", name: "亥猪" },
]);
// 状态变量
const selectedName = ref("");
const selectedIdx = ref(-1);
const rollIndex = ref(0);
const betNum = ref<number[]>([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
const needPeople = ref(100);
let rollTimer: number | null = null;
let dataTimer: number | null = null;

// 统一20s超时POST请求（全局规范：res.code===0，取res.data）
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

// 滚动动画逻辑
const startRollAnimate = () => {
  rollIndex.value = (rollIndex.value + 1) % 12;
};

// 选中生肖
const selectZod = (idx: number, name: string) => {
  if (rollTimer) clearInterval(rollTimer);
  selectedIdx.value = idx;
  selectedName.value = name;
};

// 投注弹窗逻辑
const startZodiacBet = () => {
  if (selectedIdx.value === -1) return alert(t("zodiacGame.alertSelectZod"));
  alert(t("zodiacGame.alertCloseTip"));
};

// 加载后端投注数据（接口升级v1）
const loadBackendData = async () => {
  try {
    const json = await httpPost("/api/v1/contract/zodiac-bets", null);
    const data = json.data;
    betNum.value = data.bets;
    needPeople.value = data.need;
  } catch (e) {
    console.log("生肖数据加载失败", e);
  }
};

// 生命周期挂载、销毁定时器
onMounted(async () => {
  await loadBackendData();
  rollTimer = window.setInterval(startRollAnimate, 1000);
  dataTimer = window.setInterval(loadBackendData, 30000);
});
onUnmounted(() => {
  if (rollTimer) clearInterval(rollTimer);
  if (dataTimer) clearInterval(dataTimer);
});
</script>

<style scoped>
:deep(body) {
  padding-bottom: 0 !important;
}
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
  font-family: system-ui, "PingFang SC", "Microsoft YaHei";
}
.game-wrap {
  width: 100%;
  min-height: 100vh;
  padding: 20px;
  position: relative;
  background: linear-gradient(135deg, #0f1935, #1a2b5a);
  color: #fff;
}
.close-btn {
  position: absolute;
  top: 20px;
  right: 20px;
  width: 44px;
  height: 44px;
  border-radius: 50%;
  background: rgba(255, 86, 86, 0.25);
  border: 2px solid #ff5656;
  color: #ff5656;
  font-size: 22px;
  font-weight: bold;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  z-index: 999;
}
.page-title {
  text-align: center;
  margin-bottom: 20px;
  margin-top: 20px;
}
.page-title h1 {
  font-size: 24px;
  font-weight: bold;
  background: linear-gradient(90deg, #ffd768, #ffaf38);
  background-clip: text;
  color: transparent;
}
.page-title p {
  font-size: 13px;
  color: #b0c4de;
  margin-top: 4px;
  line-height: 1.5;
}
.zodiac-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 10px;
  margin-bottom: 24px;
}
.zodiac-item {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 12px;
  padding: 14px 6px;
  text-align: center;
  cursor: pointer;
  transition: all 0.2s;
}
.zodiac-item.roll {
  background: rgba(255, 215, 104, 0.15);
  border: 2px solid #ffd768;
  animation: rollLight 0.5s infinite alternate;
}
.zodiac-item.active {
  background: rgba(255, 86, 86, 0.15);
  border: 2px solid #ff5656;
  transform: scale(1.02);
  animation: none !important;
}
@keyframes rollLight {
  0% {
    opacity: 1;
    transform: scale(1);
  }
  100% {
    opacity: 0.85;
    transform: scale(1.03);
  }
}
.info-box {
  background: rgba(255, 255, 255, 0.06);
  border-radius: 12px;
  padding: 12px;
  text-align: center;
  margin-bottom: 20px;
}
.info-box-text {
  font-size: 20px;
  font-weight: bold;
  color: #ffd768;
}
.btn-bet {
  background: linear-gradient(90deg, #ffd768, #ffaf38);
  color: #111;
  border: none;
  border-radius: 14px;
  padding: 16px;
  font-size: 16px;
  font-weight: bold;
  width: 100%;
  box-shadow: 0 6px 20px rgba(255, 175, 56, 0.3);
  cursor: pointer;
}
.btn-bet:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  box-shadow: none;
}
</style>
