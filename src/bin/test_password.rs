use sqlx::mysql::MySqlPoolOptions;
use sqlx::Row;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    // 解析URL
    let base_url = database_url.split('?').next().unwrap_or(&database_url);
    let without_scheme = base_url.strip_prefix("mysql://").unwrap_or(base_url);
    let at_pos = without_scheme.rfind('@').unwrap();
    let user_pass = &without_scheme[..at_pos];
    let host_db = &without_scheme[at_pos + 1..];
    let colon_pos = user_pass.find(':').unwrap();
    let username = &user_pass[..colon_pos];
    let password = &user_pass[colon_pos + 1..].replace("%40", "@");
    let slash_pos = host_db.find('/').unwrap();
    let host_port = &host_db[..slash_pos];
    let database = &host_db[slash_pos + 1..];
    let colon_pos = host_port.rfind(':').unwrap();
    let host = &host_port[..colon_pos];
    let port: u16 = host_port[colon_pos + 1..].parse().unwrap();
    
    let options = sqlx::mysql::MySqlConnectOptions::new()
        .host(host)
        .port(port)
        .username(username)
        .password(&password)
        .database(database)
        .ssl_mode(sqlx::mysql::MySqlSslMode::Disabled);
    
    let pool = MySqlPoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .expect("Failed to connect");
    
    // 查询虹湾威鹏的密码
    let row = sqlx::query("SELECT id, username, password FROM tb_user WHERE username = '虹湾威鹏'")
        .fetch_one(&pool)
        .await
        .expect("Failed to query");
    
    let id: String = row.get("id");
    let username: String = row.get("username");
    let password_hash: String = row.get("password");
    
    println!("用户ID: {}", id);
    println!("用户名: {}", username);
    println!("密码哈希: {}", password_hash);
    println!();
    
    // 测试密码验证
    let test_password = "123456";
    println!("测试密码: {}", test_password);
    println!("密码哈希是否以$2开头: {}", password_hash.starts_with("$2"));
    
    if password_hash.starts_with("$2") {
        // BCrypt验证
        match bcrypt::verify(test_password, &password_hash) {
            Ok(valid) => println!("BCrypt验证结果: {}", valid),
            Err(e) => println!("BCrypt验证错误: {}", e),
        }
    } else {
        // 明文比较
        println!("明文比较结果: {}", test_password == password_hash);
    }
}
