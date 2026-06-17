import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
// 新增i18n引入
import i18n from "./i18n";

const app = createApp(App);
app.use(router);
// 全局注册国际化
app.use(i18n);

// 预留Pi SDK初始化位置，后面写完piSdk工具再补这里
app.mount("#app");
