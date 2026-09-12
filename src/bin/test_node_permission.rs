use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let base_url = "http://localhost:19999";

    println!("=== 船只权限管理模块测试 ===\n");

    // 1. 测试添加船只权限
    println!("1. 测试添加船只权限...");
    let add_request = json!({
        "name": "测试权限",
        "code": 3,
        "details": "这是一个测试权限"
    });

    let response = client.post(format!("{}/nodePermissions/addNodePermission", base_url))
        .json(&add_request)
        .send()
        .await?;
    println!("   状态码: {}", response.status());
    let result: serde_json::Value = response.json().await?;
    println!("   响应: {}", serde_json::to_string_pretty(&result)?);
    
    let permission_id = result["data"]["id"].as_str().unwrap_or("");
    println!("   创建的权限ID: {}\n", permission_id);

    // 2. 测试查询所有权限
    println!("2. 测试查询所有权限...");
    let query_request = json!({
        "page": 1,
        "limit": 10
    });
    let response = client.post(format!("{}/nodePermissions/queryNodePermissionsAll", base_url))
        .json(&query_request)
        .send()
        .await?;
    println!("   状态码: {}", response.status());
    let result: serde_json::Value = response.json().await?;
    println!("   响应: {}", serde_json::to_string_pretty(&result)?);
    println!();

    // 3. 测试根据船只ID查询权限
    println!("3. 测试根据船只ID查询权限...");
    let query_by_node_request = json!({
        "id": "1656376537357"
    });
    let response = client.post(format!("{}/nodePermissions/queryNodePermissionsByNodeId", base_url))
        .json(&query_by_node_request)
        .send()
        .await?;
    println!("   状态码: {}", response.status());
    let result: serde_json::Value = response.json().await?;
    println!("   响应: {}", serde_json::to_string_pretty(&result)?);
    println!();

    // 4. 测试添加船只权限关联
    if !permission_id.is_empty() {
        println!("4. 测试添加船只权限关联...");
        let binding_request = json!({
            "nodeid": "1656376537357",
            "jurid": permission_id
        });
        let response = client.post(format!("{}/nodePermissions/addNodePerAndNode", base_url))
            .json(&binding_request)
            .send()
            .await?;
        println!("   状态码: {}", response.status());
        let result: serde_json::Value = response.json().await?;
        println!("   响应: {}", serde_json::to_string_pretty(&result)?);
        println!();
    }

    // 5. 测试更新船只权限
    if !permission_id.is_empty() {
        println!("5. 测试更新船只权限...");
        let update_request = json!({
            "id": permission_id,
            "name": "更新后的权限",
            "details": "更新后的详情"
        });
        let response = client.put(format!("{}/nodePermissions/updateNodePermission", base_url))
            .json(&update_request)
            .send()
            .await?;
        println!("   状态码: {}", response.status());
        let result: serde_json::Value = response.json().await?;
        println!("   响应: {}", serde_json::to_string_pretty(&result)?);
        println!();
    }

    // 6. 测试删除船只权限关联
    if !permission_id.is_empty() {
        println!("6. 测试删除船只权限关联...");
        let unbinding_request = json!({
            "nodeid": "1656376537357",
            "jurid": permission_id
        });
        let response = client.post(format!("{}/nodePermissions/deleteNodePerAndNodeByNodeIdAndPerId", base_url))
            .json(&unbinding_request)
            .send()
            .await?;
        println!("   状态码: {}", response.status());
        let result: serde_json::Value = response.json().await?;
        println!("   响应: {}", serde_json::to_string_pretty(&result)?);
        println!();
    }

    // 7. 测试删除船只权限
    if !permission_id.is_empty() {
        println!("7. 测试删除船只权限...");
        let delete_request = json!({
            "id": permission_id
        });
        let response = client.delete(format!("{}/nodePermissions/deleteNodePermission", base_url))
            .json(&delete_request)
            .send()
            .await?;
        println!("   状态码: {}", response.status());
        let result: serde_json::Value = response.json().await?;
        println!("   响应: {}", serde_json::to_string_pretty(&result)?);
        println!();
    }

    println!("=== 船只权限管理模块测试完成 ===");

    Ok(())
}