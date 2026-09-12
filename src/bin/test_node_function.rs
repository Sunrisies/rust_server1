use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let base_url = "http://localhost:19999";

    println!("=== 船只功能管理模块测试 ===\n");

    // 1. 测试添加船只功能
    println!("1. 测试添加船只功能...");
    let add_request = json!({
        "name": "测试功能",
        "type": "1",
        "code": "test_code",
        "remark": "这是一个测试功能",
        "on": "1"
    });

    let response = client.post(format!("{}/nodeFuncation/addNodeFuncation", base_url))
        .json(&add_request)
        .send()
        .await?;
    println!("   状态码: {}", response.status());
    let result: serde_json::Value = response.json().await?;
    println!("   响应: {}", serde_json::to_string_pretty(&result)?);
    
    let function_id = result["data"]["id"].as_str().unwrap_or("");
    println!("   创建的功能ID: {}\n", function_id);

    // 2. 测试查询船只功能列表
    println!("2. 测试查询船只功能列表...");
    let query_request = json!({
        "page": 1,
        "limit": 10
    });
    let response = client.post(format!("{}/nodeFuncation/queryNodeFuncation", base_url))
        .json(&query_request)
        .send()
        .await?;
    println!("   状态码: {}", response.status());
    let result: serde_json::Value = response.json().await?;
    println!("   响应: {}", serde_json::to_string_pretty(&result)?);
    println!();

    // 3. 测试根据船只ID查询功能
    println!("3. 测试根据船只ID查询功能...");
    let query_by_node_request = json!({
        "id": "1656376537357"
    });
    let response = client.post(format!("{}/nodeFuncation/queryNodeFuncationById", base_url))
        .json(&query_by_node_request)
        .send()
        .await?;
    println!("   状态码: {}", response.status());
    let result: serde_json::Value = response.json().await?;
    println!("   响应: {}", serde_json::to_string_pretty(&result)?);
    println!();

    // 4. 测试添加船只功能关联
    if !function_id.is_empty() {
        println!("4. 测试添加船只功能关联...");
        let binding_request = json!({
            "nid": "1656376537357",
            "fid": function_id
        });
        let response = client.post(format!("{}/nodeFuncation/addNodeFunAndNode", base_url))
            .json(&binding_request)
            .send()
            .await?;
        println!("   状态码: {}", response.status());
        let result: serde_json::Value = response.json().await?;
        println!("   响应: {}", serde_json::to_string_pretty(&result)?);
        println!();
    }

    // 5. 测试更新船只功能
    if !function_id.is_empty() {
        println!("5. 测试更新船只功能...");
        let update_request = json!({
            "id": function_id,
            "name": "更新后的功能",
            "remark": "更新后的备注"
        });
        let response = client.put(format!("{}/nodeFuncation/updateNodeFuncation", base_url))
            .json(&update_request)
            .send()
            .await?;
        println!("   状态码: {}", response.status());
        let result: serde_json::Value = response.json().await?;
        println!("   响应: {}", serde_json::to_string_pretty(&result)?);
        println!();
    }

    // 6. 测试删除船只功能关联
    if !function_id.is_empty() {
        println!("6. 测试删除船只功能关联...");
        let unbinding_request = json!({
            "nid": "1656376537357",
            "fid": function_id
        });
        let response = client.post(format!("{}/nodeFuncation/deleteNodeFunAndNodeByNodeIdAndFunId", base_url))
            .json(&unbinding_request)
            .send()
            .await?;
        println!("   状态码: {}", response.status());
        let result: serde_json::Value = response.json().await?;
        println!("   响应: {}", serde_json::to_string_pretty(&result)?);
        println!();
    }

    // 7. 测试删除船只功能
    if !function_id.is_empty() {
        println!("7. 测试删除船只功能...");
        let delete_request = json!({
            "id": function_id
        });
        let response = client.delete(format!("{}/nodeFuncation/deleteNodeFuncation", base_url))
            .json(&delete_request)
            .send()
            .await?;
        println!("   状态码: {}", response.status());
        let result: serde_json::Value = response.json().await?;
        println!("   响应: {}", serde_json::to_string_pretty(&result)?);
        println!();
    }

    println!("=== 船只功能管理模块测试完成 ===");

    Ok(())
}