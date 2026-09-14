# APP管理接口文档

## 1. 查询所有APP

| 属性 | 值 |
|------|-----|
| **请求路径** | `/app/queryAll`（前端调用 `/admin/app/queryAll`） |
| **请求方式** | POST |

### SQL
```sql
SELECT id,appdesc,apptype,version,downloadurl,createtime,updatetime,removecode
FROM tb_app_record WHERE removecode='1'
```

### 文件: `AppRecordMapper.xml` 行4-10

## 2. 查询最新APP版本

| 属性 | 值 |
|------|-----|
| **请求路径** | `/app/queryNewVersion` |
| **请求方式** | POST |

### 入参: `{"apptype":"1","createtime":"2024-01-01 00:00:00"}`

### SQL
```sql
SELECT id,appdesc,apptype,version,downloadurl,createtime,updatetime,removecode
FROM tb_app_record WHERE apptype=#{apptype} AND createtime>#{createtime} AND removecode='1'
```

### 文件: `AppRecordMapper.xml` 行30-33
