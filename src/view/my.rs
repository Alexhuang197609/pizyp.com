use crate::model::user::User;

/// 渲染个人中心页面（纯静态壳，异步加载）
pub async fn render_my_page() -> String {
    format!(
        r#"
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width,initial-scale=1.0">
    <title>个人中心</title>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/font-awesome@4.7.0/css/font-awesome.min.css">
    <style>
        body {{
            margin:0;
            padding:0;
            padding-bottom:70px;
            background:#f5f5f5;
            font-family:-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        }}
        .header-bar {{
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
            gap:10px;
        }}
        .bottom-nav {{
            position:fixed;
            left:0;
            bottom:0;
            width:100%;
            height:60px;
            background:#fff;
            border-top:1px solid #eee;
            display:flex;
            justify-content:space-around;
            align-items:center;
        }}
        .bottom-nav a {{
            display:flex;
            flex-direction:column;
            align-items:center;
            color:#666;
            text-decoration:none;
            font-size:13px;
            gap:4px;
        }}
        .bottom-nav a.active {{ color:#0066cc; }}
        .bottom-nav i {{ font-size:20px; }}
    </style>
    <script src="https://sdk.minepi.com/pi-sdk.js"></script>
</head>
<body>
    <div class="header-bar">
        <i class="fa fa-user-o"></i>
        <span>用户中心</span>
    </div>

    <!-- 异步渲染区域 -->
    <div id="user-container"></div>

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
        <a href="/my" class="active" id="my-link">
            <i class="fa fa-user-o"></i>
            <span>我的</span>
        </a>
    </div>

<script>
// ====================== 统一请求封装（100% 无 Rust 报错） ======================
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

Pi.init({{ version: "2.0", sandbox: true }});

// 登录
async function piLogin() {{
    let waitCount = 0;
    while(typeof Pi === 'undefined' && waitCount < 20) {{
        await new Promise(resolve => setTimeout(resolve, 100));
        waitCount++;
    }}

    if (typeof Pi==='undefined'){{
        alert('请确保在pi浏览器环境！');
        return;
    }}

    try {{
        const auth = await Pi.authenticate(['payments', 'username']);
        localStorage.setItem('pi_user', JSON.stringify({{
            username: auth.user.username,
            uid: auth.user.uid,
            accessToken: auth.accessToken
        }}));
        alert("登录成功！");
        window.location.reload();
    }} catch (err) {{
        alert("登录失败：" + err.message);
    }}
}}

// 加载我的页面数据
async function loadMyData() {{
    const userStr = localStorage.getItem('pi_user');
    if (!userStr) {{
        document.getElementById('user-container').innerHTML = `
<div style="background:#fff; padding:30px; margin:12px; border-radius:12px; text-align:center; box-shadow:0 2px 8px rgba(0,0,0,0.05);">
    <div style="width:64px; height:64px; border-radius:50%; background:#eee; display:flex; align-items:center; justify-content:center; font-size:24px; color:#666; margin:0 auto 15px;">
        <i class="fa fa-user-o"></i>
    </div>
    <div style="font-size:16px; color:#666; margin-bottom:15px;">请登录后查看个人中心</div>
    <button onclick="piLogin()" style="background:#0066cc; color:#fff; border:none; padding:10px 20px; border-radius:8px; font-size:15px; cursor:pointer;">
        Pi 授权登录
    </button>
</div>`;
        return;
    }}

    try {{
        const user = JSON.parse(userStr);
        const json = await httpPost("/api/my/data?uid=" + user.uid, {{}});

        if (!json.user) {{
            localStorage.removeItem('pi_user');
            loadMyData();
            return;
        }}

        const u = json.user;
        let avatarHtml = '';
        if (u.avatar_base64) {{
            avatarHtml = `<img src="data:image/jpeg;base64,${{u.avatar_base64}}" style="width:64px; height:64px; border-radius:50%; object-fit:cover; border:2px solid #eee;">`;
        }} else {{
            avatarHtml = `<div style="width:64px; height:64px; border-radius:50%; background:#eee; display:flex; align-items:center; justify-content:center; font-size:24px; color:#666;"><i class="fa fa-user-o"></i></div>`;
        }}

        const html = `
<div style="background:#fff; padding:20px; margin:12px; border-radius:12px; display:flex; align-items:center; gap:15px; box-shadow:0 2px 8px rgba(0,0,0,0.05);">
    ${{avatarHtml}}
    <div>
        <div style="font-size:18px; font-weight:bold; color:#111;">${{u.nickname || u.username || '未设置昵称'}}</div>
        <div style="font-size:14px; color:#666; margin-top:4px;">
            我的 UPI: <span style="color:#0066cc; font-weight:bold;">${{u.upi_num}}</span>
        </div>
    </div>
</div>

<div style="background:#fff; margin:12px; border-radius:12px; overflow:hidden; box-shadow:0 2px 8px rgba(0,0,0,0.05);">
    <div style="padding:16px; display:flex; justify-content:space-between; align-items:center; border-bottom:1px solid #eee;">
        <div style="display:flex; align-items:center; gap:10px;">
            <i class="fa fa-exchange" style="color:#0066cc; font-size:18px;"></i>
            <span style="font-size:15px;" onclick="alert('请留意小程序即将发布的迁移方案！')">UPI迁移</span>
        </div>
        <i class="fa fa-angle-right" style="color:#999;"></i>
    </div>
    <div style="padding:16px; display:flex; justify-content:space-between; align-items:center; border-bottom:1px solid #eee; cursor:pointer;" onclick="window.location.href='/cart?uid=${{u.upi_num}}'">
        <div style="display:flex; align-items:center; gap:10px;">
            <i class="fa fa-shopping-cart" style="color:#0066cc; font-size:18px;"></i>
            <span style="font-size:15px;">我的购物车</span>
        </div>
        <i class="fa fa-angle-right" style="color:#999;"></i>
    </div>
    <div style="padding:16px; display:flex; justify-content:space-between; align-items:center; cursor:pointer;" onclick="window.location.href='/order'">
        <div style="display:flex; align-items:center; gap:10px;">
            <i class="fa fa-file-text-o" style="color:#0066cc; font-size:18px;"></i>
            <span style="font-size:15px;">我的订单</span>
        </div>
        <i class="fa fa-angle-right" style="color:#999;"></i>
    </div>
</div>`;

        document.getElementById('user-container').innerHTML = html;
    }} catch (e) {{
        console.error("加载个人信息失败", e);
    }}
}}

document.addEventListener('DOMContentLoaded', async function() {{
    await loadMyData();

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