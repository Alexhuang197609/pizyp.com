# pizyp.com PiZhiYouPin派之优品
## 📌 Project Introduction 
A fully functional open-source e-commerce DApp exclusively built for the Pi Network ecosystem.
The project has completed full-stack architecture upgrade:
- Backend refactored to RESTful `/api/v1` standard based on Rust + Axum + SQLite;
- Frontend fully migrated to **Vue3 + Vite + TypeScript** following Pi official development specifications, completely separated from backend rendering;
- Multi-language internationalization (6 languages: Chinese, English, Spanish, French, Japanese, Korean) fully implemented to serve global Pi users;
- Global website acceleration enabled via Cloudflare CDN to drastically reduce cross-border access latency for overseas visitors.

It covers full mall workflows: product browsing, shopping cart, order management, native Pi SDK payment authorization, personal account system, and is fully deployed and running on the Pi Network Testnet.

### 中文介绍
本项目是专为 Pi Network 生态打造的全功能开源电商DApp，现已完成全套全栈架构重大升级：
- 后端重构为 Rust+Axum+SQLite 标准 RESTful `/api/v1` 接口体系；
- 前端彻底脱离旧版服务端模板渲染，全面迁移至**Vue3 + Vite + TypeScript**，严格遵循 Pi App Studio 官方开发规范，实现标准前后端分离；
- 完成六国语言国际化适配（中、英、西、法、日、韩），覆盖全球Pi生态用户；
- 接入 Cloudflare 全球CDN加速，大幅降低海外用户跨境访问延迟，页面加载性能显著提升。

项目完整覆盖商品浏览、购物车、订单管理、Pi官方SDK支付授权、个人中心全链路商城能力，已稳定部署运行于 Pi Network 测试网。

---
## 🛠 Tech Stack 技术栈
### Backend 后端
- Language & Framework: Rust, Axum
- ORM: SQLx
- Database: SQLite
- API Standard: RESTful API v1

### Frontend 前端
- Core Framework: Vue3 + Vite + TypeScript
- I18n: Multi-language support (CN/EN/ES/FR/JA/KO)
- Deployment: Apache reverse proxy + Cloudflare Global CDN

### Blockchain 链上能力
- Pi Network SDK v2.0 (Testnet)
- Soroban Smart Contract (Protocol 23 compatible)

---
## ✨ Core Features 核心功能
1. Global Multi-language Adaptation
   六国语言国际化切换，适配全球各地Pi用户访问
2. Standard Frontend-Backend Separation Architecture
   前端Vue3+TS规范开发，后端RESTful标准化接口，代码分层清晰合规
3. Complete E-commerce Basic Workflow
   商品分类展示、购物车、立即下单、完整订单状态管理（待支付/已完成）
4. Native Pi Official Payment Integration
   原生接入Pi SDK支付授权，安全对接Pi生态支付体系
5. User Account & Wallet Binding System
   用户信息管理、钱包地址映射绑定，打通小程序与DApp用户数据
6. Global CDN Acceleration
   Cloudflare全球边缘节点缓存静态资源，海外访问速度大幅优化
7. Lightweight Stable Backend Service
   Rust高性能后端，低资源占用，高并发承载海外用户访问

---
## 🚧 Follow-up Plan 后续开发规划
After Pi Protocol 23 officially launches, we will deploy our self-developed original game smart contract **Zodiac Guess Game (生肖猜猜乐)** on the testnet, enriching the ecological entertainment scene of PiZhiYouPin.
待 Pi Protocol 23 正式上线后，将部署自研原创链游智能合约「生肖猜猜乐」，拓展派之优品生态娱乐场景。

---
## 📄 Open Source License 开源协议
This project is open-sourced under the MIT License. You are free to use, learn, modify and perform secondary development without commercial restrictions.
本项目基于 MIT 开源协议，可免费学习、修改、二次开发，无商用限制。

---
## 🔗 Official Links 官方链接
- Official Website: https://pizyp.com
- GitHub Repository: https://github.com/Alexhuang197609/pizyp.com
