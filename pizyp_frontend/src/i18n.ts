import { createI18n, useI18n, type LocaleMessages } from "vue-i18n";
import zhCN from "./locales/zh-CN.json";
import enUS from "./locales/en-US.json";
import esES from "./locales/es-ES.json";
import frFR from "./locales/fr-FR.json";
import jaJP from "./locales/ja-JP.json";
import koKR from "./locales/ko-KR.json";

// 类型断言，解决 {} 无法分配给 LocaleMessage 报错
const messages: LocaleMessages<Record<string, any>> = {
  "zh-CN": zhCN as Record<string, any>,
  "en-US": enUS as Record<string, any>,
  "es-ES": esES as Record<string, any>,
  "fr-FR": frFR as Record<string, any>,
  "ja-JP": jaJP as Record<string, any>,
  "ko-KR": koKR as Record<string, any>,
};

// 读取本地存储语言，默认中文
const storageLang = localStorage.getItem("site_lang") || "zh-CN";

const i18n = createI18n({
  legacy: false,
  globalInjection: true, // 模板直接使用 $t() 无需手动引入
  locale: storageLang,
  fallbackLocale: "en-US", // 缺少翻译自动回退英文
  messages,
});

export default i18n;
export { useI18n };
