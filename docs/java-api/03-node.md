# 船只管理模块接口文档

## 1. 条件查询船只

| 属性 | 值 |
|------|-----|
| **方法名** | `ManageNodeServiceImpl.queryNodeBycondition()` |
| **请求路径** | `/node/queryAllNode`（前端调用 `/admin/node/queryAllNode`） |
| **请求方式** | POST |
| **Content-Type** | application/json |

### 入参

```json
{
    "page": 1,
    "limit": 10,
    "id": "",
    "sname": "",          // 模糊搜索
    "ip": "",
    "port": "",
    "url": "",
    "imei": "",
    "state": "",
    "taskid": "",
    "remote_port": "",
    "soundBoardId": "",
    "soundBoardName": "",
    "keywords": "搜索词"  // 模糊搜索sname/describes
}
```

### 出参

```json
{
    "code": 1,
    "msg": "",
    "data": [
        {
            "id": "1656376537357",
            "sname": "1号清理船",
            "ip": "116.62.200.219",
            "port": "42259",
            "url": "J41849497",
            "urlname": null,
            "url2": null,
            "urlname2": null,
            "url3": null,
            "urlname3": null,
            "sonorurl1": null,
            "sonorurl1name": null,
            "sonorurl2": null,
            "sonorurl2name": null,
            "imei": null,
            "state": "1",
            "taskid": "d53675",
            "tcpPort": "32259",
            "charge": null,
            "describes": "镇江",
            "remote_port": "28080",
            "fourthGCardNumber": null,
            "fourthGCardNumberExpirationTime": null,
            "rtkcardNumber": null,
            "rtkcardNumberExpirationTime": null,
            "soundBoardId": null,
            "soundBoardName": null,
            "createTime": "2022-06-28 00:42:32"
        }
    ],
    "count": 145
}
```

### 相关SQL

```sql
-- 动态条件查询（ManageNodeMapper.xml:4-48）
SELECT id,sname,ip,port,url,urlname,url2,urlname2,url3,urlname3,
       sonorurl1,sonorurl1name,sonorurl2,sonorurl2name,
       imei,state,taskid,tcpPort,charge,describes,remote_port,
       fourthGCardNumber,fourthGCardNumberExpirationTime,
       rtkcardNumber,rtkcardNumberExpirationTime,
       soundBoardId, soundBoardName
FROM tb_node
WHERE removecode=1
  AND id=#{id}                          -- 可选
  AND sname LIKE '%...%'                -- 可选，模糊
  AND ip=#{ip}                          -- 可选
  AND state=#{state}                    -- 可选
  AND taskid=#{taskid}                  -- 可选
  AND (sname LIKE '%...%' OR describes LIKE '%...%')  -- keywords
ORDER BY id DESC LIMIT #{limit} OFFSET #{offset}
```

### 文件位置

| 文件 | 行号 |
|------|------|
| Controller: `3sai-admin/.../NodeController.java` | 80-82 |
| Service: `3sai-admin/.../ManageNodeServiceImpl.java` | 340-360 |
| Mapper XML: `3sai-admin/.../ManageNodeMapper.xml` | 4-48 |

---

## 2. 查询所有船只（无分页）

| 属性 | 值 |
|------|-----|
| **方法名** | `ManageNodeServiceImpl.queryNodeAll()` |
| **请求路径** | `/node/queryNodeAll`（前端调用 `/admin/node/queryNodeAll`） |
| **请求方式** | POST |
| **Content-Type** | application/json |

### 入参

```json
{}
```

### 出参

```json
{
    "code": 1,
    "msg": "",
    "data": [...],
    "count": 145
}
```

### 相关SQL

```sql
-- 无分页查询（ManageNodeMapper.xml:110-111）
SELECT *,latitude,longitude FROM tb_node WHERE removecode=1 ORDER BY id DESC
```

### 文件位置

| 文件 | 行号 |
|------|------|
| Controller: `3sai-admin/.../NodeController.java` | 92-94 |
| Mapper XML: `3sai-admin/.../ManageNodeMapper.xml` | 110-111 |

---

## 3. 根据用户查询船只

| 属性 | 值 |
|------|-----|
| **方法名** | `ManageNodeServiceImpl.queryNodeByUser()` |
| **请求路径** | `/node/queryNodeByUser` |
| **请求方式** | POST |
| **Content-Type** | application/json |

### 入参

```json
{
    "id": "1656376952367",
    "username": ""
}
```

### 出参

```json
{
    "code": 1,
    "msg": "",
    "data": [...],
    "count": 28
}
```

### 相关SQL

```sql
-- 按用户查询船只（ManageNodeMapper.xml:59-78）
SELECT n.*
FROM tb_user u
JOIN tb_user_node un ON u.id=un.userid
JOIN tb_node n ON un.nodeid=n.id
WHERE u.id=#{id}
  AND n.removecode=1
  AND un.removecode=1
  AND u.removecode=1
  AND u.username=#{username}  -- 可选
```

### 文件位置

| 文件 | 行号 |
|------|------|
| Controller: `3sai-admin/.../NodeController.java` | 103-105 |
| Mapper XML: `3sai-admin/.../ManageNodeMapper.xml` | 59-78 |

---

## 4. 添加船只

| 属性 | 值 |
|------|-----|
| **方法名** | `ManageNodeServiceImpl.addNode()` |
| **请求路径** | `/node/addNode`（前端调用 `/admin/node/addNode`） |
| **请求方式** | POST |
| **Content-Type** | application/json |

### 入参

```json
{
    "sname": "测试船",
    "ip": "192.168.1.100",
    "port": "8080",
    "url": "",
    "url2": "",
    "url3": "",
    "imei": "",
    "tcpPort": "32080",
    "describes": "备注",
    "remote_port": "",
    "fourthGCardNumber": "",
    "fourthGCardNumberExpirationTime": "",
    "rtkcardNumber": "",
    "rtkcardNumberExpirationTime": "",
    "soundBoardId": "",
    "soundBoardName": ""
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
-- 检查船只名称是否已存在
SELECT id,sname FROM tb_node WHERE sname=#{sname} AND removecode=1 LIMIT 1

-- 插入船只（ManageNodeMapper.xml:133-138）
INSERT INTO tb_node (id,sname,url2,ip,port,url,urlname,urlname2,urlname3,
       sonorurl1,sonorurl1name,sonorurl2,sonorurl2name,imei,state,tcpPort,
       taskid,removecode,url3,describes,remote_port,fourthGCardNumber,
       fourthGCardNumberExpirationTime,rtkcardNumber,rtkcardNumberExpirationTime,
       soundBoardId, soundBoardName)
VALUES (#{id},#{sname},#{url2},#{ip},#{port},#{url},#{urlname},#{urlname2},
        #{urlname3},#{sonorurl1},#{sonorurl1name},#{sonorurl2},#{sonorurl2name},
        #{imei},#{state},#{tcpPort},#{taskid},#{removecode},#{url3},#{describes},
        #{remote_port},#{fourthGCardNumber},#{fourthGCardNumberExpirationTime},
        #{rtkcardNumber},#{rtkcardNumberExpirationTime},#{soundBoardId},#{soundBoardName})
```

### 文件位置

| 文件 | 行号 |
|------|------|
| Controller: `3sai-admin/.../NodeController.java` | 37-39 |
| Service: `3sai-admin/.../ManageNodeServiceImpl.java` | 95-115 |
| Mapper XML: `3sai-admin/.../ManageNodeMapper.xml` | 133-138 |

---

## 5. 更新船只

| 属性 | 值 |
|------|-----|
| **方法名** | `ManageNodeServiceImpl.updataNode()` |
| **请求路径** | `/node/updataNode`（前端调用 `/admin/node/updataNode`） |
| **请求方式** | POST |
| **Content-Type** | application/json |

### 入参

与添加船只相同，所有字段可选。

### 出参

```json
{
    "code": 1,
    "msg": ""
}
```

### 相关SQL

```sql
-- 动态更新（ManageNodeMapper.xml:139-186）
UPDATE tb_node SET
    sname=#{sname}, ip=#{ip}, port=#{port}, url=#{url},
    urlname=#{urlname}, url2=#{url2}, urlname2=#{urlname2},
    url3=#{url3}, urlname3=#{urlname3},
    sonorurl1=#{sonorurl1}, sonorurl1name=#{sonorurl1name},
    sonorurl2=#{sonorurl2}, sonorurl2name=#{sonorurl2name},
    imei=#{imei}, state=#{state}, taskid=#{taskid},
    tcpPort=#{tcpPort}, charge=#{charge},
    removecode=#{removecode}, describes=#{describes},
    remote_port=#{remote_port},
    fourthGCardNumber=#{fourthGCardNumber},
    fourthGCardNumberExpirationTime=#{fourthGCardNumberExpirationTime},
    rtkcardNumber=#{rtkcardNumber},
    rtkcardNumberExpirationTime=#{rtkcardNumberExpirationTime},
    soundBoardId=#{soundBoardId}, soundBoardName=#{soundBoardName}
WHERE id=#{id}
```

### 文件位置

| 文件 | 行号 |
|------|------|
| Controller: `3sai-admin/.../NodeController.java` | 43-45 |
| Service: `3sai-admin/.../ManageNodeServiceImpl.java` | 117-125 |
| Mapper XML: `3sai-admin/.../ManageNodeMapper.xml` | 139-186 |

---

## 6. 删除船只（逻辑删除）

| 属性 | 值 |
|------|-----|
| **方法名** | `ManageNodeServiceImpl.deteleNode()` |
| **请求路径** | `/node/deteleNode`（前端调用 `/admin/node/deteleNode`） |
| **请求方式** | POST |
| **Content-Type** | application/json |

### 入参

```json
{
    "id": "1656376537357"
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
-- 逻辑删除：将removecode设为0
UPDATE tb_node SET removecode='0' WHERE id=#{id}
```

### 文件位置

| 文件 | 行号 |
|------|------|
| Controller: `3sai-admin/.../NodeController.java` | 62-64 |
| Service: `3sai-admin/.../ManageNodeServiceImpl.java` | 127-135 |
| Mapper XML: `3sai-admin/.../ManageNodeMapper.xml` | 139-186（复用update） |

---

## 7. 更新船只位置

| 属性 | 值 |
|------|-----|
| **方法名** | `NodeController.updateNodeLocation()` |
| **请求路径** | `/node/updateNodeLocation` |
| **请求方式** | POST |
| **Content-Type** | application/json |

### 入参

```json
{
    "taskid": "d53675",
    "latitude": 32.123456,
    "longitude": 119.123456
}
```

### 出参

```json
{
    "code": 1,
    "msg": "更新成功"
}
```

### 处理逻辑

| 步骤 | 说明 | 行号 |
|------|------|------|
| 1 | 通过taskid查询nodeId | NodeController:158 |
| 2 | 验证经纬度范围 | NodeController:163-165 |
| 3 | 更新tb_node_location | NodeController:167-172 |

### 文件位置

| 文件 | 行号 |
|------|------|
| Controller: `3sai-admin/.../NodeController.java` | 155-176 |

---

## 实体字段参考

### Node实体

```java
// 3sai-common/.../transmit/entity/Node.java
private String id;
private String sname;
private String ip;
private String port;
private List<Map<String, Object>> video;
private String url;           // 摄像头地址
private String urlname;
private String url2;          // 摄像头地址2
private String urlname2;
private String url3;          // 摄像头地址3
private String urlname3;
private String sonorurl1name; // 声纳地址名称
private String sonorurl1;     // 声纳地址
private String sonorurl2name; // 声纳备用地址名称
private String sonorurl2;     // 声纳备用地址
private String imei;          // 继电器IMEI值
private Integer state;        // 状态
private String taskid;        // 任务id
private String message;       // 消息
private String removecode;
private String charge;
private List<NodeFunction> functions;
private List<NodeJurisdiction> jurisdictions;
private List<NodeParameter> nodeParameters;
private Long tcpPort;
private String describes;     // 备注
private String remote_port;   // 远程端口
private String fourthGCardNumber;
private String fourthGCardNumberExpirationTime;
private String rtkcardNumber;
private String rtkcardNumberExpirationTime;
private String createTime;
private Double latitude;
private Double longitude;
private String soundBoardId;
private String soundBoardName;
```

### LayuiNodeTable实体

```java
// 3sai-common/.../manage/entity/LayuiNodeTable.java
// 字段同Node，额外包含：
private String page;
private String limit;
private String keywords;
private String tcpPorts;
```
