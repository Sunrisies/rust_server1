use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let base_url = "http://localhost:19999";

    println!("=== 船只高级查询模块测试 ===\n");

    // 1. 测试根据用户查询船只
    println!("1. 测试根据用户ID查询船只...");
    let response = client.get(format!(
        "{}/node/queryAllByUserAndOrganization?userId=1656376952367",
        base_url
    ))
    .send()
    .await?;
    println!("   状态码: {}", response.status());
    let result: serde_json::Value = response.json().await?;
    let count = result["data"].as_array().map(|a| a.len()).unwrap_or(0);
    println!("   查询到 {} 艘船只", count);
    if count > 0 {
        println!("   第一艘船: {}", result["data"][0]["sname"].as_str().unwrap_or(""));
    }
    println!();

    // 2. 测试根据用户名查询船只
    println!("2. 测试根据用户名查询船只...");
    let response = client.get(format!(
        "{}/node/queryAllByUserAndOrganization?userName=虹湾威鹏",
        base_url
    ))
    .send()
    .await?;
    println!("   状态码: {}", response.status());
    let result: serde_json::Value = response.json().await?;
    let count = result["data"].as_array().map(|a| a.len()).unwrap_or(0);
    println!("   查询到 {} 艘船只\n", count);

    // 3. 测试带筛选条件查询
    println!("3. 测试带名称筛选查询...");
    let response = client.get(format!(
        "{}/node/queryAllByUserAndOrganization?userId=1656376952367&sname=清理",
        base_url
    ))
    .send()
    .await?;
    println!("   状态码: {}", response.status());
    let result: serde_json::Value = response.json().await?;
    let count = result["data"].as_array().map(|a| a.len()).unwrap_or(0);
    println!("   查询到 {} 艘包含'清理'的船只\n", count);

    // 4. 测试查询用户船只及功能权限
    println!("4. 测试查询用户船只及功能权限...");
    let response = client.get(format!(
        "{}/node/queryNodeAndFunctionByUserId?userId=1656376952367",
        base_url
    ))
    .send()
    .await?;
    println!("   状态码: {}", response.status());
    let result: serde_json::Value = response.json().await?;
    let count = result["data"].as_array().map(|a| a.len()).unwrap_or(0);
    println!("   查询到 {} 艘船只", count);
    if count > 0 {
        let first_node = &result["data"][0];
        let functions_count = first_node["functions"].as_array().map(|a| a.len()).unwrap_or(0);
        let permissions_count = first_node["jurisdictions"].as_array().map(|a| a.len()).unwrap_or(0);
        let parameters_count = first_node["nodeParameters"].as_array().map(|a| a.len()).unwrap_or(0);
        println!("   第一艘船功能数: {}, 权限数: {}, 参数数: {}", 
            functions_count, permissions_count, parameters_count);
    }
    println!();

    // 5. 测试查询用户在线船只
    println!("5. 测试查询用户在线船只...");
    let response = client.get(format!(
        "{}/node/queryOnlineDataByUser?userName=虹湾威鹏",
        base_url
    ))
    .send()
    .await?;
    println!("   状态码: {}", response.status());
    let result: serde_json::Value = response.json().await?;
    let count = result["data"].as_array().map(|a| a.len()).unwrap_or(0);
    println!("   查询到 {} 艘在线船只\n", count);

    println!("=== 船只高级查询模块测试完成 ===");

    Ok(())
}