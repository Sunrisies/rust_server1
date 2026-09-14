# 船只功能管理接口文档

## 1. 查询功能列表

| 属性 | 值 |
|------|-----|
| **请求路径** | `/nodeFuncation/queryNodeFuncation` |
| **请求方式** | POST |

### 入参

```json
{
    "keywords": "搜索词",     // 可选
    "type": "1",              // 可选
    "on": "1",                // 可选
    "page": 1,
    "limit": 10
}
```

### SQL
```sql
SELECT id,name,type,code,remark,`on` FROM tb_node_function
WHERE (name LIKE '%...%' OR remark LIKE '%...%') AND type=#{type} AND `on`=#{on}
ORDER BY id DESC
```

### 文件: `NodeFunctionMapper.xml` 行38-58

## 2. 根据船只ID查询功能

| 属性 | 值 |
|------|-----|
| **请求路径** | `/nodeFuncation/queryNodeFuncationById` |
| **请求方式** | POST |

### 入参: `{"id":"1656376537357"}`

### SQL
```sql
SELECT f.id,f.name,f.type,f.code,f.remark,f.`on`
FROM tb_node n JOIN tb_node_and_function nf ON n.id=nf.nid JOIN tb_node_function f ON nf.fid=f.id
WHERE nf.nid=#{id} AND nf.removecode=1
```

### 文件: `NodeFunctionMapper.xml` 行15-23
