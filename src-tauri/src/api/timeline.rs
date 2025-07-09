use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde_json::Value;
use std::collections::HashMap;

#[tauri::command]
pub async fn fetch_user_timeline(
    cookie: String,
    uid: String,
    comment_text: String,
) -> Result<Value, String> {
    println!(
        "开始执行fetch_user_timeline，参数：uid={}, comment_text={}",
        uid, comment_text
    );

    // 1. 构建请求头
    let mut headers = HeaderMap::new();
    headers.insert(
        "Cookie",
        HeaderValue::from_str(&cookie).map_err(|e| {
            let err_msg = format!("非法Cookie格式: {}", e);
            println!("错误: {}", err_msg);
            err_msg
        })?,
    );
    headers.insert("Referer", HeaderValue::from_static("https://weibo.com"));
    headers.insert("User-Agent", 
        HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36")
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

    // 步骤1: 调用接口获取topicId
    let search_url = format!("https://weibo.com/ajax/statuses/searchProfile?uid={}&page=1&endtime=1752076800&hasori=1&hasret=1", uid);
    println!("请求URL: {}", search_url);

    let search_response = client.get(&search_url).send().await.map_err(|e| {
        let err_msg = format!("网络请求失败: {}", e);
        println!("错误: {}", err_msg);
        err_msg
    })?;

    println!("响应状态码: {}", search_response.status());
    if !search_response.status().is_success() {
        let status = search_response.status();
        let err_msg = format!(
            "HTTP错误: {} {}",
            status,
            status.canonical_reason().unwrap_or("")
        );
        println!("错误: {}", err_msg);

        // 尝试读取错误响应内容
        let error_body = search_response
            .text()
            .await
            .map_err(|e| format!("读取错误响应失败: {}", e))?;
        println!("错误响应内容: {}", error_body);

        return Err(format!("请求失败: {}. 响应: {}", err_msg, error_body));
    }

    // 解析响应获取topicId
    let search_body = search_response.text().await.map_err(|e| {
        let err_msg = format!("读取响应内容失败: {}", e);
        println!("错误: {}", err_msg);
        err_msg
    })?;

    let search_result: Value = serde_json::from_str(&search_body).map_err(|e| {
        let err_msg = format!("JSON解析失败: {}", e);
        println!("错误: {}", err_msg);
        err_msg
    })?;

    // 从data.list[0].id获取topicId
    let topic_id = search_result["data"]["list"][0]["idstr"]
        .as_str()
        .ok_or_else(|| "未找到topicId".to_string())?;

    println!("获取到的topicId: {}", topic_id);

    // 步骤2: 使用topicId创建评论
    let comment_url = "https://weibo.com/ajax/comments/create";
    println!("评论请求URL: {}", comment_url);

    // 设置评论请求的headers
    let mut comment_headers = HeaderMap::new();
    comment_headers.insert("Cookie", HeaderValue::from_str(&cookie).unwrap());
    comment_headers.insert("Referer", HeaderValue::from_static("https://weibo.com"));
    comment_headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36"));
    comment_headers.insert(
        "Content-Type",
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    comment_headers.insert("Origin", HeaderValue::from_static("https://weibo.com"));

    // 如果cookie中包含XSRF-TOKEN，添加到请求头
    if let Some(token_start) = cookie.find("XSRF-TOKEN=") {
        let token_part = &cookie[token_start + 11..];
        if let Some(token_end) = token_part.find(';') {
            let xsrf_token = &token_part[..token_end];
            comment_headers.insert("x-xsrf-token", HeaderValue::from_str(xsrf_token).unwrap());
        } else {
            comment_headers.insert("x-xsrf-token", HeaderValue::from_str(token_part).unwrap());
        }
    }

    // 创建评论请求的表单数据
    let mut form = HashMap::new();
    form.insert("id", topic_id);
    form.insert("comment", &comment_text);
    form.insert("is_repost", "0");
    form.insert("comment_ori", "0");
    form.insert("is_comment", "0");

    // 发送评论请求
    let comment_client = Client::builder()
        .default_headers(comment_headers)
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("创建评论客户端失败: {}", e))?;

    let comment_response = comment_client
        .post(comment_url)
        .form(&form)
        .send()
        .await
        .map_err(|e| format!("发送评论请求失败: {}", e))?;

    println!("评论响应状态码: {}", comment_response.status());

    // 解析评论响应
    let comment_body = comment_response
        .text()
        .await
        .map_err(|e| format!("读取评论响应失败: {}", e))?;
    println!("评论响应内容: {}", comment_body);
    
    let comment_result: Value =
        serde_json::from_str(&comment_body).map_err(|e| format!("解析评论响应失败: {}", e))?;
    
    // 检查API返回的错误信息
    if let Some(error) = comment_result.get("error") {
        if let Some(error_str) = error.as_str() {
            let err_msg = error_str.to_string();
            println!("微博API返回错误: {}", err_msg);
            return Err(err_msg);
        }
    }
    
    println!("评论创建完成");
    Ok(comment_result)
}
