# Rust 重写实现状态文档

## 项目概述

| 项目 | 说明 |
|------|------|
| **项目名称** | rust_hover |
| **原项目** | platform-backend (Java Spring Cloud微服务) |
| **目标** | 使用Rust重写，适配2H4G服务器 |
| **当前进度** | 阶段3完成 (60%) |
| **服务器地址** | http://localhost:19999 |

---

## 项目结构

```
rust_hover/
├── Cargo.toml              # 依赖配置
├── .env                    # 环境变量配置
├── .gitignore              # Git忽略文件
├── docs/                   # 文档目录
│   └── implementation-status.md
├── src/
│   ├── main.rs             # 主入口
│   ├── config.rs           # 配置模块
│   ├── database.rs         # 数据库模块 (SQLx + MySQL)
│   ├── redis.rs            # Redis模块
│   ├── error.rs            # 统一错误处理
│   ├── jwt.rs              # JWT认证模块
│   ├── middleware.rs        # 认证中间件
│   └── routes.rs           # API路由定义
└── bin/
    ├── test_api.rs         # API测试脚本
    ├── test_login.rs       # 登录测试
    ├── test_node.rs        # 船只模块测试
    ├── test_db.rs          # 数据库连接测试
    ├── test_password.rs    # 密码验证测试
    └── check_tables.rs     # 表结构检查
```

---

## 技术栈

| 组件 | 选型 | 版本 |
|------|------|------|
| 语言 | Rust | 2021 Edition |
| Web框架 | Axum | 0.7 |
| 异步运行时 | Tokio | 1.x |
| ORM | SQLx | 0.7 |
| 缓存 | redis-rs | 0.24 |
| JWT | jsonwebtoken | 9 |
| 密码加密 | bcrypt | 0.15 |
| 日志 | tracing | 0.1 |
| 配置 | dotenvy | 0.15 |

---

## 环境配置

### .env 文件

```env
# 数据库配置
DATABASE_URL=mysql://root:zhuzhongqian%40123456@api.chaoyang1024.top:9906/db_1125

# Redis配置
REDIS_URL=redis://122.51.231.16:9379

# 服务器配置
SERVER_HOST=0.0.0.0
SERVER_PORT=19999

# JWT配置
JWT_SECRET=your-jwt-secret-key-change-in-production
JWT_EXPIRE_HOURS=24
```

### 注意事项

1. **密码特殊字符**: 数据库密码中的 `@` 需要URL编码为 `%40`
2. **SSL配置**: MySQL连接禁用了SSL (`ssl-mode=disabled`)
3. **端口选择**: 使用19999避免与其他服务冲突

---

## 已实现模块

### 1. 配置模块 (config.rs)

**功能**: 从 `.env` 文件读取配置

```rust
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub server: ServerConfig,
    pub jwt: JwtConfig,
}
```

### 2. 数据库模块 (database.rs)

**功能**: MySQL连接池管理

- 支持URL编码解码（处理密码特殊字符）
- 连接池配置（最大25连接）
- 自动重连机制

### 3. Redis模块 (redis.rs)

**功能**: Redis缓存操作

- 连接管理 (ConnectionManager)
- 基本操作: `set`, `get`, `del`, `set_ex`, `exists`
- 异步支持

### 4. 错误处理 (error.rs)

**功能**: 统一错误处理

```rust
pub enum AppError {
    Database(sqlx::Error),
    Redis(redis::RedisError),
    Auth(String),
    NotFound(String),
    BadRequest(String),
    Internal(anyhow::Error),
}
```

### 5. JWT模块 (jwt.rs)

**功能**: JWT Token生成和验证

```rust
pub struct JwtUtil {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    expire_hours: i64,
}
```

### 6. 认证中间件 (middleware.rs)

**功能**: 请求认证

- 白名单路径（/health, /auth/login, /auth/register）
- Token提取（Header或Query参数）
- 用户信息注入

### 7. 路由模块 (routes.rs)

**功能**: API路由定义

---

## API接口文档

### 认证接口

#### 登录

```
POST /auth/login
Content-Type: application/json

{
    "username": "虹湾威鹏",
    "password": "123456"
}

Response:
{
    "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
    "user_id": "1656376952367",
    "username": "虹湾威鹏"
}
```

#### 注册

```
POST /auth/register
Content-Type: application/json

{
    "username": "test_user",
    "password": "test123456",
    "nickname": "测试用户"
}

Response:
{
    "code": 200,
    "message": "success",
    "data": {
        "user_id": "uuid",
        "username": "test_user"
    }
}
```

### 用户接口

#### 用户列表

```
GET /user/list

Response:
{
    "code": 200,
    "data": [
        {
            "id": "1656376952367",
            "username": "虹湾威鹏",
            "nickname": "虹湾威鹏",
            "phone": "138390939231"
        }
    ],
    "message": "success"
}
```

#### 添加用户

```
POST /user/add
Content-Type: application/json

{
    "username": "new_user",
    "password": "password123",
    "nickname": "新用户",
    "phone": "13800138000"
}
```

#### 更新用户

```
PUT /user/update
Content-Type: application/json

{
    "id": "user_id",
    "nickname": "新昵称",
    "phone": "13900139000"
}
```

#### 删除用户

```
DELETE /user/delete?id=user_id
```

### 船只接口

#### 船只列表

```
GET /node/list

Response:
{
    "code": 200,
    "data": [
        {
            "id": "1656376537357",
            "sname": "1号清理船",
            "ip": "116.62.200.219",
            "port": "42259",
            "state": "1",
            "task_id": "d53675",
            "describes": "镇江"
        }
    ],
    "message": "success"
}
```

#### 船只详情

```
GET /node/detail?id=1656376537357

Response:
{
    "code": 200,
    "data": {
        "id": "1656376537357",
        "sname": "1号清理船",
        "ip": "116.62.200.219",
        "port": "42259",
        "url": "J41849497",
        "state": "1",
        "task_id": "d53675",
        "tcp_port": "32259",
        "describes": "镇江",
        "remote_port": "28080"
    },
    "message": "success"
}
```

#### 添加船只

```
POST /node/add
Content-Type: application/json

{
    "sname": "测试无人船",
    "ip": "192.168.1.100",
    "port": "8080",
    "describes": "这是一艘测试无人船"
}

Response:
{
    "code": 200,
    "message": "success",
    "data": {
        "node_id": "uuid"
    }
}
```

#### 更新船只

```
PUT /node/update
Content-Type: application/json

{
    "id": "node_id",
    "sname": "新名称",
    "ip": "192.168.1.101",
    "state": "online"
}
```

#### 删除船只

```
DELETE /node/delete?id=node_id
```

#### 更新位置

```
PUT /node/updateLocation
Content-Type: application/json

{
    "id": "1656376537357",
    "latitude": 39.9042,
    "longitude": 116.4074
}
```

#### 船只参数列表

```
GET /node/parameter/list?nodeId=1656376537357

Response:
{
    "code": 200,
    "data": [
        {
            "id": "param_id",
            "node_id": "1656376537357",
            "npid": "param_type_id"
        }
    ],
    "message": "success"
}
```

#### 更新参数

```
POST /node/parameter/update
Content-Type: application/json

{
    "node_id": "node_id",
    "name": "param_name",
    "value": "param_value",
    "code": "param_code"
}
```

### 用户船只关联

```
GET /user/node/list?userId=1656376952367

Response:
{
    "code": 200,
    "data": [
        {
            "id": "rel_id",
            "user_id": "1656376952367",
            "node_id": "1656424151304",
            "node_name": "105-1"
        }
    ],
    "message": "success"
}
```

### 区块任务接口

#### 任务列表

```
GET /blockPlan/list
GET /blockPlan/list?boatId=boat_id

Response:
{
    "code": 200,
    "data": [
        {
            "id": "task_id",
            "boat_id": "927be5",
            "task_name": "2025-04-25 08:44"
        }
    ],
    "message": "success"
}
```

#### 添加任务

```
POST /blockPlan/add
Content-Type: application/json

{
    "boat_id": "boat_id",
    "task_name": "任务名称",
    "create_address": "[...]",
    "route": "[...]",
    "polygon": "[...]",
    "course_spacing": 2.0,
    "course_angle": 0.0,
    "complete_action": "LOITER"
}
```

#### 更新任务

```
PUT /blockPlan/update
Content-Type: application/json

{
    "id": "task_id",
    "task_name": "新任务名称",
    "course_spacing": 3.0
}
```

#### 删除任务

```
DELETE /blockPlan/delete?id=task_id
```

---

## 测试结果

### 基础测试

| 测试项 | 状态 | 说明 |
|--------|------|------|
| 数据库连接 | ✅ 通过 | MySQL 9.1.0 |
| Redis连接 | ✅ 通过 | 读写正常 |
| 健康检查 | ✅ 通过 | /health |

### 认证测试

| 测试项 | 状态 | 说明 |
|--------|------|------|
| 用户登录 | ✅ 通过 | 虹湾威鹏/123456 |
| 用户注册 | ✅ 通过 | 创建新用户 |
| JWT生成 | ✅ 通过 | Token正常 |
| BCrypt验证 | ✅ 通过 | 密码匹配 |

### 船只模块测试

| 测试项 | 状态 | 数据 |
|--------|------|------|
| 船只列表 | ✅ 通过 | 145艘船只 |
| 船只详情 | ✅ 通过 | 返回完整信息 |
| 添加船只 | ✅ 通过 | 成功创建 |
| 更新位置 | ✅ 通过 | 位置记录 |
| 船只参数 | ✅ 通过 | 21个参数 |
| 用户船只 | ✅ 通过 | 34艘关联 |
| 区块任务 | ✅ 通过 | 15个任务 |

---

## 数据库表结构

### 主要业务表

| 表名 | 用途 | 记录数 |
|------|------|--------|
| tb_user | 用户表 | 77 |
| tb_node | 船只表 | 145 |
| tb_node_location | 船只位置 | - |
| tb_node_and_node_parameter | 船只参数 | 723 |
| tb_user_node | 用户船只关联 | 290 |
| block_plan | 区块任务 | 15 |

### 表字段映射

#### tb_node (船只表)

| 字段 | 类型 | 说明 |
|------|------|------|
| id | varchar(255) | 主键 |
| sname | varchar(255) | 船只名称 |
| ip | varchar(255) | IP地址 |
| port | varchar(255) | 端口 |
| url | varchar(255) | 视频流ID |
| state | varchar(255) | 状态 |
| taskid | varchar(255) | 当前任务ID |
| tcpPort | varchar(255) | TCP端口 |
| describes | varchar(255) | 备注 |
| remote_port | varchar(255) | 远程端口 |

#### tb_node_location (位置表)

| 字段 | 类型 | 说明 |
|------|------|------|
| id | varchar(191) | 主键 |
| latitude | decimal(10,6) | 纬度 |
| longitude | decimal(10,6) | 经度 |
| update_time | timestamp | 更新时间 |

#### tb_node_and_node_parameter (参数表)

| 字段 | 类型 | 说明 |
|------|------|------|
| id | varchar(255) | 主键 |
| nodeid | varchar(255) | 船只ID |
| npid | varchar(255) | 参数类型ID |
| removecode | varchar(10) | 删除标记 |

#### tb_user_node (用户船只关联)

| 字段 | 类型 | 说明 |
|------|------|------|
| id | varchar(255) | 主键 |
| userid | varchar(255) | 用户ID |
| nodeid | varchar(255) | 船只ID |
| removecode | int | 删除标记 |

---

## 开发日志

### 2026-09-12

1. **环境搭建**
   - 创建Rust项目
   - 配置Cargo.toml依赖
   - 设置.env配置文件

2. **数据库接入**
   - 实现SQLx连接
   - 解决URL编码问题（密码中的@）
   - 测试连接成功

3. **Redis接入**
   - 实现redis-rs连接
   - 测试基本操作

4. **核心框架**
   - 实现Axum Web框架
   - 统一错误处理
   - JWT认证模块
   - 认证中间件

5. **用户模块**
   - 登录接口（支持BCrypt）
   - 注册接口
   - 用户CRUD

6. **船只模块**
   - 船只列表/详情
   - 添加/更新/删除
   - 位置更新
   - 参数管理
   - 用户船只关联

7. **任务模块**
   - 区块任务CRUD

---

## 待实现功能

### 阶段4: 任务模块完善

- [ ] 船坞任务 (mavlinkOperation)
- [ ] 地图观测计划 (mapObsPlan)
- [ ] 任务状态管理
- [ ] 任务下发

### 阶段5: 通信模块

- [ ] MQTT客户端
- [ ] WebSocket服务
- [ ] 消息路由
- [ ] 实时通信

### 优化项

- [ ] 添加认证中间件到路由
- [ ] 完善错误处理
- [ ] 添加日志记录
- [ ] 性能优化
- [ ] 单元测试

---

## 性能指标

| 指标 | 目标值 | 当前值 |
|------|--------|--------|
| 内存占用 | < 100MB | ~30MB |
| 启动时间 | < 1秒 | ~0.5秒 |
| QPS | > 1000 | 待测试 |
| P99延迟 | < 100ms | 待测试 |

---

## 运行命令

### 编译

```bash
cargo build --release
```

### 运行服务

```bash
cargo run --bin rust_hover
```

### 运行测试

```bash
cargo run --bin test_api      # API测试
cargo run --bin test_login    # 登录测试
cargo run --bin test_node     # 船只模块测试
```

---

## 与Java版对比

| 维度 | Java版 | Rust版 |
|------|--------|--------|
| 服务数量 | 6个微服务 | 1个单体 |
| 内存占用 | ~3.6GB | ~30MB |
| 启动时间 | 30-60秒 | <1秒 |
| 技术栈 | Spring Cloud | Axum |
| 数据库 | MySQL + MongoDB | MySQL |
| 缓存 | Redis | Redis |
| 通信 | HTTP + MQTT | HTTP (待实现MQTT) |

---

## 注意事项

1. **密码处理**: 数据库中的密码是BCrypt加密，登录时自动识别并验证
2. **URL编码**: 密码中的特殊字符（如@）需要URL编码
3. **端口冲突**: 默认使用19999端口，避免与其他服务冲突
4. **表结构**: 代码中的表结构基于实际数据库，可能与预期不同
