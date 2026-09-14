# 区块任务接口文档

## 1. 按ID查询

| 属性 | 值 |
|------|-----|
| **请求路径** | `/blockPlan/queryBlockPlanById` |
| **请求方式** | POST |

### 入参: Query参数 `?id=xxx`

### SQL
```sql
SELECT id,boat_id,task_name,create_address,course_spacing,course_angle,
       complete_action,add_time,update_time,task_attribute,
       route,polygon,water_points,time_spans
FROM block_plan WHERE id=#{id}
```

### 文件: `BlockPlanMapper.xml` 行18-28

## 2. 按船只ID查询

| 属性 | 值 |
|------|-----|
| **请求路径** | `/blockPlan/queryBlockPlanByBoatId` |
| **请求方式** | POST |

### 入参: Query参数 `?boatId=xxx`

### SQL
```sql
SELECT id,boat_id,task_name FROM block_plan WHERE boat_id=#{boat_id} ORDER BY add_time DESC
```

### 文件: `BlockPlanMapper.xml` 行29-38
