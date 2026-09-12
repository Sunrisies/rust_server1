use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let base_url = "http://localhost:19999";

    println!("=== 船坞任务模块测试 ===\n");

    // 1. 测试添加船坞任务
    println!("1. 测试添加船坞任务...");
    let add_request = json!({
        "name": "测试任务-2024",
        "is_out": false,
        "is_stop": true,
        "start_time": ["2024-01-01 10:00:00", "2024-01-02 10:00:00"],
        "air_route": ["route1", "route2"],
        "boat_id": "1656376537357",
        "port": 42259,
        "target_destination": [["116.62", "39.90"], ["116.63", "39.91"]],
        "taskAttribute": "默认"
    });

    let response = client.post(format!("{}/mavlinkOperation/addDockyard", base_url))
        .json(&add_request)
        .send()
        .await?;
    println!("   状态码: {}", response.status());
    let result: serde_json::Value = response.json().await?;
    println!("   响应: {}", serde_json::to_string_pretty(&result)?);
    
    let task_id = result["data"]["id"].as_str().unwrap_or("");
    println!("   创建的任务ID: {}\n", task_id);

    // 2. 测试查询船坞任务（根据boatId查询多个）
    println!("2. 测试查询船坞任务（根据boatId查询多个）...");
    let response = client.get(format!("{}/mavlinkOperation/queryDockyard?boatId=1656376537357", base_url))
        .send()
        .await?;
    println!("   状态码: {}", response.status());
    let result: serde_json::Value = response.json().await?;
    println!("   响应: {}", serde_json::to_string_pretty(&result)?);
    println!();

    // 3. 测试查询单个船坞任务
    if !task_id.is_empty() {
        println!("3. 测试查询单个船坞任务...");
        let response = client.get(format!(
            "{}/mavlinkOperation/querySingleDockyard?boatId=1656376537357&id={}", 
            base_url, task_id
        ))
        .send()
        .await?;
        println!("   状态码: {}", response.status());
        let result: serde_json::Value = response.json().await?;
        println!("   响应: {}", serde_json::to_string_pretty(&result)?);
        println!();
    }

    // 4. 测试更新船坞任务
    if !task_id.is_empty() {
        println!("4. 测试更新船坞任务...");
        let update_request = json!({
            "id": task_id,
            "name": "更新后的任务名",
            "boat_id": "1656376537357",
            "is_out": true,
            "is_stop": true,
            "start_time": ["2024-01-01 11:00:00"],
            "air_route": ["route1", "route2", "route3"],
            "taskAttribute": "紧急"
        });

        let response = client.put(format!("{}/mavlinkOperation/updateDockyard", base_url))
            .json(&update_request)
            .send()
            .await?;
        println!("   状态码: {}", response.status());
        let result: serde_json::Value = response.json().await?;
        println!("   响应: {}", serde_json::to_string_pretty(&result)?);
        println!();
    }

    // 5. 测试删除船坞任务
    if !task_id.is_empty() {
        println!("5. 测试删除船坞任务...");
        let response = client.delete(format!(
            "{}/mavlinkOperation/deleteDockyard?boatId=1656376537357&id={}", 
            base_url, task_id
        ))
        .send()
        .await?;
        println!("   状态码: {}", response.status());
        let result: serde_json::Value = response.json().await?;
        println!("   响应: {}", serde_json::to_string_pretty(&result)?);
        println!();
    }

    // 6. 再次查询确认删除成功
    println!("6. 再次查询确认删除成功...");
    let response = client.get(format!("{}/mavlinkOperation/queryDockyard?boatId=1656376537357", base_url))
        .send()
        .await?;
    println!("   状态码: {}", response.status());
    let result: serde_json::Value = response.json().await?;
    println!("   响应: {}", serde_json::to_string_pretty(&result)?);

    println!("\n=== 船坞任务模块测试完成 ===");

    Ok(())
}