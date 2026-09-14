# 船坞任务接口文档

## 1. 创建船坞任务

| 属性 | 值 |
|------|-----|
| **请求路径** | `/mavlinkOperation/addDockyard` |
| **请求方式** | POST |

### 入参

```json
{
    "boat_id": "1728ad",
    "name": "任务名称",
    "is_out": false,
    "is_stop": true,
    "start_time": ["2024-01-01 10:00:00"],
    "air_route": ["route1","route2"],
    "target_destination": [["116.62","39.90"]],
    "state": "1"
}
```

### 存储: Redis Hash
- Key: `DOCKYAR_YEY_36`
- Field: `boat_id`
- Value: Map<taskId, DockyardNew>

### 文件: `3sai-repeater/.../MavlinkOperationController.java` 行30-31
