use reqwest::{Client, header::{HeaderMap, HeaderValue}};
use serde_json::Value;

#[tauri::command]
pub async fn fetch_fans(
    cookie: String,
    page: u32,
    uid: String,
) -> Result<Value, String> {
    let mut headers = HeaderMap::new();
    headers.insert("Cookie", 
        HeaderValue::from_str(&cookie).map_err(|_| "非法Cookie格式")?  
    );
    headers.insert("Referer", 
        HeaderValue::from_static("https://weibo.com")
    );

    let client = Client::builder()
        .default_headers(headers)
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("客户端创建失败: {}", e))?;

    let url = format!(
        "https://weibo.com/ajax/friendships/friends?relate=fans&page={}&uid={}&type=fans",
        page, uid
    );

    let response = client.get(&url)
        .send()
        .await
        .map_err(|e| format!("网络请求失败: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        return Err(format!("HTTP错误: {} {}", status, status.canonical_reason().unwrap_or(""))); 
    }

    response.json::<Value>().await
        .map_err(|e| format!("JSON解析失败: {}", e))
}