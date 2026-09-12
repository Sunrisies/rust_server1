use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let base_url = "http://localhost:19999";

    println!("=== 权限管理模块测试 ===\n");

    // 1. 测试查询所有权限
    println!("1. 测试查询所有权限...");
    let query_request = json!({
        "page": 1,
        "limit": 100
    });
    let response = client.post(format!("{}/permission/queryAll", base_url))
        .json(&query_request)
        .send()
        .await?;
    println!("   状态码: {}", response.status());
    let result: serde_json::Value = response.json().await?;
    let count = result["data"].as_array().map(|a| a.len()).unwrap_or(0);
    println!("   查询到 {} 个权限\n", count);

    // 2. 测试添加权限
    println!("2. 测试添加权限...");
    let add_request = json!({
        "mname": "测试权限",
        "mdesc": "这是一个测试权限",
        "uri": "/admin/test",
        "method": "*"
    });
    let response = client.post(format!("{}/permission/add", base_url))
        .json(&add_request)
        .send()
        .await?;
    println!("   状态码: {}", response.status());
    let result: serde_json::Value = response.json().await?;
    println!("   响应: {}", serde_json::to_string_pretty(&result)?);
    let permission_id = result["data"]["id"].as_str().unwrap_or("");
    println!("   创建的权限ID: {}\n", permission_id);

    // 3. 测试更新权限
    if !permission_id.is_empty() {
        println!("3. 测试更新权限...");
        let update_request = json!({
            "id": permission_id,
            "mname": "更新后的权限",
            "mdesc": "更新后的描述"
        });
        let response = client.put(format!("{}/permission/updatePermission", base_url))
            .json(&update_request)
            .send()
            .await?;
        println!("   状态码: {}", response.status());
        let result: serde_json::Value = response.json().await?;
        println!("   响应: {}", serde_json::to_string_pretty(&result)?);
        println!();
    }

    // 4. 测试带关键词查询
    println!("4. 测试带关键词查询...");
    let query_request = json!({
        "keywords": "测试"
    });
    let response = client.post(format!("{}/permission/queryAll", base_url))
        .json(&query_request)
        .send()
        .await?;
    println!("   状态码: {}", response.status());
    let result: serde_json::Value = response.json().await?;
    let count = result["data"].as_array().map(|a| a.len()).unwrap_or(0);
    println!("   查询到 {} 个包含'测试'的权限\n", count);

    // 5. 测试删除权限
    if !permission_id.is_empty() {
        println!("5. 测试删除权限...");
        let delete_request = json!({
            "id": permission_id
        });
        let response = client.delete(format!("{}/permission/deletePermission", base_url))
            .json(&delete_request)
            .send()
            .await?;
        println!("   状态码: {}", response.status());
        let result: serde_json::Value = response.json().await?;
        println!("   响应: {}", serde_json::to_string_pretty(&result)?);
        println!();
    }

    // 6. 验证删除成功
    println!("6. 验证删除成功...");
    let query_request = json!({
        "keywords": "测试"
    });
    let response = client.post(format!("{}/permission/queryAll", base_url))
        .json(&query_request)
        .send()
        .await?;
    println!("   状态码: {}", response.status());
    let result: serde_json::Value = response.json().await?;
    let count = result["data"].as_array().map(|a| a.len()).unwrap_or(0);
    println!("   查询到 {} 个包含'测试'的权限（应该是0）\n", count);

    println!("=== 权限管理模块测试完成 ===");

    Ok(())
}