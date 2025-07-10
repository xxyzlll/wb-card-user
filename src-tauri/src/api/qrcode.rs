use reqwest::{Client, header::{HeaderMap, HeaderValue}};
use serde_json::Value;

#[tauri::command]
pub async fn get_login_qrcode(size: Option<u32>) -> Result<Value, String> {
    let mut headers = HeaderMap::new();
    headers.insert("Referer", 
        HeaderValue::from_static("https://passport.weibo.com/sso/signin?entry=miniblog&source=miniblog&disp=popup&url=https%3A%2F%2Fweibo.com%2Fnewlogin%3Ftabtype%3Dweibo%26gid%3D102803%26openLoginLayer%3D0%26url%3Dhttps%253A%252F%252Fweibo.com%252F&from=weibopro")
    );
    headers.insert("Accept", 
        HeaderValue::from_static("application/json, text/plain, */*")
    );
    headers.insert("User-Agent", 
        HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36")
    );
    headers.insert("sec-ch-ua", 
        HeaderValue::from_static("\"Not)A;Brand\";v=\"8\", \"Chromium\";v=\"138\", \"Google Chrome\";v=\"138\"")
    );
    headers.insert("sec-ch-ua-platform", 
        HeaderValue::from_static("\"macOS\"")
    );
    headers.insert("sec-fetch-dest", 
        HeaderValue::from_static("empty")
    );
    headers.insert("sec-fetch-mode", 
        HeaderValue::from_static("cors")
    );
    headers.insert("sec-fetch-site", 
        HeaderValue::from_static("same-origin")
    );
    headers.insert("x-requested-with", 
        HeaderValue::from_static("XMLHttpRequest")
    );

    let client = Client::builder()
        .default_headers(headers)
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("客户端创建失败: {}", e))?;

    // 使用用户提供的size参数，默认为180
    let size_value = size.unwrap_or(180);
    let url = format!(
        "https://passport.weibo.com/sso/v2/qrcode/image?entry=miniblog&size={}",
        size_value
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