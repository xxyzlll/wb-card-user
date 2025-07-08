use reqwest::{Client, header::HeaderMap, Error};
use serde_json::Value;

#[tauri::command]
async fn fetch_fans(
    cookie: String,    // 前端传入的 Cookie
    page: u32,         // 分页页码
    uid: &str,         // 目标用户 ID
) -> Result<Value, String> {
    // 1. 构建请求头
    let mut headers = HeaderMap::new();
    headers.insert("Cookie", cookie.parse().unwrap());
    headers.insert("Referer", "https://weibo.com".parse().unwrap());

    // 2. 配置客户端（支持代理/超时）
    let client = Client::builder()
        .default_headers(headers)
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    // 3. 构造请求 URL
    let url = format!(
        "https://weibo.com/ajax/friendships/friends?relate=fans&page={}&uid={}&type=fans",
        page, uid
    );

    // 4. 发送请求并解析 JSON
    let response = client.get(&url).send().await?;
    if !response.status().is_success() {
        return Err(format!("请求失败: {}", response.status()));
    }

    let data: Value = response.json().await?;
    Ok(data) // 返回 JSON 数据给前端
}