# 用户管理模块接口文档

## 1. 查询所有用户

| 属性 | 值 |
|------|-----|
| **方法名** | `ManageUserServiceImpl.queryAllUser()` |
| **请求路径** | `/user/queryAll`（前端调用 `/admin/user/queryAll`） |
| **请求方式** | POST |
| **Content-Type** | application/json |

### 入参

```json
{
    "page": 1,           // 可选，默认1
    "limit": 10,         // 可选，默认10
    "keywords": "搜索词", // 可选，模糊搜索用户名/昵称/备注
    "username": "",      // 可选，精确匹配
    "workspaceId": "",   // 可选，精确匹配
    "moduleids": "",     // 可选，模糊匹配
    "phone": "",         // 可选，精确匹配
    "nickname": ""       // 可选，精确匹配
}
```

### 出参

```json
{
    "code": 1,
    "msg": "",
    "data": [
        {
            "id": "1656376952367",
            "username": "虹湾威鹏",
            "moduleids": "1656395208298,...",
            "modulenames": "无人船管理平台,...",
            "logo": "http://...",
            "phone": "138390939231",
            "created": "2022-06-28 00:42:32",
            "updated": "2025-08-04 07:52:43",
            "nickname": "虹湾威鹏",
            "img": null,
            "remarks": "管理员",
            "userType": "admin",
            "workspaceId": "e3dea0f5-...",
            "mqttUsername": "admin",
            "mqttPassword": "admin",
            "history_url": null,
            "realtime_url": null
        }
    ],
    "count": 73
}
```

### 相关SQL

```sql
-- 查询条件（ManageUserMapper.xml:4-35）
SELECT id,username,moduleids,logo,phone,created,updated,nickname,img,remarks,
       user_type as userType,workspace_id as workspaceId,
       mqtt_username as mqttUsername,mqtt_password as mqttPassword,
       history_url,realtime_url
FROM tb_user
WHERE removecode=1
  AND username=#{username}           -- 可选
  AND workspace_id=#{workspaceId}    -- 可选
  AND moduleids LIKE '%...%'         -- 可选
  AND phone=#{phone}                 -- 可选
  AND nickname=#{nickname}           -- 可选
  AND (username LIKE '%...%' OR nickname LIKE '%...%' OR remarks LIKE '%...%')  -- keywords

-- 总数查询（PageHelper自动生成）
SELECT COUNT(*) FROM tb_user WHERE removecode=1 AND ...
```

### 文件位置

| 文件 | 行号 |
|------|------|
| Controller: `3sai-admin/.../UserController.java` | 117-118 |
| Service: `3sai-admin/.../ManageUserServiceImpl.java` | 290-320 |
| Mapper XML: `3sai-admin/.../ManageUserMapper.xml` | 4-35 |

---

## 2. 添加用户

| 属性 | 值 |
|------|-----|
| **方法名** | `ManageUserServiceImpl.addUser()` |
| **请求路径** | `/user/add`（前端调用 `/admin/user/add`） |
| **请求方式** | POST |
| **Content-Type** | application/json |

### 入参

```json
{
    "id": "",
    "username": "test_user",
    "password": "密码",
    "nickname": "昵称",
    "phone": "13800138000",
    "moduleids": "",
    "img": "",
    "logo": "",
    "remarks": "",
    "userType": "admin",
    "workspaceId": "",
    "mqttUsername": "",
    "mqttPassword": "",
    "history_url": "",
    "realtime_url": ""
}
```

### 出参

```json
{
    "code": 1,     // 1=成功, 0=用户名已存在
    "msg": ""
}
```

### 相关SQL

```sql
-- 检查用户名是否存在
SELECT COUNT(*) > 0 FROM tb_user WHERE username=#{username}

-- 插入用户（ManageUserMapper.xml:47-50）
INSERT INTO tb_user (id,username,moduleids,password,phone,created,nickname,img,logo,
                     removecode,remarks,user_type,workspace_id,mqtt_username,mqtt_password,
                     history_url,realtime_url)
VALUES (#{id},#{username},#{moduleids},#{password},#{phone},#{created},#{nickname},#{img},
        #{logo},#{removecode},#{remarks},#{userType},#{workspaceId},#{mqttUsername},
        #{mqttPassword},#{history_url},#{realtime_url})
```

### 文件位置

| 文件 | 行号 |
|------|------|
| Controller: `3sai-admin/.../UserController.java` | 32-44 |
| Service: `3sai-admin/.../ManageUserServiceImpl.java` | 100-130 |
| Mapper XML: `3sai-admin/.../ManageUserMapper.xml` | 47-50 |

---

## 3. 更新用户

| 属性 | 值 |
|------|-----|
| **方法名** | `ManageUserServiceImpl.updateUser()` |
| **请求路径** | `/user/update`（前端调用 `/admin/user/update`） |
| **请求方式** | POST |
| **Content-Type** | application/json |

### 入参

```json
{
    "id": "1656376952367",
    "username": "新用户名",     // 可选
    "moduleids": "1656395208298", // 可选
    "password": "新密码",       // 可选
    "phone": "13800138000",     // 可选
    "nickname": "新昵称",       // 可选
    "img": "",                  // 可选
    "logo": "",                 // 可选
    "remarks": "",              // 可选
    "userType": "admin",        // 可选
    "workspaceId": "",          // 可选
    "mqttUsername": "",         // 可选
    "mqttPassword": "",         // 可选
    "history_url": "",          // 可选
    "realtime_url": ""          // 可选
}
```

### 出参

```json
{
    "code": 1,     // 1=成功, 0=用户名已被其他用户使用
    "msg": ""
}
```

### 相关SQL

```sql
-- 检查用户名是否被其他用户使用
SELECT id,username FROM tb_user WHERE username=#{username} AND id!=#{id}

-- 动态更新（ManageUserMapper.xml:74-130）
UPDATE tb_user SET
    username=#{username},          -- 可选
    moduleids=#{moduleids},        -- 可选
    password=#{password},          -- 可选
    phone=#{phone},                -- 可选
    nickname=#{nickname},          -- 可选
    img=#{img},                    -- 可选
    logo=#{logo},                  -- 可选
    remarks=#{remarks},            -- 可选
    removecode=#{removecode},      -- 可选
    user_type=#{userType},         -- 可选
    workspace_id=#{workspaceId},   -- 可选
    mqtt_username=#{mqttUsername}, -- 可选
    mqtt_password=#{mqttPassword}, -- 可选
    history_url=#{history_url},    -- 可选
    realtime_url=#{realtime_url},  -- 可选
    updated=NOW()
WHERE id=#{id}
```

### 文件位置

| 文件 | 行号 |
|------|------|
| Controller: `3sai-admin/.../UserController.java` | 46-67 |
| Service: `3sai-admin/.../ManageUserServiceImpl.java` | 132-170 |
| Mapper XML: `3sai-admin/.../ManageUserMapper.xml` | 74-130 |

---

## 4. 删除用户

| 属性 | 值 |
|------|-----|
| **方法名** | `ManageUserServiceImpl.deleteUser()` |
| **请求路径** | `/user/deleteUser`（前端调用 `/admin/user/deleteUser`） |
| **请求方式** | POST |
| **Content-Type** | application/json |

### 入参

```json
{
    "id": "1656376952367"
}
```

### 出参

```json
{
    "code": 1,
    "msg": ""
}
```

### 相关SQL

```sql
-- 物理删除（ManageUserMapper.xml:133-134）
DELETE FROM tb_user WHERE id=#{id}
```

### 文件位置

| 文件 | 行号 |
|------|------|
| Controller: `3sai-admin/.../UserController.java` | 106-107 |
| Mapper XML: `3sai-admin/.../ManageUserMapper.xml` | 133-134 |

---

## 5. 更新用户资料

| 属性 | 值 |
|------|-----|
| **方法名** | `ManageUserServiceImpl.updateUser()` |
| **请求路径** | `/user/update/account/profile`（前端调用 `/admin/user/update/account/profile`） |
| **请求方式** | POST |
| **Content-Type** | application/json |

### 入参

与 `/user/update` 相同。

### 出参

与 `/user/update` 相同。

### 处理逻辑

与 `/user/update` 相同，增加了用户名唯一性检查。

### 文件位置

| 文件 | 行号 |
|------|------|
| Controller: `3sai-admin/.../UserController.java` | 69-89 |

---

## 6. 更新用户密码

| 属性 | 值 |
|------|-----|
| **方法名** | `ManageUserServiceImpl.updateUserPassword()` |
| **请求路径** | `/user/update/account/password`（前端调用 `/admin/user/update/account/password`） |
| **请求方式** | POST |
| **Content-Type** | application/json |

### 入参

```json
{
    "id": "1656376952367",
    "oldPassword": "旧密码",
    "newPassword": "新密码"
}
```

### 出参

```json
{
    "code": 1,
    "msg": ""
}
```

### 文件位置

| 文件 | 行号 |
|------|------|
| Controller: `3sai-admin/.../UserController.java` | 91-92 |

---

## 7. 查询用户列表（不带分页）

| 属性 | 值 |
|------|-----|
| **方法名** | `ManageUserServiceImpl.queryUserByUserName()` |
| **请求路径** | `/user/queryUserByUserName` |
| **请求方式** | POST（Query参数） |
| **Content-Type** | 无（Query参数） |

### 入参

```
POST /user/queryUserByUserName?username=虹湾威鹏
```

### 出参

```json
// 直接返回List<User>，不包LayUiTableTemplateLay
[
    {
        "id": "1656376952367",
        "username": "虹湾威鹏",
        "nickname": "虹湾威鹏",
        "phone": "138390939231",
        "logo": "http://..."
    }
]
```

### 相关SQL

```sql
SELECT id,username,moduleids,logo,phone,created,updated,nickname,img,remarks,
       user_type as userType,workspace_id as workspaceId,
       mqtt_username as mqttUsername,mqtt_password as mqttPassword
FROM tb_user WHERE username=#{username} AND removecode=1
```

### 文件位置

| 文件 | 行号 |
|------|------|
| Controller: `3sai-admin/.../UserController.java` | 122-123 |
| Service: `3sai-admin/.../ManageUserServiceImpl.java` | 322-330 |
| Mapper XML: `3sai-admin/.../ManageUserMapper.xml` | 137-150 |

---

## 8. 绑定设备

| 属性 | 值 |
|------|-----|
| **方法名** | `ManageUserServiceImpl.bindBoat()` |
| **请求路径** | `/user/bindBoat`（前端调用 `/admin/user/bindBoat`） |
| **请求方式** | POST |
| **Content-Type** | application/json |

### 入参

```json
{
    "userId": "1656376952367",
    "nodeId": "1656376537357"
}
```

### 出参

```json
{
    "success": true,
    "message": ""
}
```

### 相关SQL

```sql
-- 插入绑定关系（ManageUserMapper.xml:64-67）
INSERT INTO tb_user_node (id, userid, nodeid, removecode)
VALUES (#{id}, #{userId}, #{nodeId}, 1)
```

### 文件位置

| 文件 | 行号 |
|------|------|
| Controller: `3sai-admin/.../UserController.java` | 176-178 |
| Mapper XML: `3sai-admin/.../ManageUserMapper.xml` | 64-67 |

---

## 9. 解除绑定设备

| 属性 | 值 |
|------|-----|
| **方法名** | `ManageUserServiceImpl.notBindboat()` |
| **请求路径** | `/user/notBindboat`（前端调用 `/admin/user/notBindboat`） |
| **请求方式** | POST |
| **Content-Type** | application/json |

### 入参

```json
{
    "userId": "1656376952367",
    "nodeId": "1656376537357"
}
```

### 出参

```json
{
    "success": true,
    "message": ""
}
```

### 相关SQL

```sql
-- 删除绑定关系（ManageUserMapper.xml:70-72）
DELETE FROM tb_user_node WHERE userid=#{userId} AND nodeid=#{nodeId}
```

### 文件位置

| 文件 | 行号 |
|------|------|
| Controller: `3sai-admin/.../UserController.java` | 184-186 |
| Mapper XML: `3sai-admin/.../ManageUserMapper.xml` | 70-72 |

---

## 10. 绑定公共权限

| 属性 | 值 |
|------|-----|
| **方法名** | `ManageUserServiceImpl.bindCommonPermission()` |
| **请求路径** | `/user/bindCommonPermission` |
| **请求方式** | POST |
| **Content-Type** | application/json |

### 入参

```json
{
    "id": "",
    "userid": "1656376952367",
    "permissionid": "1656394942695"
}
```

### 出参

```json
{
    "success": true,
    "message": ""
}
```

### 相关SQL

```sql
-- 插入权限绑定（ManageUserMapper.xml:53-56）
INSERT INTO tb_user_and_cpermission (id, uid, cpid)
VALUES (#{id}, #{userid}, #{permissionid})
```

### 文件位置

| 文件 | 行号 |
|------|------|
| Controller: `3sai-admin/.../UserController.java` | 160-162 |
| Mapper XML: `3sai-admin/.../ManageUserMapper.xml` | 53-56 |
| 实体: `3sai-common/.../UserAndCommonPermission.java` | 全文件 |

---

## 11. 解除绑定公共权限

| 属性 | 值 |
|------|-----|
| **方法名** | `ManageUserServiceImpl.notBindCommonPermission()` |
| **请求路径** | `/user/notBindCommonPermission` |
| **请求方式** | POST |
| **Content-Type** | application/json |

### 入参

```json
{
    "userid": "1656376952367",
    "permissionid": "1656394942695"
}
```

### 出参

```json
{
    "success": true,
    "message": ""
}
```

### 相关SQL

```sql
-- 删除权限绑定（ManageUserMapper.xml:59-61）
DELETE FROM tb_user_and_cpermission WHERE uid=#{userid} AND cpid=#{permissionid}
```

### 文件位置

| 文件 | 行号 |
|------|------|
| Controller: `3sai-admin/.../UserController.java` | 168-170 |
| Mapper XML: `3sai-admin/.../ManageUserMapper.xml` | 59-61 |
