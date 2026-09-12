fn main() {
    let password = "zhuzhongqian@123456";
    let url_encoded = password.replace('@', "%40");
    
    println!("原始密码: {}", password);
    println!("URL编码后: {}", url_encoded);
    
    // 模拟解析
    let url = format!("mysql://root:{}@api.chaoyang1024.top:9906/db_1125", url_encoded);
    println!();
    println!("完整URL: {}", url);
    
    // 从URL解析密码
    let without_scheme = url.strip_prefix("mysql://").unwrap();
    let at_pos = without_scheme.rfind('@').unwrap();
    let user_pass = &without_scheme[..at_pos];
    let colon_pos = user_pass.find(':').unwrap();
    let decoded_password = &user_pass[colon_pos + 1..];
    
    println!();
    println!("解析出的密码: {}", decoded_password);
    println!("密码是否正确: {}", decoded_password == password);
}
