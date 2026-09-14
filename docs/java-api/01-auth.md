# 认证模块接口文档

## 1. 登录

| 属性 | 值 |
|------|-----|
| **方法名** | `LoginServiceImpl.login()` |
| **请求路径** | `/register/login`（前端调用 `/login/register/login`） |
| **请求方式** | POST |
| **Content-Type** | application/json |

### 入参

```java
// 3sai-common/.../LoginMessage.java
{
    "userName": "虹湾威鹏",       // 必填，用户名
    "passWord": "RSA加密后的密码", // 必填，RSA加密密码
    "verifyCode": "",              // 可选，验证码
    "userKey": ""                  // 可选，用户密钥
}
```

**文件位置**: `3sai-common/src/main/java/com/hongwanweipeng/user/entity/LoginMessage.java`

### 出参

```json
{
    "code": 1,           // 1=成功, 0=失败
    "msg": "获取成功",
    "data": {
        "userId": "1656376952367",
        "userName": "虹湾威鹏",
        "permission": [],
        "modules": [...],
        "token": "RSA加密的Token",
        "nickName": "虹湾威鹏",
        "logo": "http://...",
        "history_url": null,
        "realtime_url": null,
        "workspaceId": "e3dea0f5-..."
    },
    "count": null,
    "obj": null
}
```

### 处理逻辑

| 步骤 | 说明 | 行号 |
|------|------|------|
| 1 | 参数校验：userName/passWord 不能为空 | LoginServiceImpl:70 |
| 2 | RSA解密 passWord | LoginServiceImpl:76-85 |
| 3 | 查询用户（条件：username） | ManageUserMapper: queryUserBycondition |
| 4 | BCrypt校验密码 | LoginServiceImpl:128-130 |
| 5 | 生成Token（时间戳+盐值+权限，RSA加密） | LoginServiceImpl:131-138 |
| 6 | 保存Session到Redis（key=login:user:{token}, TTL=1800秒） | Session:35 |
| 7 | 查询用户模块（tb_module WHERE id IN moduleids AND removecode='1'） | LoginServiceImpl:142-155 |
| 8 | 查询模块权限并缓存到Redis | LoginServiceImpl:158-182 |

### 相关SQL

```sql
-- 查询用户
SELECT id,username,moduleids,password,logo,phone,created,updated,nickname,img,remarks,
       user_type as userType,workspace_id as workspaceId,mqtt_username as mqttUsername,
       mqtt_password as mqttPassword,history_url,realtime_url
FROM tb_user WHERE username=#{username} AND removecode=1

-- 查询模块
SELECT id,modulename,moduledesc,moduleico,routeurl,removecode FROM tb_module WHERE id=#{id} AND removecode='1'

-- 查询模块权限
SELECT id,mid,pid FROM tb_module_permission WHERE mid=#{moduleid}
```

### 文件位置

| 文件 | 行号 |
|------|------|
| Controller: `3sai-ucenter/.../LoginController.java` | 37-39 |
| Service: `3sai-ucenter/.../LoginServiceImpl.java` | 64-198 |
| 实体: `3sai-common/.../LoginMessage.java` | 全文件 |
| Session: `3sai-common/.../Session.java` | 35-48 |

---

## 2. 获取公钥

| 属性 | 值 |
|------|-----|
| **方法名** | `LoginServiceImpl.getPublicKey()` |
| **请求路径** | `/register/getPublicKey` |
| **请求方式** | GET |
| **Content-Type** | 无 |

### 入参

无参数。

### 出参

```json
{
    "code": 1,
    "msg": "公钥信息",
    "data": "MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQ...",  // Base64编码的X509 DER公钥
    "obj": "key"
}
```

### 处理逻辑

| 步骤 | 说明 | 行号 |
|------|------|------|
| 1 | 从Redis获取RSA公钥 | RSAUtils:347 |
| 2 | Base64编码返回 | RSAUtils:347-350 |

### 文件位置

| 文件 | 行号 |
|------|------|
| Controller: `3sai-ucenter/.../LoginController.java` | 49-51 |
| Service: `3sai-ucenter/.../LoginServiceImpl.java` | 210-225 |

---

## 3. 验证Token

| 属性 | 值 |
|------|-----|
| **方法名** | `LoginServiceImpl.verifyToken()` |
| **请求路径** | `/register/verifyToken` |
| **请求方式** | POST（Query参数） |
| **Content-Type** | 无（Query参数） |

### 入参

```
GET /register/verifyToken?token=xxx
```

### 出参

```json
// 成功
{
    "success": true,
    "message": "ok",
    "data": "时间戳,HOVERWEAPON,权限"
}

// 失败
{
    "success": false,
    "message": "token为空/token异常/token不合法/token已过期"
}
```

### 处理逻辑

| 步骤 | 说明 | 行号 |
|------|------|------|
| 1 | 检查token是否为空 | LoginServiceImpl:233 |
| 2 | RSA解密token | TokenUtils:analysis() |
| 3 | 验证格式：时间戳,HOVERWEAPON,权限 | LoginServiceImpl:238-245 |
| 4 | 验证有效期（7200000ms = 2小时） | LoginServiceImpl:247-249 |

### 文件位置

| 文件 | 行号 |
|------|------|
| Controller: `3sai-ucenter/.../LoginController.java` | 61-63 |
| Service: `3sai-ucenter/.../LoginServiceImpl.java` | 232-255 |

---

## 4. 退出登录

| 属性 | 值 |
|------|-----|
| **方法名** | `LoginServiceImpl.logout()` |
| **请求路径** | `/register/logout` |
| **请求方式** | POST |
| **Content-Type** | 无 |

### 入参

```
Header: token: xxx（Token字符串）
```

### 出参

```json
{
    "code": 1,
    "msg": "",
    "data": "登出成功！"
}
```

### 处理逻辑

| 步骤 | 说明 | 行号 |
|------|------|------|
| 1 | 从请求头获取token | LoginController:75-79 |
| 2 | 获取Session | Session:getSession() |
| 3 | 删除Session和权限缓存 | Session:deleteSession() |

### 文件位置

| 文件 | 行号 |
|------|------|
| Controller: `3sai-ucenter/.../LoginController.java` | 71-89 |
