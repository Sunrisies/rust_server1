# 模块管理接口文档

## 1. 查询所有模块

| 属性 | 值 |
|------|-----|
| **请求路径** | `/module/queryAll`（前端调用 `/admin/module/queryAll`） |
| **请求方式** | POST |

### 出参

```json
{
    "code": 1,
    "msg": "",
    "data": [
        {"id":"1656395208299","modulename":"无人船管理平台","moduledesc":"...","moduleico":"http://...","routeurl":"/shipControl","removecode":"1"}
    ]
}
```

### SQL
```sql
SELECT id,modulename,moduledesc,moduleico,routeurl,removecode FROM tb_module WHERE removecode = '1'
```

### 文件: `ModuleMapper.xml` 行4-18
