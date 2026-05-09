/// 智能合约页面渲染
pub fn render_contract_page() -> String {
    r#"
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width,initial-scale=1.0">
    <title>智能合约</title>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/font-awesome@4.7.0/css/font-awesome.min.css">
    <style>
        body { 
            margin:0; 
            padding:0; 
            padding-bottom:70px; 
            background:#f5f5f5; 
            font-family:-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
        }
        /* 跟微商城首页抬头样式完全一致 */
        .header-bar { 
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
        }
        .header-bar i {
            font-size:20px;
            color:#0066cc;
        }
        .content-card { 
            background:#fff; 
            margin:12px; 
            border-radius:12px; 
            padding:16px; 
            box-shadow:0 2px 8px rgba(0,0,0,0.05);
        }
        .content-card p {
            margin:0 0 10px 0;
            line-height:1.6;
            color:#333;
        }
        .img-wrap {
            background:#fff;
            margin:12px;
            border-radius:12px;
            padding:12px;
            box-shadow:0 2px 8px rgba(0,0,0,0.05);
            text-align:center;
        }
        .img-wrap img {
            max-width:100%;
            border-radius:8px;
            display:block;
            margin:0 auto;
        }
        .bottom-nav {
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
        }
        .bottom-nav a {
            display:flex; 
            flex-direction:column; 
            align-items:center;
            color:#666; 
            text-decoration:none; 
            font-size:13px;
            gap:4px;
        }
        .bottom-nav a.active { color:#0066cc; }
        .bottom-nav i { font-size:20px; }
    </style>
</head>
<body>
    <!-- 抬头：图标 + 智能合约 和微商城同风格 -->
    <div class="header-bar">
        <i class="fa fa-file-text-o"></i>
        <span>智能合约</span>
    </div>

    <div class="content-card">
        <p>派之优品即将正式上线测试网应用生态名录，并登陆 Pi LaunchPad 首发 testUPI。项目将持续跟进 Protocol23 版本动态，适时部署生态专属智能合约。</p>
        <p>在此期间，您可前往微信搜索「派之优品」，或直接扫码进入 2.0 小程序生态，抢先参与 PiRC1 早期项目，获取专属生态贡献积分。</p>
    </div>

    <div class="img-wrap">
        <img src="/static/qrcode.jpg" alt="合约二维码">
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
        <a href="/contract" class="active">
            <i class="fa fa-file-text-o"></i>
            <span>合约</span>
        </a>
        <!-- 已加 id="my-link" -->
        <a href="/my" id="my-link">
            <i class="fa fa-user-o"></i>
            <span>我的</span>
        </a>
    </div>

<script>
// 统一逻辑：自动给我的链接带上 uid
document.addEventListener('DOMContentLoaded', function() {
    const userStr = localStorage.getItem('pi_user');
    if (userStr) {
        const user = JSON.parse(userStr);
        const myLink = document.getElementById('my-link');
        if (myLink && user.uid) {
            myLink.href = `/my?uid=${user.uid}`;
        }
    }
});
</script>

</body>
</html>
    "#.to_string()
}