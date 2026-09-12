#[tokio::main]
async fn main() {
    let base_url = "http://localhost:19999";
    
    println!("========================================");
    println!("  登录测试");
    println!("========================================");
    println!();
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap();
    
    // 测试登录
    println!("测试登录: 虹湾威鹏 / 123456");
    let login_body = serde_json::json!({
        "username": "虹湾威鹏",
        "password": "123456"
    });
    
    println!("请求URL: {}/auth/login", base_url);
    println!("请求体: {}", serde_json::to_string_pretty(&login_body).unwrap());
    println!();
    
    match client.post(format!("{}/auth/login", base_url))
        .header("Content-Type", "application/json")
        .json(&login_body)
        .send()
        .await
    {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("状态码: {}", status);
            println!("响应: {}", body);
            println!();
            
            if status.is_success() {
                println!("✅ 登录成功！");
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                    if let Some(token) = json.get("token").and_then(|t| t.as_str()) {
                        println!("Token: {}", token);
                    }
                }
            } else {
                println!("❌ 登录失败");
            }
        }
        Err(e) => println!("请求失败: {}", e),
    }
}
