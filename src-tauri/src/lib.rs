// src-tauri/src/main.rs
use reqwest::{Client, header::{HeaderMap, HeaderValue}};
use serde_json::Value;

#[tauri::command]
async fn fetch_fans(
    cookie: String,
    page: u32,
    uid: String,  // 关键修改点
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

#[tauri::command]
async fn send_direct_message(
    cookie: String,
    text: String,
    uid: String,
    source: String,
) -> Result<Value, String> {
    println!("开始执行send_direct_message，参数：uid={}, text长度={}", uid, text.len());
    
    // 1. 构建请求头 - 修改为与正常请求一致的头信息
    let mut headers = HeaderMap::new();
    headers.insert("Cookie", 
        HeaderValue::from_str(&cookie).map_err(|e| {
            let err_msg = format!("非法Cookie格式: {}", e);
            println!("错误: {}", err_msg);
            err_msg
        })?
    );
    // 修改Referer为正确的值
    headers.insert("Referer", 
        HeaderValue::from_static("https://api.weibo.com/chat/")
    );
    // 修改Content-Type为正确的值
    headers.insert("Content-Type", 
        HeaderValue::from_static("application/x-www-form-urlencoded")
    );
    // 添加其他必要的请求头
    headers.insert("User-Agent", 
        HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36")
    );
    headers.insert("Accept", 
        HeaderValue::from_static("application/json, text/plain, */*")
    );
    headers.insert("Origin", 
        HeaderValue::from_static("https://api.weibo.com")
    );
    println!("请求头设置完成");

    // 2. 配置客户端
    let client = Client::builder()
        .default_headers(headers.clone())
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| {
            let err_msg = format!("客户端创建失败: {}", e);
            println!("错误: {}", err_msg);
            err_msg
        })?;
    println!("HTTP客户端创建完成");

    // 3. 第一步：调用handshake接口获取clientId
    let handshake_url = "https://web.im.weibo.com/im/handshake";
    println!("准备调用handshake接口: {}", handshake_url);
    
    let handshake_payload = serde_json::json!([
        {
            "id": "1",
            "version": "1.0",
            "minimumVersion": "1.0",
            "channel": "/meta/handshake",
            "supportedConnectionTypes": [
                "long-polling",
                "callback-polling"
            ],
            "advice": {
                "timeout": 60000,
                "interval": 0
            }
        }
    ]);
    println!("handshake请求数据: {}", handshake_payload);

    let handshake_response = client.post(handshake_url)
        .json(&handshake_payload)
        .send()
        .await
        .map_err(|e| {
            let err_msg = format!("握手请求失败: {}", e);
            println!("错误: {}", err_msg);
            err_msg
        })?;

    println!("handshake响应状态码: {}", handshake_response.status());
    if !handshake_response.status().is_success() {
        let status = handshake_response.status();
        let err_msg = format!("握手HTTP错误: {} {}", status, status.canonical_reason().unwrap_or(""));
        println!("错误: {}", err_msg);
        return Err(err_msg);
    }

    let handshake_body = handshake_response.text().await
        .map_err(|e| {
            let err_msg = format!("读取握手响应内容失败: {}", e);
            println!("错误: {}", err_msg);
            err_msg
        })?;
    println!("handshake响应内容: {}", handshake_body);
    
    let handshake_data: Value = serde_json::from_str(&handshake_body)
        .map_err(|e| {
            let err_msg = format!("握手响应解析失败: {}", e);
            println!("错误: {}", err_msg);
            err_msg
        })?;

    // 从响应中提取clientId
    let client_id = handshake_data[0]["clientId"].as_str()
        .ok_or_else(|| {
            let err_msg = "未能获取到clientId".to_string();
            println!("错误: {}", err_msg);
            err_msg
        })?;
    println!("成功获取clientId: {}", client_id);

    // 4. 第二步：使用clientId发送私信
    let dm_url = "https://api.weibo.com/webim/2/direct_messages/new.json";
    println!("准备调用发送私信接口: {}", dm_url);

    // 构建表单数据
    let mut form = std::collections::HashMap::new();
    form.insert("text", text.clone());
    form.insert("uid", uid.clone());
    form.insert("extensions", format!("{{\"clientid\":\"{}\"}}", client_id));
    form.insert("is_encoded", "0".to_string());
    form.insert("decodetime", "1".to_string());
    form.insert("source", source.clone());
    println!("发送私信表单数据: {:?}", form);

    // 打印完整请求信息用于调试
    println!("发送私信完整请求信息:");
    println!("URL: {}", dm_url);
    println!("方法: POST");
    println!("Content-Type: {}", headers.get("Content-Type").unwrap().to_str().unwrap_or("未知"));
    println!("Referer: {}", headers.get("Referer").unwrap().to_str().unwrap_or("未知"));
    println!("Cookie长度: {}", cookie.len());
    
    let dm_response = client.post(dm_url)
        .form(&form) // 使用form方法发送表单数据
        .send()
        .await
        .map_err(|e| {
            let err_msg = format!("发送私信请求失败: {}", e);
            println!("错误: {}", err_msg);
            err_msg
        })?;

    println!("发送私信响应状态码: {}", dm_response.status());
    if !dm_response.status().is_success() {
        let status = dm_response.status();
        let err_msg = format!("发送私信HTTP错误: {} {}", status, status.canonical_reason().unwrap_or(""));
        println!("错误: {}", err_msg);
        return Err(err_msg);
    }

    let dm_body = dm_response.text().await
        .map_err(|e| {
            let err_msg = format!("读取发送私信响应内容失败: {}", e);
            println!("错误: {}", err_msg);
            err_msg
        })?;
    println!("发送私信响应内容: {}", dm_body);
    
    let result: Value = serde_json::from_str(&dm_body)
        .map_err(|e| {
            let err_msg = format!("发送私信响应解析失败: {}", e);
            println!("错误: {}", err_msg);
            err_msg
        })?;
    
    println!("发送私信成功完成");
    Ok(result)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![fetch_fans, send_direct_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}