# 参数管理模块接口文档

## 1. 条件查询参数

| 属性 | 值 |
|------|-----|
| **方法名** | `NodeAndNodeParameterServiceImpl.queryParameter()` |
| **请求路径** | `/parameter/queryParameter`（前端调用 `/admin/parameter/queryParameter`） |
| **请求方式** | POST |
| **Content-Type** | application/json |

### 入参

```json
{
    "nodeId": "1656376537357",
    "page": 1,
    "limit": 10
}
```

### 出参

```json
{
    "code": 1,
    "msg": "",
    "data": [
        {
            "id": "param_id",
            "name": "参数名",
            "code": "ph",
            "hcode": null,
            "unit": "mg/L"
        }
    ],
    "count": 5
}
```

### 相关SQL

```sql
-- 查询船只关联的参数（NodeAndNodeParameterMapper.xml:45-55）
SELECT n.id,n.name,n.code,n.hcode,n.unit
FROM tb_node_parameter n
JOIN tb_node_and_node_parameter np ON n.id=np.npid
WHERE np.nodeid=#{nodeId}
```

### 文件位置

| 文件 | 行号 |
|------|------|
| Controller: `3sai-admin/.../NodeAndNodeParameterController.java` | 29-31 |
| Mapper XML: `3sai-admin/.../NodeAndNodeParameterMapper.xml` | 45-55 |

---

## 2. 查询所有参数

| 属性 | 值 |
|------|-----|
| **方法名** | `NodeAndNodeParameterServiceImpl.queryParameterAll()` |
| **请求路径** | `/parameter/queryParameterAll`（前端调用 `/admin/parameter/queryParameterAll`） |
| **请求方式** | POST |
| **Content-Type** | 无参数 |

### 出参

```json
{
    "code": 1,
    "msg": "",
    "data": [
        {"id": "1656377652016", "name": "氨氮", "code": "NH34", "hcode": null, "unit": null}
    ]
}
```

### 相关SQL

```sql
-- 查询所有参数（NodeAndNodeParameterMapper.xml:4-14）
SELECT id,name,code,hcode,unit FROM tb_node_parameter
```

### 文件位置

| 文件 | 行号 |
|------|------|
| Controller: `3sai-admin/.../NodeAndNodeParameterController.java` | 38-40 |
| Mapper XML: `3sai-admin/.../NodeAndNodeParameterMapper.xml` | 4-14 |

---

## 3. 添加参数

| 属性 | 值 |
|------|-----|
| **请求路径** | `/parameter/addParameter` |
| **请求方式** | POST |
| **Content-Type** | application/json |

### 入参

```json
{
    "name": "参数名",
    "code": "ph",
    "hcode": "",
    "unit": "mg/L"
}
```

### 相关SQL

```sql
-- 插入参数（NodeAndNodeParameterMapper.xml:38-44）
INSERT INTO tb_node_parameter (id,name,code,hcode,unit)
VALUES (#{id},#{name},#{code},#{hcode},#{unit})
```

### 文件位置

| 文件 | 行号 |
|------|------|
| Controller: `3sai-admin/.../NodeAndNodeParameterController.java` | 66-68 |
| Mapper XML: `3sai-admin/.../NodeAndNodeParameterMapper.xml` | 38-44 |

---

## 4. 绑定参数到船只

| 属性 | 值 |
|------|-----|
| **请求路径** | `/parameter/bindingParameterAndNode` |
| **请求方式** | POST |
| **Content-Type** | application/json |

### 入参

```json
{
    "nodeId": "1656376537357",
    "npId": "1656377652016"
}
```

### 相关SQL

```sql
-- 绑定参数（NodeAndNodeParameterMapper.xml:65-73）
INSERT INTO tb_node_and_node_parameter (id,nodeid,npid,removecode)
VALUES (#{nodeIdAndParId},#{nodeId},#{npId},1)
```

### 文件位置

| 文件 | 行号 |
|------|------|
| Controller: `3sai-admin/.../NodeAndNodeParameterController.java` | 87-89 |
| Mapper XML: `3sai-admin/.../NodeAndNodeParameterMapper.xml` | 65-73 |

---

## 5. 解绑参数

| 属性 | 值 |
|------|-----|
| **请求路径** | `/parameter/unbindParameterAndNode` |
| **请求方式** | POST |
| **Content-Type** | application/json |

### 入参

```json
{
    "nodeId": "1656376537357",
    "npId": "1656377652016"
}
```

### 相关SQL

```sql
-- 解绑参数（NodeAndNodeParameterMapper.xml:75-79）
UPDATE tb_node_and_node_parameter
SET removecode=0
WHERE nodeid=#{nodeId} AND npid=#{npId}
```

### 文件位置

| 文件 | 行号 |
|------|------|
| Controller: `3sai-admin/.../NodeAndNodeParameterController.java` | 96-98 |
| Mapper XML: `3sai-admin/.../NodeAndNodeParameterMapper.xml` | 75-79 |
