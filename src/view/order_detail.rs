//src/view/order_detail.rs
use axum::response::Html;

pub fn order_detail_page() -> Html<String> {
    Html(format!(r#"
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width,initial-scale=1.0">
    <title>订单详情</title>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/font-awesome@4.7.0/css/font-awesome.min.css">
    <script src="https://sdk.minepi.com/pi-sdk.js"></script>
    <style>
        *{{margin:0;padding:0;box-sizing:border-box;}}
        body {{
            font-family:-apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,sans-serif;
            background:#f5f5f5;
            padding-bottom:70px;
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
        .back-btn {{font-size:20px;cursor:pointer;}}

        /* 订单头部信息 */
        .info-card {{
            background:#fff;
            margin:10px;
            border-radius:12px;
            padding:16px;
        }}
        .info-row {{
            display:flex;
            justify-content:space-between;
            padding:8px 0;
            font-size:14px;
        }}
        .info-label {{color:#666;}}
        .info-value {{color:#333;}}
        .status-value {{color:#e64340;font-weight:bold;}}

        /* 商品列表 */
        .goods-card {{
            background:#fff;
            margin:10px;
            border-radius:12px;
            padding:16px;
        }}
        .goods-title {{
            font-size:15px;
            font-weight:bold;
            margin-bottom:12px;
        }}
        .goods-item {{
            display:flex;
            justify-content:space-between;
            padding:10px 0;
            border-bottom:1px solid #f5f5f5;
        }}
        .goods-item:last-child {{border-bottom:none;}}
        .goods-left {{
            flex:1;
        }}
        .goods-name {{font-size:14px;color:#333;}}
        .goods-spec {{font-size:12px;color:#999;margin-top:4px;}}
        .goods-right {{
            text-align:right;
        }}
        .goods-price {{font-size:14px;color:#e64340;}}
        .goods-num {{font-size:12px;color:#999;margin-top:4px;}}

        /* 底部金额 */
        .total-card {{
            background:#fff;
            margin:10px;
            border-radius:12px;
            padding:16px;
        }}
        .total-row {{
            display:flex;
            justify-content:space-between;
            padding:6px 0;
            font-size:14px;
        }}
        .total-final {{
            display:flex;
            justify-content:space-between;
            padding-top:10px;
            margin-top:10px;
            border-top:1px solid #eee;
            font-size:16px;
            font-weight:bold;
        }}
        .total-price {{color:#e64340;}}

        /* 支付按钮样式 */
        .pay-btn-wrap {{
            margin:15px 10px;
            text-align:right;
        }}
        .pay-btn {{
            background:#e64340;
            color:#fff;
            border:none;
            padding:10px 22px;
            border-radius:8px;
            font-size:15px;
            cursor:pointer;
        }}
        .pay-btn:disabled {{
            background:#cccccc;
            cursor:not-allowed;
        }}

        /* 底部导航 完全和首页一致 */
        .bottom-nav {{
            position: fixed !important;
            bottom: 0 !important;
            left: 0 !important;
            width: 100% !important;
            height: 60px !important;
            background: #fff !important;
            border-top: 1px solid #eee !important;
            display: flex !important;
            align-items: center !important;
            justify-content: space-around !important;
            z-index: 999 !important;
        }}
        .bottom-nav a {{
            color: #666 !important;
            text-decoration: none !important;
            font-size: 13px !important;
            display: flex !important;
            flex-direction: column !important;
            align-items: center !important;
            gap: 4px !important;
        }}
        .bottom-nav a i {{font-size:20px !important;}}
        .bottom-nav a.active {{color:#0066cc !important;}}
    </style>
</head>
<body>
    <div class="top-bar">
        <i class="fa fa-angle-left back-btn" onclick="window.history.back()"></i>
        <span>📦 订单详情</span>
    </div>

    <!-- 订单基础信息 -->
    <div class="info-card">
        <div class="info-row">
            <span class="info-label">订单状态</span>
            <span class="status-value" id="order-status"></span>
        </div>
        <div class="info-row">
            <span class="info-label">订单编号</span>
            <span class="info-value" id="order-no"></span>
        </div>
        <div class="info-row">
            <span class="info-label">下单时间</span>
            <span class="info-value" id="create-time"></span>
        </div>
    </div>

    <!-- 商品明细 -->
    <div class="goods-card">
        <div class="goods-title">商品明细</div>
        <div id="goods-list"></div>
    </div>

    <!-- 金额汇总 -->
    <div class="total-card">
        <div class="total-row">
            <span>商品总额</span>
            <span id="goods-total"></span>
        </div>
        <div class="total-final">
            <span>实付金额</span>
            <span class="total-price" id="pay-total"></span>
        </div>
    </div>

    <!-- 支付按钮容器 -->
    <div class="pay-btn-wrap" id="pay-box">
        <button class="pay-btn" id="pay-btn">立即支付</button>
    </div>

    <!-- 底部导航 -->
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
// ====================== 统一请求封装（普通接口20秒超时，支付不用这个） ======================
async function httpPost(url, data, headers) {{
    var timeoutMs = 20000;
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

// 1. 全局初始化Pi SDK
Pi.init({{ version: "2.0", sandbox: true }});

// 2. 全局变量
let currentTotalPrice = 0;
let currentOrderNo = "";

document.addEventListener("DOMContentLoaded", async function() {{
    let currentOrderId = "";
    const urlParams = new URLSearchParams(window.location.search);
    const orderId = urlParams.get("id");
    if(!orderId){{
        alert("订单ID错误");
        window.history.back();
        return;
    }}

    const userStr = localStorage.getItem("pi_user");
    if(!userStr){{
        alert("请先登录");
        window.location.href="/my";
        return;
    }}
    const userData = JSON.parse(userStr);

    try {{
        // 订单详情接口
        const json = await httpPost("/api/order/detail", {{
            uid: userData.uid,
            order_id: orderId
        }});

        const order = json.order;
        const goodsList = json.list;

        currentOrderId = orderId;
        currentOrderNo = order.order_no;
        currentTotalPrice = order.total_price_pi;

        document.getElementById("order-status").innerText = order.status_text;
        document.getElementById("order-no").innerText = order.order_no;
        document.getElementById("create-time").innerText = order.create_time;
        document.getElementById("goods-total").innerText = order.total_price_pi + " π";

        if (order.status === 1) {{
            document.getElementById("pay-total").innerText = order.total_price_pi + " π";
        }} else {{
            document.querySelector(".total-final").style.display = "none";
        }}

        if(order.status === 0){{
            document.getElementById("pay-box").style.display = "block";
            document.getElementById("pay-btn").onclick = startPiPayment;
        }} else {{
            document.getElementById("pay-box").style.display = "none";
        }}

        let goodsHtml = "";
        goodsList.forEach(goods=>{{
            goodsHtml += `
            <div class="goods-item">
                <div class="goods-left">
                    <div class="goods-name">${{goods.goods_name}}</div>
                    <div class="goods-spec">规格：${{goods.goods_spec||"无"}}</div>
                </div>
                <div class="goods-right">
                    <div class="goods-price">${{goods.price_pi}} π</div>
                    <div class="goods-num">x${{goods.num}}</div>
                </div>
            </div>`;
        }});
        document.getElementById("goods-list").innerHTML = goodsHtml;

    }} catch(err){{
        alert("加载失败："+err.message);
    }}
}});

// ==============================================
// Pi 订单支付函数（支付回调原生fetch，无超时）
// ==============================================
async function startPiPayment() {{
    const payBtn = document.getElementById("pay-btn");
    payBtn.disabled = true;

    try {{
        const userData = JSON.parse(localStorage.getItem('pi_user'));
        if (!userData || !userData.accessToken) {{
            alert("请先登录");
            return;
        }}
        const token = userData.accessToken;
        const orderId = new URLSearchParams(window.location.search).get("id");

        const auth = await Pi.authenticate(['payments', 'username'], function onIncompletePaymentFound(payment) {{
            console.log('未完成支付:', payment);
        }});

        await Pi.createPayment({{
            amount: currentTotalPrice,
            memo: "订单支付：" + currentOrderNo,
            metadata: {{
                type: "order",
                order_id: orderId
            }}
        }}, {{
            // 原生fetch，无超时，适配钱包密钥输入
            onReadyForServerApproval: async (paymentId) => {{
                try {{
                    const res = await fetch(`/api/pi/payments/${{paymentId}}/approve`, {{
                        method:"POST",
                        headers:{{
                            "Authorization":"Bearer "+token,
                            "Content-Type":"application/json"
                        }},
                        body:JSON.stringify({{order_id:orderId}})
                    }});
                    if(!res.ok) throw new Error("approve接口异常");
                }} catch (e) {{
                    console.error(e);
                }}
            }},

            onReadyForServerCompletion: async (paymentId, txid) => {{
                try {{
                    const res = await fetch(`/api/pi/payments/${{paymentId}}/complete`, {{
                        method:"POST",
                        headers:{{
                            "Authorization":"Bearer "+token,
                            "Content-Type":"application/json"
                        }},
                        body:JSON.stringify({{order_id:orderId,txid:txid}})
                    }});
                    if(!res.ok) throw new Error("complete接口异常");

                    alert("✅ 支付成功！订单已确认");
                    location.reload();
                }} catch (e) {{
                    console.error(e);
                    alert("支付异常：" + e.message);
                }}
            }},

            onCancel: () => alert("已取消支付"),
            onError: (err) => alert("支付错误：" + err.message)
        }});

    }} catch (e) {{
        alert("支付启动失败：" + e.message);
    }} finally {{
        payBtn.disabled = false;
    }}
}}

</script>
</body>
</html>
    "#))
}