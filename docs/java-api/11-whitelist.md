# 白名单管理接口文档

## 1. 查询所有白名单

| 属性 | 值 |
|------|-----|
| **请求路径** | `/whitelist/queryAll` |
| **请求方式** | POST |

### 出参

```json
{
    "code": 1,
    "msg": "",
    "data": [
        {"ipAddress":"192.168.1.1","name":"测试","username":"admin","token":"","desc":"测试白名单","createTime":"2024-01-01 10:00:00"}
    ]
}
```

### 存储: Redis Hash，key=`IPWhiteList`

### 文件: `3sai-admin/.../WhileManagerController.java` 行88-120
