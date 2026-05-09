pub mod home;
pub mod shop;
pub mod contract;
pub mod my;
pub mod cart;
pub mod order;
pub mod order_detail;

// 给控制器调用的快捷导出
pub use home::render_index_html;
pub use shop::render_shop_page;
pub use shop::render_goods_detail_page;
pub use contract::render_contract_page; // <-- 新增这行
