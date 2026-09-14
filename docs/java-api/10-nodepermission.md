# 船只权限管理接口文档

## 1. 查询所有权限

| 属性 | 值 |
|------|-----|
| **请求路径** | `/nodePermissions/queryNodePermissionsAll` |
| **请求方式** | POST |

### SQL
```sql
SELECT id,name,code,details FROM tb_node_jurisdiction WHERE (name LIKE '%...%' OR details LIKE '%...%')
```

### 文件: `NodePermissionMapper.xml` 行14-34

## 2. 根据船只ID查询权限

| 属性 | 值 |
|------|-----|
| **请求路径** | `/nodePermissions/queryNodePermissionsByNodeId` |
| **请求方式** | POST |

### SQL
```sql
SELECT j.id,j.name,j.code,j.details
FROM tb_node_and_jurisdiction nj JOIN tb_node_jurisdiction j ON nj.jurid=j.id
WHERE nj.nodeid=#{id}
```

### 文件: `NodePermissionMapper.xml` 行4-12
