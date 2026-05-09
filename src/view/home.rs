//src/view/home.rs
use crate::model::action::UserAction;

/// 渲染首页HTML视图（纯静态壳，数据由JS异步加载）
pub async fn render_index_html() -> String {
    format!(r#"
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width,initial-scale=1.0">
    <title>pizyp.com派之优品</title>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/font-awesome@4.7.0/css/font-awesome.min.css">
    

<script src="https://sdk.minepi.com/pi-sdk.js"></script>

    <style>
        *{{
            margin:0;
            padding:0;
            box-sizing:border-box;
        }}
        body {{
            font-family: Arial, sans-serif;
            padding-bottom: 70px;
        }}
        .container {{
            background: transparent;
            box-shadow: none;
        }}
/* 轮播样式：高度150px 宽度自适应 */
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
        h5 {{
            color: #333;
            margin:5px 10px 5px;
            font-size:14px;
            text-align: right;
            white-space: nowrap;
        }}

/* 底部导航栏 优化样式 */
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

/* 朋友圈动态样式 */
.post-list {{
    padding: 0 10px;
}}
.post-item {{
    background: #fff;
    border-bottom: 1px solid #f0f0f0;
    padding: 12px 0;
    display: flex;
    gap: 10px;
}}
.post-avatar {{
    width: 48px;
    height: 48px;
    border-radius: 50%;
    object-fit: cover;
    flex-shrink: 0;
}}
.post-content {{
    flex: 1;
}}
.post-nick {{
    font-size: 15px;
    font-weight: bold;
    color: #333;
}}
.post-time {{
    font-size: 12px;
    color: #999;
    margin-left: 8px;
}}
.post-text {{
    font-size: 14px;
    color: #333;
    line-height: 1.5;
    margin: 6px 0;
}}
.post-img {{
    max-width: 180px;
    border-radius: 6px;
    margin: 4px 0;
}}
.post-video {{
    max-width: 220px;
    border-radius: 6px;
    margin: 4px 0;
}}
.post-actions {{
    font-size: 12px;
    color: #999;
    margin-top: 6px;
    display: flex;
    gap: 16px;
}}

/* ====== 新增：加载动画样式 ====== */
.loading-box {{
    text-align:center;
    padding:40px 0;
    color:#666;
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
    <div class="container">
        <h5 id="welcome-text">🎉游客🎉请登录：<a onclick="piLogin()" style="margin-left:15px;color:#0066cc;text-decoration:none;cursor:pointer;">pi授权登录</a ></h5>
    </div>
    <div class="banner">
        <div class="banner-wrap">
            <img src="/static/banner1.jpg" alt="轮播1">
            <img src="/static/banner2.jpg" alt="轮播2">
            <img src="/static/banner3.jpg" alt="轮播3">
        </div>
    </div>

<!-- ====================== 用户动态（朋友圈）容器，JS异步渲染 ====================== -->
<div class="post-list" id="post-list">
    <!-- 加载动画 -->
    <div class="loading-box">
        <div class="loader"></div>
        <div>优社群数据加载中...</div>
    </div>
</div>
<!-- ================================================================= -->

<!-- 底部导航 带图标 -->
<div class="bottom-nav">
    <a href="/" class="active">
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
    <a href="/my" id="my-link">
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

Pi.init({{ version: "2.0", sandbox: true }});

async function piLogin() {{
    try {{
        localStorage.removeItem('pi_user');
        const auth = await Pi.authenticate(['payments', 'username'], function onIncompletePaymentFound(payment) {{
            console.log('未完成支付:', payment);
        }});

        const json = await httpPost("/api/pi/verify", null, {{
            "Authorization": "Bearer " + auth.accessToken
        }});

        localStorage.setItem('pi_user', JSON.stringify({{
            username: auth.user.username,
            uid: auth.user.uid,
            accessToken: auth.accessToken
        }}));

        alert('授权成功！刷新页面');
        window.location.href = window.location.href;
    }} catch (err) {{
        alert('调用piLogin授权失败: ' + err.message);
        console.error(err);
    }}
}}

async function createPiPayment(){{
    let paying = false;
    if (paying) return;
    paying = true;

    try {{
        const userData = JSON.parse(localStorage.getItem('pi_user'));
        if (!userData || !userData.accessToken) {{
            alert('请先登录');
            paying = false;
            return;
        }}

        const auth = await Pi.authenticate(['payments', 'username'], function onIncompletePaymentFound(payment) {{
            console.log('未完成支付:', payment);
        }});

        await Pi.createPayment({{
            amount: 0.01,
            memo: "测试打赏",
            metadata: {{
		        type: "tip",
                order_id: "test_" + Date.now()
            }}
        }}, {{
            // 支付回调：原生fetch，**无超时**，适配钱包密钥输入
            onReadyForServerApproval: async (paymentId) => {{
                try {{
                    const res = await fetch(`/api/pi/payments/${{paymentId}}/approve`, {{
                        method: "POST",
                        headers: {{
                            "Authorization": "Bearer " + userData.accessToken,
                            "Content-Type": "application/json"
                        }},
                        body: JSON.stringify({{order_id: "test_" + Date.now()}})
                    }});
                    if (!res.ok) {{
                        alert("approve 失败 状态码:" + res.status);
                    }}
                }} catch (e) {{
                    alert("approve 请求异常:" + e.message);
                }}
            }},
            onReadyForServerCompletion: async (paymentId, txid) => {{
                try {{
                    const res = await fetch(`/api/pi/payments/${{paymentId}}/complete`, {{
                        method: "POST",
                        headers: {{
                            "Authorization": "Bearer " + userData.accessToken,
                            "Content-Type": "application/json"
                        }},
                        body: JSON.stringify({{order_id: "test_" + Date.now(), txid: txid}})
                    }});
                    if (!res.ok) {{
                        alert("complete 失败 状态码:" + res.status);
                    }} else {{
                        alert("✅ 支付流程完成");
                    }}
                }} catch (e) {{
                    alert("complete 请求异常:" + e.message);
                }}
            }},
            onCancel: () => {{ alert("已取消"); paying = false; }},
            onError: (e) => {{ alert("支付错误: " + e.message); paying = false; }}
        }});

    }} catch (err) {{
        alert("支付失败: " + err.message);
    }} finally {{
        paying = false;
    }}
}}

// 加载首页朋友圈数据
async function loadHomeData() {{
    try {{
        const json = await httpPost("/api/home/data", null);
        const list = json.list;
        let html = "";
        list.forEach(item=>{{
            let imgHtml = item.img_base64 ? `<img class="post-img" src="data:image/jpeg;base64,${{item.img_base64}}">` : "";
            let videoHtml = item.video_base64 ? `
<video class="post-video" controls>
    <source src="data:video/mp4;base64,${{item.video_base64}}" type="video/mp4">
    您的浏览器不支持视频播放
</video>` : "";

            html += `
<div class="post-item">
    <img class="post-avatar" src="data:image/jpeg;base64,${{item.face_base64}}">
    <div class="post-content">
        <span class="post-nick">${{item.nick_name}}</span>
        <span class="post-time">${{item.time}}</span>
        <div class="post-text">${{item.text}}</div>
        ${{imgHtml}}
        ${{videoHtml}}
        <div class="post-actions">
            <span>👍 点赞 ${{item.prize_num}}</span>
            <span>💬 评论 ${{item.comments_num}}</span>
        </div>
    </div>
</div>`;
        }});
        document.getElementById("post-list").innerHTML = html;
    }} catch(e){{
        document.getElementById("post-list").innerHTML = `<div style="text-align:center;padding:40px 0;color:#999;">加载失败，请刷新重试</div>`;
        console.error("加载首页动态失败",e);
    }}
}}

document.addEventListener('DOMContentLoaded', async function() {{
    await loadHomeData();

    const userStr = localStorage.getItem('pi_user');
    if (userStr) {{
        const user = JSON.parse(userStr);
        const h5 = document.getElementById('welcome-text');
        h5.innerHTML = `🎉 ${{user.username}} 🎉<a onclick="createPiPayment()" style="margin-left:15px;color:#0066cc;text-decoration:none;cursor:pointer;">打赏0.01TestPi</a >`;

        const myLink = document.getElementById('my-link');
        if(myLink && user.uid){{
           myLink.href = `/my?uid=${{user.uid}}`;
        }}
    }}
}});
    </script>
</body>
</html>
    "#)
}