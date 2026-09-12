#[tokio::main]
async fn main() {
    let base_url = "http://localhost:19999";
    
    println!("========================================");
    println!("  API 接口测试");
    println!("========================================");
    println!();
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap();
    
    // 测试1: 健康检查
    println!("【测试1】健康检查 GET /health");
    match client.get(format!("{}/health", base_url)).send().await {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("  状态码: {}", status);
            println!("  响应: {}", body);
            if status.is_success() {
                println!("  ✅ 通过");
            } else {
                println!("  ❌ 失败");
            }
        }
        Err(e) => println!("  ❌ 请求失败: {}", e),
    }
    println!();
    
    // 测试2: 用户列表
    println!("【测试2】用户列表 GET /user/list");
    match client.get(format!("{}/user/list", base_url)).send().await {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("  状态码: {}", status);
            // 解析JSON并格式化输出
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                println!("  响应: {}", serde_json::to_string_pretty(&json).unwrap_or_default());
                if let Some(data) = json.get("data").and_then(|d| d.as_array()) {
                    println!("  用户数量: {}", data.len());
                }
            } else {
                println!("  响应: {}", body);
            }
            if status.is_success() {
                println!("  ✅ 通过");
            } else {
                println!("  ❌ 失败");
            }
        }
        Err(e) => println!("  ❌ 请求失败: {}", e),
    }
    println!();
    
    // 测试3: 船只列表
    println!("【测试3】船只列表 GET /node/list");
    match client.get(format!("{}/node/list", base_url)).send().await {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("  状态码: {}", status);
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                // 只显示前3条记录
                if let Some(data) = json.get("data").and_then(|d| d.as_array()) {
                    let display_data: Vec<&serde_json::Value> = data.iter().take(3).collect();
                    println!("  响应 (前3条): {}", serde_json::to_string(&display_data).unwrap_or_default());
                    println!("  船只数量: {}", data.len());
                } else {
                    println!("  响应: {}", serde_json::to_string_pretty(&json).unwrap_or_default());
                }
            } else {
                println!("  响应: {}", body);
            }
            if status.is_success() {
                println!("  ✅ 通过");
            } else {
                println!("  ❌ 失败");
            }
        }
        Err(e) => println!("  ❌ 请求失败: {}", e),
    }
    println!();
    
    // 测试4: 用户登录
    println!("【测试4】用户登录 POST /auth/login");
    let login_body = serde_json::json!({
        "username": "虹湾威鹏",
        "password": "123"
    });
    match client.post(format!("{}/auth/login", base_url))
        .header("Content-Type", "application/json")
        .json(&login_body)
        .send()
        .await
    {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("  状态码: {}", status);
            println!("  响应: {}", body);
            if status.is_success() {
                println!("  ✅ 通过");
                // 保存token
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                    if let Some(token) = json.get("token").and_then(|t| t.as_str()) {
                        println!("  Token: {}...", &token[..token.len().min(50)]);
                    }
                }
            } else {
                println!("  ❌ 失败");
            }
        }
        Err(e) => println!("  ❌ 请求失败: {}", e),
    }
    println!();
    
    // 测试5: 注册新用户
    println!("【测试5】注册用户 POST /auth/register");
    let register_body = serde_json::json!({
        "username": "test_user_001",
        "password": "test123456",
        "nickname": "测试用户"
    });
    match client.post(format!("{}/auth/register", base_url))
        .header("Content-Type", "application/json")
        .json(&register_body)
        .send()
        .await
    {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("  状态码: {}", status);
            println!("  响应: {}", body);
            if status.is_success() {
                println!("  ✅ 通过");
            } else {
                println!("  ❌ 失败");
            }
        }
        Err(e) => println!("  ❌ 请求失败: {}", e),
    }
    println!();
    
    println!("========================================");
    println!("  测试完成");
    println!("========================================");
}
