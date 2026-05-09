//src/view/shop.rs
use crate::model::goods::Goods;
use base64::engine::{general_purpose, Engine as _};

/// 商城页面渲染（纯静态壳，JS异步加载）
pub async fn render_shop_page() -> String {
    format!(
        r#"
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width,initial-scale=1.0">
    <title>优商城</title>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/font-awesome@4.7.0/css/font-awesome.min.css">
    <style>
        body {{ margin:0; padding:0; padding-bottom:70px; background:#f5f5f5; font-family:-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif; }}
        .head-title {{ padding:16px; background:#fff; font-size:18px; font-weight:bold; border-bottom:1px solid #eee; position:sticky; top:0; z-index:99; }}
        .goods-grid {{ display:grid; grid-template-columns: 1fr 1fr; gap:12px; padding:12px; }}
        .bottom-nav {{
            position:fixed; left:0; bottom:0; width:100%; height:60px;
            background:#fff; border-top:1px solid #eee;
            display:flex; justify-content:space-around; align-items:center;
        }}
        .bottom-nav a {{
            display:flex; flex-direction:column; align-items:center;
            color:#666; text-decoration:none; font-size:13px;
            gap:4px;
        }}
        .bottom-nav a.active {{ color:#0066cc; }}
        .bottom-nav i {{ font-size:20px; }}

        /* 加载动画样式 */
        .loading-box {{
            text-align:center;
            padding:40px 0;
            color:#666;
            grid-column: 1 / 3;
        }}
        .loader {{
            width:40px;
            height:40px;
            border:3px solid #eee;
            border-top:3px solid #666;
            border-radius:50%;
            animation: spin 1s linear infinite;
            margin:0 auto 10px;
        }}
        @keyframes spin {{
            0% {{ transform: rotate(0deg); }}
            100% {{ transform: rotate(360deg); }}
        }}
    </style>
</head>
<body>
    <div class="head-title">🏪 优商城</div>
    
    <!-- 商品容器：JS 异步渲染 -->
    <div class="goods-grid" id="goods-list">
        <!-- 加载动画 -->
        <div class="loading-box">
            <div class="loader"></div>
            <div>商品数据加载中...</div>
        </div>
    </div>

    <div class="bottom-nav">
        <a href="/">
            <i class="fa fa-users"></i>
            <span>优社群</span>
        </a>
        <a href="/shop" class="active">
            <i class="fa fa-shopping-bag"></i>
            <span>优商城</span>
        </a>
        <a href="/contract">
            <i class="fa fa-file-text-o"></i>
            <span>合约</span>
        </a>
        <a href="/my" id="my-link">
            <i class="fa fa-user-o"></i>
            <span>我的</span>
        </a>
    </div>

<script>
// ====================== 统一请求封装 ======================
async function httpPost(url, data, headers) {{
    var timeoutMs = 10000;
    var abortCtrl = new AbortController();
    var timer = setTimeout(function() {{
        abortCtrl.abort();
    }}, timeoutMs);

    try {{
        var h = {{ "Content-Type": "application/json" }};
        if (headers) {{
            for (var k in headers) {{
                h[k] = headers[k];
            }}
        }}

        var res = await fetch(url, {{
            method: "POST",
            headers: h,
            body: JSON.stringify(data),
            signal: abortCtrl.signal
        }});

        clearTimeout(timer);
        var json = await res.json();

        if (json.code !== 0) {{
            throw new Error(json.msg || "请求失败");
        }}
        return json;
    }} catch (e) {{
        clearTimeout(timer);
        var msg = e.message || "网络异常";
        if (e.name === "AbortError") {{
            msg = "请求超时，请检查网络";
        }}
        throw new Error(msg);
    }}
}}

// 加载商品列表
async function loadShopData() {{
    try {{
        const json = await httpPost("/api/shop/data", {{}});

        let html = "";
        json.list.forEach(item => {{
            let imgSrc = item.img_base64 
                ? "data:image/jpeg;base64," + item.img_base64 
                : "https://via.placeholder.com/400x400.png?text=商品图片";

            html += `
<a href="/goods/${{item.id}}" style="text-decoration:none;">
<div style="background:#fff; border-radius:12px; padding:12px; box-shadow:0 2px 8px rgba(0,0,0,0.05);">
    <img src="${{imgSrc}}" style="width:100%; border-radius:8px; display:block;">
    <div style="margin-top:10px;">
        <div style="font-size:15px; font-weight:bold; color:#111; line-height:1.4;">${{item.goods_name}}</div>
        <div style="font-size:13px; color:#666; margin:6px 0; line-height:1.3;">${{item.goods_spec}}</div>
        <div style="font-size:18px; color:#e64340; font-weight:bold; margin:8px 0;">${{item.price_pi}} π</div>
        <div style="font-size:12px; color:#999;">库存：${{item.goods_stock}}</div>
    </div>
</div>
</a>`;
        }});

        document.getElementById("goods-list").innerHTML = html;
    }} catch (e) {{
        document.getElementById("goods-list").innerHTML = `<div class="loading-box">加载失败，请刷新重试</div>`;
        console.error("加载商品失败", e);
    }}
}}

document.addEventListener('DOMContentLoaded', async function() {{
    // 异步加载商品
    await loadShopData();

    const userStr = localStorage.getItem('pi_user');
    if (userStr) {{
        const user = JSON.parse(userStr);
        const myLink = document.getElementById('my-link');
        if (myLink && user.uid) {{
            myLink.href = `/my?uid=${{user.uid}}`;
        }}
    }}
}});
</script>

</body>
</html>
    "#
    )
}

/// 商品详情页渲染（不动，保持原样）
pub fn render_goods_detail_page(item: &Goods) -> String {
    let mut carousel_imgs = Vec::new();

    if let Some(b) = &item.goods_img1_blob {
        carousel_imgs.push(general_purpose::STANDARD.encode(b));
    }
    if let Some(b) = &item.goods_img2_blob {
        carousel_imgs.push(general_purpose::STANDARD.encode(b));
    }
    if let Some(b) = &item.goods_img3_blob {
        carousel_imgs.push(general_purpose::STANDARD.encode(b));
    }

    let mut carousel_html = String::new();
    for b64 in carousel_imgs {
        carousel_html.push_str(&format!(
            r#"<img src="data:image/jpeg;base64,{}">"#,
            b64
        ));
    }
    if carousel_html.is_empty() {
        carousel_html = r#"<img src="https://via.placeholder.com/400x240.png?text=暂无商品图片">"#.to_string();
    }

let mut detail_img_html = String::new();

// 商品详情图 1-4，逐个渲染
if let Some(blob) = &item.goods_detail1_blob {
    let b64 = general_purpose::STANDARD.encode(blob);
    detail_img_html.push_str(&format!(
        r#"<img src="data:image/jpeg;base64,{}" style="width:100%;display:block;margin:8px 0;border-radius:8px;">"#,
        b64
    ));
}
if let Some(blob) = &item.goods_detail2_blob {
    let b64 = general_purpose::STANDARD.encode(blob);
    detail_img_html.push_str(&format!(
        r#"<img src="data:image/jpeg;base64,{}" style="width:100%;display:block;margin:8px 0;border-radius:8px;">"#,
        b64
    ));
}
if let Some(blob) = &item.goods_detail3_blob {
    let b64 = general_purpose::STANDARD.encode(blob);
    detail_img_html.push_str(&format!(
        r#"<img src="data:image/jpeg;base64,{}" style="width:100%;display:block;margin:8px 0;border-radius:8px;">"#,
        b64
    ));
}
if let Some(blob) = &item.goods_detail4_blob {
    let b64 = general_purpose::STANDARD.encode(blob);
    detail_img_html.push_str(&format!(
        r#"<img src="data:image/jpeg;base64,{}" style="width:100%;display:block;margin:8px 0;border-radius:8px;">"#,
        b64
    ));
}

    let name = &item.goods_name;
    let spec = item.goods_spec.as_deref().unwrap_or("暂无规格");
    let desc = item.goods_desc.as_deref().unwrap_or("暂无商品简介");
    let ship_addr = item.goods_ship_addr.as_deref().unwrap_or("官方统一发货");
    let price = item.price_pi;
    let stock = item.goods_stock;
    let sales = item.goods_sales;

    format!(
        r#"
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width,initial-scale=1.0">
    <title>{name} - 商品详情</title>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/font-awesome@4.7.0/css/font-awesome.min.css">
    <style>
        body {{ margin:0; padding:0; padding-bottom:120px; background:#f5f5f5; font-family:-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif; }}
        .head-back {{ padding:16px; background:#fff; font-size:16px; font-weight:bold; border-bottom:1px solid #eee; position:sticky; top:0; z-index:99; }}
        .head-back a {{ color:#0066cc; text-decoration:none; }}

        .banner {{
            width: calc(100% - 20px);
            height: 160px;
            overflow: hidden;
            position: relative;
            margin: 0 10px 15px;
        }}
        .banner-wrap {{
            display: flex;
            width: 300%;
            height: 100%;
            animation: bannerSlide 12s infinite;
        }}
        .banner-wrap img {{
            width: 33.333%;
            height: 100%;
            object-fit: cover;
            flex-shrink: 0;
        }}
        @keyframes bannerSlide {{
            0%, 30% {{ transform: translateX(0); }}
            33%, 63% {{ transform: translateX(-33.333%); }}
            66%, 96% {{ transform: translateX(-66.666%); }}
            100% {{ transform: translateX(0); }}
        }}

        .detail-card {{ background:#fff; margin:12px; border-radius:12px; padding:12px; }}
        
        /* 底部固定操作栏 */
        .goods-action-bar {{
            position:fixed; left:0; bottom:0; width:100%; height:60px;
            background:#fff; border-top:1px solid #eee;
            display:flex; align-items:center; padding:0 15px;
            box-sizing:border-box; gap:10px;
        }}
        .btn-cart {{
            flex:1; height:40px; line-height:40px;
            background:#ffb800; color:#fff;
            text-align:center; border-radius:20px;
            font-size:14px; border:none;
        }}
        .btn-buy {{
            flex:1; height:40px; line-height:40px;
            background:#e64340; color:#fff;
            text-align:center; border-radius:20px;
            font-size:14px; border:none;
        }}

        .bottom-nav {{ display:none; }}
    </style>
</head>
<body>
    <div class="head-back">
        <a href="/shop"><i class="fa fa-arrow-left"></i> 返回商城</a>
    </div>

    <div class="banner">
        <div class="banner-wrap">
            {carousel_html}
        </div>
    </div>

    <div class="detail-card">
        <div style="font-size:18px; font-weight:bold; color:#111;">{name}</div>
        <div style="font-size:14px; color:#666; margin:8px 0;">规格：{spec}</div>
        <div style="font-size:22px; color:#e64340; font-weight:bold; margin:12px 0;">{price} π</div>
        <div style="font-size:13px; color:#999; line-height:1.6;">
            库存：{stock} &nbsp;&nbsp; 销量：{sales}<br>
            发货：{ship_addr}
        </div>
    </div>

    <div class="detail-card">
        <div style="font-size:16px; font-weight:bold; margin-bottom:10px;">商品简介</div>
        <div style="font-size:14px; color:#333; line-height:1.7;">{desc}</div>
    </div>

    <div class="detail-card">
        <div style="font-size:16px; font-weight:bold; margin-bottom:10px;">商品详情</div>
        {detail_img_html}
    </div>

    <!-- 底部固定操作按钮：加入购物车 / 立即购买 -->
    <div class="goods-action-bar">
        <div class="btn-cart" onclick="addToCart()">加入购物车</div>
        <div class="btn-buy" onclick="buyNow()">立即购买</div>
    </div>

<script>
// ====================== 统一请求封装 ======================
async function httpPost(url, data, headers) {{
    var timeoutMs = 10000;
    var abortCtrl = new AbortController();
    var timer = setTimeout(function() {{
        abortCtrl.abort();
    }}, timeoutMs);

    try {{
        var h = {{ "Content-Type": "application/json" }};
        if (headers) {{
            for (var k in headers) {{
                h[k] = headers[k];
            }}
        }}

        var res = await fetch(url, {{
            method: "POST",
            headers: h,
            body: JSON.stringify(data),
            signal: abortCtrl.signal
        }});

        clearTimeout(timer);
        var json = await res.json();

        if (json.code !== 0) {{
            throw new Error(json.msg || "请求失败");
        }}
        return json;
    }} catch (e) {{
        clearTimeout(timer);
        var msg = e.message || "网络异常";
        if (e.name === "AbortError") {{
            msg = "请求超时，请检查网络";
        }}
        throw new Error(msg);
    }}
}}

// 全局函数：加入购物车
function addToCart() {{
    const userStr = localStorage.getItem('pi_user');
    if (!userStr) {{
        alert('请先登录');
        return;
    }}
    
    const user = JSON.parse(userStr);
    httpPost("/api/cart/add", {{
        uid: user.uid,
        goods_id: {id},
        goods_spec: "{spec}",
        num: 1
    }}).then(data => {{
        alert("✅ 加入购物车成功！");
    }}).catch(err => {{
        alert("❌ 失败：" + err.message);
    }});
}}

// 立即购买
function buyNow() {{
    const userStr = localStorage.getItem('pi_user');
    if (!userStr) {{
        alert('请先登录');
        return;
    }}

    const user = JSON.parse(userStr);
    httpPost("/api/order/buy-now", {{
        uid: user.uid,
        goods_id: {id},
        goods_spec: "{spec}",
        num: 1
    }}).then(data => {{
        alert("✅ 订单创建成功！\\n订单号：" + data.order_no + "\\n金额：" + data.total_price + " π");
    }}).catch(err => {{
        alert("❌ 创建订单失败：" + err.message);
    }});
}}

// 我的链接
document.addEventListener('DOMContentLoaded', function() {{
    const userStr = localStorage.getItem('pi_user');
    if (userStr) {{
        const user = JSON.parse(userStr);
        const myLink = document.getElementById('my-link');
        if (myLink && user.uid) {{
            myLink.href = `/my?uid=${{user.uid}}`;
        }}
    }}
}});
</script>

</body>
</html>
    "#,
        id = item.id,
        name = name,
        carousel_html = carousel_html,
        spec = spec,
        desc = desc,
        ship_addr = ship_addr,
        price = price,
        stock = stock,
        sales = sales,
        detail_img_html = detail_img_html
    )
}