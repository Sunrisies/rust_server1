use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let base_url = "http://localhost:19999";

    println!("=== 用户高级功能模块测试 ===\n");

    // 先登录获取用户信息
    println!("0. 登录获取用户信息...");
    let login_request = json!({
        "username": "虹湾威鹏",
        "password": "123456"
    });
    let response = client.post(format!("{}/auth/login", base_url))
        .json(&login_request)
        .send()
        .await?;
    let login_result: serde_json::Value = response.json().await?;
    let user_id = login_result["user_id"].as_str().unwrap_or("");
    println!("   登录用户ID: {}\n", user_id);

    // 1. 测试查询用户详情
    println!("1. 测试查询用户详情...");
    let query_request = json!({
        "username": "虹湾威鹏"
    });
    let response = client.post(format!("{}/user/queryUserByUserNameX", base_url))
        .json(&query_request)
        .send()
        .await?;
    println!("   状态码: {}", response.status());
    let result: serde_json::Value = response.json().await?;
    println!("   响应: {}", serde_json::to_string_pretty(&result)?);
    println!();

    // 2. 测试根据用户名查询用户
    println!("2. 测试根据用户名查询用户...");
    let query_request = json!({
        "username": "虹湾威鹏"
    });
    let response = client.post(format!("{}/user/queryUserByUserName", base_url))
        .json(&query_request)
        .send()
        .await?;
    println!("   状态码: {}", response.status());
    let result: serde_json::Value = response.json().await?;
    println!("   响应: {}", serde_json::to_string_pretty(&result)?);
    println!();

    // 3. 测试更新用户资料
    if !user_id.is_empty() {
        println!("3. 测试更新用户资料...");
        let update_request = json!({
            "id": user_id,
            "nickname": "测试昵称",
            "phone": "13800138000"
        });
        let response = client.put(format!("{}/user/update/account/profile", base_url))
            .json(&update_request)
            .send()
            .await?;
        println!("   状态码: {}", response.status());
        let result: serde_json::Value = response.json().await?;
        println!("   响应: {}", serde_json::to_string_pretty(&result)?);
        println!();
    }

    // 4. 测试更新用户密码（错误的旧密码）
    if !user_id.is_empty() {
        println!("4. 测试更新用户密码（错误的旧密码）...");
        let password_request = json!({
            "id": user_id,
            "oldPassword": "wrong_password",
            "newPassword": "new_password"
        });
        let response = client.put(format!("{}/user/update/account/password", base_url))
            .json(&password_request)
            .send()
            .await?;
        println!("   状态码: {}", response.status());
        let result: serde_json::Value = response.json().await?;
        println!("   响应: {}", serde_json::to_string_pretty(&result)?);
        println!();
    }

    // 5. 测试绑定设备
    if !user_id.is_empty() {
        println!("5. 测试绑定设备...");
        let bind_request = json!({
            "userId": user_id,
            "nodeId": "1656376537357"
        });
        let response = client.post(format!("{}/user/bindBoat", base_url))
            .json(&bind_request)
            .send()
            .await?;
        println!("   状态码: {}", response.status());
        let result: serde_json::Value = response.json().await?;
        println!("   响应: {}", serde_json::to_string_pretty(&result)?);
        println!();
    }

    // 6. 测试查询用户船只列表
    if !user_id.is_empty() {
        println!("6. 测试查询用户船只列表...");
        let response = client.get(format!("{}/user/node/list?userId={}", base_url, user_id))
            .send()
            .await?;
        println!("   状态码: {}", response.status());
        let result: serde_json::Value = response.json().await?;
        println!("   响应: {}", serde_json::to_string_pretty(&result)?);
        println!();
    }

    // 7. 测试解除绑定设备
    if !user_id.is_empty() {
        println!("7. 测试解除绑定设备...");
        let unbind_request = json!({
            "userId": user_id,
            "nodeId": "1656376537357"
        });
        let response = client.post(format!("{}/user/notBindboat", base_url))
            .json(&unbind_request)
            .send()
            .await?;
        println!("   状态码: {}", response.status());
        let result: serde_json::Value = response.json().await?;
        println!("   响应: {}", serde_json::to_string_pretty(&result)?);
        println!();
    }

    // 8. 恢复用户资料
    if !user_id.is_empty() {
        println!("8. 恢复用户资料...");
        let restore_request = json!({
            "id": user_id,
            "nickname": "虹湾威鹏",
            "phone": "138390939231"
        });
        let response = client.put(format!("{}/user/update/account/profile", base_url))
            .json(&restore_request)
            .send()
            .await?;
        println!("   状态码: {}", response.status());
        let result: serde_json::Value = response.json().await?;
        println!("   响应: {}", serde_json::to_string_pretty(&result)?);
        println!();
    }

    println!("=== 用户高级功能模块测试完成 ===");

    Ok(())
}