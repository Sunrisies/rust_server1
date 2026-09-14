# 权限管理接口文档

## 1. 查询所有权限

| 属性 | 值 |
|------|-----|
| **请求路径** | `/permission/queryAll`（前端调用 `/admin/permission/queryAll`） |
| **请求方式** | POST |

### 出参

```json
{
    "code": 1,
    "msg": "",
    "data": [
        {"id":"1656394924310","mname":"blockPlan","mdesc":null,"uri":"/admin/blockPlan","method":"*","parentid":"","removecode":"1"}
    ]
}
```

### SQL
```sql
SELECT id,mname,mdesc,uri,method,parentid,removecode FROM tb_permission WHERE removecode='1'
```

### 文件: `PermissionMapper.xml` 行4-24
