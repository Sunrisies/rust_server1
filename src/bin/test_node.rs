#[tokio::main]
async fn main() {
    let base_url = "http://localhost:19999";
    
    println!("========================================");
    println!("  船只模块接口测试");
    println!("========================================");
    println!();
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap();
    
    // 测试1: 船只列表
    println!("【测试1】船只列表 GET /node/list");
    match client.get(format!("{}/node/list", base_url)).send().await {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("  状态码: {}", status);
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                if let Some(data) = json.get("data").and_then(|d| d.as_array()) {
                    println!("  船只数量: {}", data.len());
                    // 显示前3条
                    for (i, node) in data.iter().take(3).enumerate() {
                        let id = node.get("id").and_then(|v| v.as_str()).unwrap_or("");
                        let name = node.get("sname").and_then(|v| v.as_str()).unwrap_or("");
                        let state = node.get("state").and_then(|v| v.as_str()).unwrap_or("");
                        println!("  {}. {} ({}) - 状态: {}", i + 1, name, id, state);
                    }
                }
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
    
    // 测试2: 船只详情
    println!("【测试2】船只详情 GET /node/detail?id=1656376537357");
    match client.get(format!("{}/node/detail?id=1656376537357", base_url)).send().await {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("  状态码: {}", status);
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                println!("  响应: {}", serde_json::to_string_pretty(&json).unwrap_or_default());
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
    
    // 测试3: 添加船只
    println!("【测试3】添加船只 POST /node/add");
    let add_node_body = serde_json::json!({
        "sname": "测试无人船",
        "ip": "192.168.1.100",
        "port": "8080",
        "describes": "这是一艘测试无人船"
    });
    match client.post(format!("{}/node/add", base_url))
        .header("Content-Type", "application/json")
        .json(&add_node_body)
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
    
    // 测试4: 更新船只位置
    println!("【测试4】更新船只位置 PUT /node/updateLocation");
    let update_location_body = serde_json::json!({
        "id": "1656376537357",
        "latitude": 39.9042,
        "longitude": 116.4074
    });
    match client.put(format!("{}/node/updateLocation", base_url))
        .header("Content-Type", "application/json")
        .json(&update_location_body)
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
    
    // 测试5: 查询船只参数
    println!("【测试5】查询船只参数 GET /node/parameter/list?nodeId=1656376537357");
    match client.get(format!("{}/node/parameter/list?nodeId=1656376537357", base_url)).send().await {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("  状态码: {}", status);
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                if let Some(data) = json.get("data").and_then(|d| d.as_array()) {
                    println!("  参数数量: {}", data.len());
                    for param in data.iter().take(3) {
                        let name = param.get("name").and_then(|v| v.as_str()).unwrap_or("");
                        let value = param.get("value").and_then(|v| v.as_str()).unwrap_or("");
                        println!("  - {}: {}", name, value);
                    }
                }
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
    
    // 测试6: 用户船只关联
    println!("【测试6】用户船只关联 GET /user/node/list?userId=1656376952367");
    match client.get(format!("{}/user/node/list?userId=1656376952367", base_url)).send().await {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("  状态码: {}", status);
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                if let Some(data) = json.get("data").and_then(|d| d.as_array()) {
                    println!("  关联船只数量: {}", data.len());
                    for node in data.iter().take(3) {
                        let node_id = node.get("node_id").and_then(|v| v.as_str()).unwrap_or("");
                        let node_name = node.get("node_name").and_then(|v| v.as_str()).unwrap_or("");
                        println!("  - {} ({})", node_name, node_id);
                    }
                }
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
    
    // 测试7: 区块任务列表
    println!("【测试7】区块任务列表 GET /blockPlan/list");
    match client.get(format!("{}/blockPlan/list", base_url)).send().await {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("  状态码: {}", status);
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                if let Some(data) = json.get("data").and_then(|d| d.as_array()) {
                    println!("  任务数量: {}", data.len());
                    for plan in data.iter().take(3) {
                        let id = plan.get("id").and_then(|v| v.as_str()).unwrap_or("");
                        let name = plan.get("task_name").and_then(|v| v.as_str()).unwrap_or("");
                        let boat_id = plan.get("boat_id").and_then(|v| v.as_str()).unwrap_or("");
                        println!("  - {} ({}) 船只: {}", name, id, boat_id);
                    }
                }
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
    
    println!("========================================");
    println!("  测试完成");
    println!("========================================");
}
