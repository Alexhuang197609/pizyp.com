// 导入路由核心方法
import { createRouter, createWebHistory } from "vue-router";
// 导入页面
import Index from "@/views/Index.vue";
import Shop from "@/views/Shop.vue";
import Contract from "@/views/Contract.vue";
import My from "@/views/My.vue";
import ShopDetail from "@/views/ShopDetail.vue";
import ZodiacGame from "@/views/ZodiacGame.vue";
import Browser from "@/views/Browser.vue";
import AddressTxs from "@/views/AddressTxs.vue";
import TxDetail from "@/views/TxDetail.vue";
import Cart from "@/views/Cart.vue";
import Order from "@/views/Order.vue";
import OrderDetail from "@/views/OrderDetail.vue";
import About from "@/views/About.vue";
// 路由列表：路径对应页面
const routes = [
  {
    // 首页 访问域名直接打开
    path: "/",
    name: "Index",
    component: Index,
  },
  {
    // 商城页面
    path: "/shop",
    name: "Shop",
    component: Shop,
  },
  {
    path: "/shop/detail",
    name: "ShopDetail",
    component: ShopDetail,
  },
  {
    // 合约浏览器页面
    path: "/contract",
    name: "Contract",
    component: Contract,
  },
  {
    // 合约浏览器页面
    path: "/my",
    name: "My",
    component: My,
  },
  {
    path: "/zodiac-game",
    name: "ZodiacGame",
    component: ZodiacGame,
  },
  {
    path: "/browser",
    name: "Browser",
    component: Browser,
  },
  {
    path: "/address-txs",
    name: "AddressTxs",
    component: AddressTxs,
  },
  {
    path: "/tx-detail",
    name: "TxDetail",
    component: TxDetail,
  },
  {
    path: "/shop/cart",
    name: "ShopCart",
    component: Cart,
  },
  {
    path: "/shop/order",
    name: "ShopOrder",
    component: Order,
  },
  {
    path: "/order/detail",
    name: "OrderDetail",
    component: OrderDetail,
  },
  {
    path: "/about",
    name: "About",
    component: About,
  },
];

// 创建路由实例
const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
});

// 导出路由，给main.ts使用
export default router;
