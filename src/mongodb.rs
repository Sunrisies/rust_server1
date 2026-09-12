use mongodb::{Client, options::ClientOptions, options::FindOptions};
use mongodb::bson::{Document, doc};
use mongodb::Collection;
use futures_util::StreamExt;
use tracing::info;

/// MongoDB连接
#[derive(Clone)]
pub struct MongoDB {
    client: Client,
    db_name: String,
    collection_name: String,
}

impl std::fmt::Debug for MongoDB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MongoDB")
            .field("db_name", &self.db_name)
            .field("collection_name", &self.collection_name)
            .finish()
    }
}

impl MongoDB {
    /// 创建新的MongoDB连接
    pub async fn new(
        uri: &str,
        db_name: &str,
        collection_name: &str,
    ) -> anyhow::Result<Self> {
        info!("正在连接MongoDB...");
        
        let client_options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(client_options)?;
        
        info!("MongoDB连接成功");
        
        Ok(Self {
            client,
            db_name: db_name.to_string(),
            collection_name: collection_name.to_string(),
        })
    }
    
    /// 获取集合
    fn get_collection(&self) -> Collection<Document> {
        let db = self.client.database(&self.db_name);
        db.collection(&self.collection_name)
    }
    
    /// 条件查询
    pub async fn find(
        &self,
        query: Document,
        sort: Option<Document>,
        projection: Option<Document>,
        skip: Option<i64>,
        limit: Option<i64>,
    ) -> anyhow::Result<Vec<Document>> {
        let collection = self.get_collection();
        
        let limit_val = limit.unwrap_or(3000);
        let skip_val = skip.unwrap_or(0);
        
        let find_options = FindOptions::builder()
            .sort(sort)
            .projection(projection)
            .skip(skip_val as u64)
            .limit(Some(limit_val))
            .build();
        
        let mut cursor = collection.find(query, find_options).await?;
        
        let mut results = Vec::new();
        while let Some(result) = cursor.next().await {
            match result {
                Ok(doc) => results.push(doc),
                Err(_) => continue,
            }
        }
        
        Ok(results)
    }
    
    /// 查询历史数据
    pub async fn find_historical_data(
        &self,
        query: Document,
        sort: Option<Document>,
        projection: Option<Document>,
        skip: Option<i64>,
        limit: Option<i64>,
    ) -> anyhow::Result<Vec<Document>> {
        self.find(query, sort, projection, skip, limit).await
    }
    
    /// 查询不同的任务名
    pub async fn find_task_names(
        &self,
        query: Document,
    ) -> anyhow::Result<Vec<String>> {
        let collection = self.get_collection();
        
        let find_options = FindOptions::builder()
            .projection(doc! { "taskname": 1, "_id": 0 })
            .build();
        
        let mut cursor = collection.find(query, find_options).await?;
        
        let mut task_names = Vec::new();
        while let Some(result) = cursor.next().await {
            if let Ok(doc) = result {
                if let Ok(taskname) = doc.get_str("taskname") {
                    if !task_names.contains(&taskname.to_string()) {
                        task_names.push(taskname.to_string());
                    }
                }
            }
        }
        
        Ok(task_names)
    }
    
    /// 测试连接
    pub async fn ping(&self) -> anyhow::Result<()> {
        let db = self.client.database(&self.db_name);
        db.run_command(doc! { "ping": 1 }, None).await?;
        Ok(())
    }
}

/// MongoDB配置
#[derive(Debug, Clone)]
pub struct MongoDBConfig {
    pub uri: String,
    pub db_name: String,
    pub collection_name: String,
}

impl MongoDBConfig {
    pub fn from_env() -> Self {
        use std::env;
        
        let ip = env::var("MONGODB_IP").unwrap_or_else(|_| "47.104.240.65".to_string());
        let port = env::var("MONGODB_PORT").unwrap_or_else(|_| "8017".to_string());
        let username = env::var("MONGODB_USERNAME").unwrap_or_else(|_| "root".to_string());
        let password = env::var("MONGODB_PASSWORD").unwrap_or_else(|_| "Hover_66weapon".to_string());
        let db_name = env::var("MONGODB_DB_NAME").unwrap_or_else(|_| "hr".to_string());
        let collection_name = env::var("MONGODB_COLLECTION").unwrap_or_else(|_| "boat1".to_string());
        
        let uri = format!("mongodb://{}:{}@{}:{}/admin", username, password, ip, port);
        
        Self {
            uri,
            db_name,
            collection_name,
        }
    }
}

/// 初始化MongoDB连接
pub async fn init_mongodb(config: &MongoDBConfig) -> anyhow::Result<MongoDB> {
    let mongodb = MongoDB::new(&config.uri, &config.db_name, &config.collection_name).await?;
    
    // 测试连接
    mongodb.ping().await?;
    
    info!("MongoDB连接测试通过");
    
    Ok(mongodb)
}