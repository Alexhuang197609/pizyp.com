//src/view/order.rs
use axum::response::Html;

pub fn order_list_page() -> Html<String> {
    Html(format!(r#"
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width,initial-scale=1.0">
    <title>我的订单</title>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/font-awesome@4.7.0/css/font-awesome.min.css">
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
        .order-empty {{text-align:center;padding:60px 20px;color:#999;}}
        .order-item {{background:#fff;margin:10px;border-radius:12px;padding:14px;}}
        .order-head {{display:flex;justify-content:space-between;align-items:center;margin-bottom:10px;}}
        .order-no {{font-size:14px;color:#333;font-weight:bold;}}
        .order-status {{font-size:14px;color:#e64340;}}
        .order-time {{font-size:12px;color:#999;margin-bottom:10px;}}
        .order-total {{display:flex;justify-content:space-between;align-items:center;padding-top:10px;border-top:1px solid #eee;}}
        .total-text {{font-size:16px;font-weight:bold;color:#e64340;}}
        .del-btn {{background:#ff4d4f;color:#fff;border:none;padding:6px 14px;border-radius:20px;font-size:13px;cursor:pointer;}}
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
        <span>📦 我的订单</span>
    </div>
    <div id="order-list"></div>
    <div class="bottom-nav">
        <a href="/"><i class="fa fa-users"></i><span>优社群</span></a>
        <a href="/shop"><i class="fa fa-shopping-bag"></i><span>优商城</span></a>
	    <a href="/contract"><i class="fa fa-file-text-o"></i><span>合约</span></a>
        <a href="/my" class="active"><i class="fa fa-user-o"></i><span>我的</span></a>
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

document.addEventListener("DOMContentLoaded", async function() {{
    const userStr = localStorage.getItem("pi_user");
    if(!userStr){{
        document.getElementById("order-list").innerHTML="<div class=\"order-empty\">请先登录</div>";
        return;
    }}
    const userData = JSON.parse(userStr);
    if(!userData.uid || userData.uid.trim() === ""){{
        document.getElementById("order-list").innerHTML="<div class=\"order-empty\">暂无订单~</div>";
        return;
    }}
    try {{
        const json = await httpPost("/api/order/list", {{
            uid: userData.uid
        }});

        if(json.list.length===0){{
            document.getElementById("order-list").innerHTML="<div class=\"order-empty\">暂无订单~</div>";
            return;
        }}
        let html = "";
        json.list.forEach(item=>{{
            let delHtml = "";
            if (item.status === 0) {{
                delHtml = "<button class=\"del-btn\" onclick=\"delOrder('" + item.id + "')\">删除订单</button>";
            }}
            html += "<div class=\"order-item\" style=\"cursor:pointer;\" onclick=\"location.href='/order/detail?id="+item.id+"'\">" +
                "<div class=\"order-head\">" +
                    "<div class=\"order-no\">订单号：" + item.order_no + "</div>" +
                    "<div class=\"order-status\">" + item.status_text + "</div>" +
                "</div>" +
                "<div class=\"order-time\">" + item.create_time + "</div>" +
                "<div class=\"order-total\">" +
                    "<div class=\"total-text\">合计：" + item.total_price_pi + " π</div>" +
                    delHtml +
                "</div>" +
            "</div>";
        }});
        document.getElementById("order-list").innerHTML = html;
    }} catch (err) {{
        document.getElementById("order-list").innerHTML="<div class=\"order-empty\">加载订单失败</div>";
    }}
}});

async function delOrder(orderId){{
    if(!confirm("确定删除该待支付订单？")) {{
        return;
    }}

    try {{
        const userStr = localStorage.getItem("pi_user");
        if (!userStr) {{
            alert("请先登录");
            return;
        }}
        const userData = JSON.parse(userStr);

        await httpPost("/api/order/delete", {{
            uid: userData.uid,
            order_id: orderId
        }});

        alert("删除成功");
        location.reload();
    }} catch (e) {{
        alert("删除失败：" + e.message);
    }}
}}

</script>
</body>
</html>
    "#))
}