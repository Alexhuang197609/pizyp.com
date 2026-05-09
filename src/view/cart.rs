/// 渲染购物车页面HTML
pub fn render_cart_page() -> String {
    format!(r#"
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width,initial-scale=1.0">
    <title>购物车</title>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/font-awesome@4.7.0/css/font-awesome.min.css">
    <style>
        *{{
            margin:0;
            padding:0;
            box-sizing:border-box;
        }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            background:#f5f5f5;
            padding-bottom:120px;
        }}
        .top-bar {{
            padding:16px;
            background:#fff;
            font-size:18px;
            font-weight:bold;
            border-bottom:1px solid #eee;
            position:sticky;
            top:0;
            z-index:99;
            display:flex;
            align-items:center;
            gap:12px;
        }}
        .back-btn {{
            font-size:20px;
            cursor:pointer;
        }}
        .cart-empty {{
            text-align:center;
            padding:60px 20px;
            color:#999;
        }}
        .cart-item {{
            background:#fff;
            margin:10px;
            border-radius:12px;
            padding:12px;
            display:flex;
            gap:12px;
            align-items:center;
        }}
        .item-info {{
            flex:1;
        }}
        .item-name {{
            font-size:15px;
            font-weight:bold;
            color:#222;
        }}
        .item-spec {{
            font-size:12px;
            color:#999;
            margin:4px 0;
        }}
        .item-price {{
            font-size:16px;
            color:#e64340;
            font-weight:bold;
        }}
        .num-box {{
            display:flex;
            align-items:center;
            gap:8px;
        }}
        .num-btn {{
            width:28px;
            height:28px;
            border-radius:50%;
            background:#eee;
            display:flex;
            align-items:center;
            justify-content:center;
            font-size:16px;
            cursor:pointer;
            user-select:none;
        }}
        .num-text {{
            width:30px;
            text-align:center;
            font-size:15px;
        }}

        .cart-bottom-bar {{
            position:fixed;
            left:0;
            bottom:60px;
            width:100%;
            background:#fff;
            padding:12px 16px;
            border-top:1px solid #eee;
            display:flex;
            justify-content:space-between;
            align-items:center;
        }}
        .total-text {{
            font-size:16px;
            font-weight:bold;
            color:#e64340;
        }}
        .pay-btn {{
            background:#0066cc;
            color:#fff;
            border:none;
            padding:8px 18px;
            border-radius:20px;
            font-size:14px;
            cursor:pointer;
        }}

        .bottom-nav {{
            position: fixed;
            bottom: 0;
            left: 0;
            width: 100%;
            height: 60px;
            background: #ffffff;
            border-top: 1px solid #eee;
            display: flex;
            align-items: center;
            justify-content: space-around;
            z-index: 999;
        }}
        .bottom-nav a {{
            color: #666;
            text-decoration: none;
            font-size: 13px;
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 4px;
        }}
        .bottom-nav a i {{
            font-size: 20px;
        }}
        .bottom-nav a.active {{
            color: #0066cc;
        }}
    </style>
</head>
<body>

    <div class="top-bar">
        <i class="fa fa-angle-left back-btn" onclick="window.history.back()"></i>
        <span>🛒 我的购物车</span>
    </div>

    <div id="cart-list"></div>

    <div class="cart-bottom-bar">
        <div class="total-text">合计：<span id="total-price">0.00</span> π</div>
        <button class="pay-btn" id="settle-btn">去结算</button>
    </div>

    <div class="bottom-nav">
        <a href="/">
            <i class="fa fa-users"></i>
            <span>优社群</span>
        </a>
        <a href="/shop">
            <i class="fa fa-shopping-bag"></i>
            <span>优商城</span>
        </a>
        <a href="/contract">
            <i class="fa fa-file-text-o"></i>
            <span>合约</span>
        </a>
        <a href="/my" class="active">
            <i class="fa fa-user-o"></i>
            <span>我的</span>
        </a>
    </div>

<script>
let userData = null;
let totalPrice = 0;

async function loadCart() {{
    const userStr = localStorage.getItem('pi_user');
    if (!userStr) {{
        document.getElementById('cart-list').innerHTML = '<div class="cart-empty">请先登录</div>';
        document.getElementById('total-price').innerText = '0.00';
        return;
    }}
    userData = JSON.parse(userStr);

    const res = await fetch("/api/cart/list", {{
        method: "POST",
        headers: {{ "Content-Type": "application/json" }},
        body: JSON.stringify({{ uid: userData.uid }})
    }});
    const json = await res.json();

    if (json.code !== 0 || json.data.length === 0) {{
        document.getElementById('cart-list').innerHTML = '<div class="cart-empty">购物车空空如也~</div>';
        document.getElementById('total-price').innerText = '0.00';
        return;
    }}

    let html = '';
    totalPrice = 0;
    json.data.forEach(item => {{
        let subtotal = item.price_pi * item.num;
        totalPrice += subtotal;
        html += `
        <div class="cart-item" data-cart-id="${{item.id}}">
            <div class="item-info">
                <div class="item-name">${{item.goods_name}}</div>
                <div class="item-spec">${{item.goods_spec || '无规格'}}</div>
                <div class="item-price">${{item.price_pi}} π</div>
            </div>
            <div class="num-box">
                <div class="num-btn minus" data-id="${{item.id}}">-</div>
                <div class="num-text">${{item.num}}</div>
                <div class="num-btn plus" data-id="${{item.id}}">+</div>
            </div>
        </div>`;
    }});

    document.getElementById('cart-list').innerHTML = html;
    document.getElementById('total-price').innerText = totalPrice.toFixed(2);
    bindNumButtons();
}}

function bindNumButtons() {{
    document.querySelectorAll('.plus').forEach(btn => {{
        btn.onclick = async () => {{
            const cartId = btn.dataset.id;
            await updateNum(cartId, 1);
        }};
    }});

    document.querySelectorAll('.minus').forEach(btn => {{
        btn.onclick = async () => {{
            const cartId = btn.dataset.id;
            await updateNum(cartId, -1);
        }};
    }});
}}

async function updateNum(cartId, step) {{
    const res = await fetch("/api/cart/update-num", {{
        method: "POST",
        headers: {{ "Content-Type": "application/json" }},
        body: JSON.stringify({{ cart_id: parseInt(cartId), step: step }})
    }});
    const result = await res.json();
    if (result.code === 0) {{
        loadCart();
    }}
}}

document.addEventListener('DOMContentLoaded', loadCart);

document.getElementById('settle-btn').onclick = async function() {{
    if (!userData || !userData.uid) {{
        alert('请先登录');
        return;
    }}

    // 拿到所有勾选的购物车ID
    const cartItems = document.querySelectorAll('.cart-item');
    if (cartItems.length === 0) {{
        alert('购物车为空，无法结算');
        return;
    }}

    let cartIds = [];
    cartItems.forEach(item => {{
        const cartId = item.getAttribute('data-cart-id');
        cartIds.push(parseInt(cartId));
    }});

    // 调用后端结算接口
    try {{
        const res = await fetch("/api/order/settle", {{
            method: "POST",
            headers: {{ "Content-Type": "application/json" }},
            body: JSON.stringify({{
                uid: userData.uid,
                cart_ids: cartIds
            }})
        }});

        const data = await res.json();
        if (data.code === 0) {{
            alert("✅ 订单创建成功！\\n订单号：" + data.order_no + "\\n总金额：" + data.total_price + " π");
            // 刷新购物车
            loadCart();
        }} else {{
            alert("❌ 订单创建失败：" + (data.msg || "未知错误"));
        }}
    }} catch (err) {{
        alert("请求失败：" + err.message);
        console.error(err);
    }}
}};
</script>

</body>
</html>
    "#)
}