# Java / Rust 接口迁移状态

更新时间：2026-09-12

## 结论

当前 Rust 项目可以编译，`src/routes.rs` 中有 **81 条路由**。但这 81 条路由不能全部视为 Java 等价接口：部分只是路径存在，部分请求方式、参数来源、SQL、字段或返回结构仍不一致。

按当前 Java Controller 和管理端实际调用核对：

- **完全确认与 Java 一致：0 个**
- **已存在且部分行为已对齐：约 20 个**
- **已存在但与 Java 存在明确差异：约 59 个**
- **Java 接口缺失或当前未挂载：至少 20 个**
- Rust 当前路由数量：**81 个**
- Java API 文档列出的接口数量：**90 个**
- Java Controller 实际还包含 API 文档未完整统计的 MQTT、MapLink、视频等接口，因此不能只用 `81/90` 作为完成率。

## 统计口径

Java Controller 的类级路径没有 `/admin`，线上管理端通过网关暴露为 `/admin/...`；登录前端实际调用 `/login/register/...`。因此路径比较采用以下规则：

- Java `/user/*` 对应 Rust/前端 `/admin/user/*`
- Java `/node/*` 对应 Rust/前端 `/admin/node/*`
- Java `/parameter/*` 对应 Rust/前端 `/admin/parameter/*`
- Java `/register/*` 对应前端 `/login/register/*`
- `/blockPlan`、`/mavlinkOperation`、`/mango`、`/mapObsPlan`、`/mqttmessage` 是否需要 `/admin`，需要以网关路由配置为最终准则，当前 Rust 与前端并未统一。

## 模块状态

| 模块 | Java Controller 接口 | 当前 Rust 路由 | 状态 | 主要问题 |
|---|---:|---:|---:|---|
| 认证 | 4 | 4 + 注册兼容路由 | 部分一致 | RSA 密钥持久化、Token 校验、权限缓存、Header token、响应细节仍需验证 |
| 用户管理 | 15 | 13 | 明确不一致 | 缺少批量关系接口；User 字段、modulenames、时间、SQL 和 BaseResultResponse 未完全一致 |
| 船只管理 | 14 | 8 | 明确不一致 | 缺少 3 个兼容接口和高级查询；Node 字段、动态 SQL、逻辑删除未完全一致 |
| 船只功能 | 8 | 7 | 明确不一致 | 缺少按 taskId 查询；字段、分页、删除关系和返回未完全一致 |
| 船只权限 | 7 | 7 | 明确不一致 | 字段、分页、重复关系检查和返回未完全一致 |
| 参数管理 | 8 | 7 | 明确不一致 | 缺少 `queryParameterByNode`；两个查询被复用；JOIN、unit、删除语义不一致 |
| 系统权限 | 4 | 4 | 明确不一致 | Java Permission 字段、动态条件、删除语义和返回未完成一致性验证 |
| 模块管理 | 7 | 7 | 明确不一致 | Module 字段、动态条件和权限关联返回未完全一致 |
| 白名单 | 3 | 3 | 明确不一致 | Rust 查询固定空数组，未实现 Java Redis Hash 读取和时间排序 |
| APP 管理 | 4 | 4 | 明确不一致 | Java 版本条件、DATETIME、字段和返回细节未完全一致 |
| 区块任务 | 5 | 4 | 明确不一致 | 缺少 `updateBlockPlan`；删除/查询参数应为 Query 参数，字段和校验不一致 |
| 船坞任务 | 5 | 5 | 明确不一致 | Redis 数据结构、查询、任务名校验、调度和字段不一致 |
| MongoDB | 4 | 4 | 未完成 | Rust handler 仍是占位实现，没有接入 MongoDB 查询服务 |
| 文件上传 | 3 | 3 | 明确不一致 | 配置、目录、文件校验、APP 类型和静态访问未完全一致 |
| 地图观测 | 1 | 0 | 未实现/未挂载 | 缺少 Java 参数和 Native/Service 算法迁移 |
| MQTT 消息 | 6 | 0 | 未实现 | 缺少 MQTT 客户端、任务调度、消息格式和设备通信 |
| MapLink | 5 | 0 | 未实现/未纳入当前 Rust | Java Controller 存在，当前 Rust 没有对应模块 |
| 视频 YS7 | 1 | 0 | 未实现/未纳入当前 Rust | Java Controller 存在，当前 Rust 没有真实视频服务实现 |

## 当前 Rust 已挂载的 81 条路由

### 认证

- `/login/register/login`
- `/login/register/getPublicKey`
- `/login/register/verifyToken`
- `/login/register/logout`
- `/login/register/register`（Java `LoginController` 没有该接口，属于 Rust 额外接口）

### 用户

- `/admin/user/queryAll`
- `/admin/user/add`
- `/admin/user/update`
- `/admin/user/deleteUser`
- `/admin/user/update/account/profile`
- `/admin/user/update/account/password`
- `/admin/user/update/account/profileAndPasswrd`
- `/admin/user/bindBoat`
- `/admin/user/notBindboat`
- `/admin/user/bindCommonPermission`
- `/admin/user/notBindCommonPermission`
- `/admin/user/queryUserByUserName`
- `/admin/user/queryUserByUserNameX`

缺少：

- `/admin/user/updateAutAndOrgAndRoleAndNodeByUser`
- `/admin/user/addUserAndRoleAndParim`

### 船只

- `/admin/node/queryAllNode`
- `/admin/node/queryNodeAll`
- `/admin/node/queryNodeByUser`
- `/admin/node/addNode`
- `/admin/node/updataNode`
- `/admin/node/deteleNode`
- `/admin/node/updateNodeLocation`
- `/admin/node/queryNodeStatusByTcpport` 当前路由版本需以源文件为准核验，前端调用方式与 Java 参数 `tcpPorts` 仍不一致

缺少：

- `/admin/node/updataNode2`
- `/admin/node/updataNodeGet`
- `/admin/node/deteleNodeGet`
- `/admin/node/queryAllByUserAndOrganization`
- `/admin/node/queryNodeAndFunctionByUserId`
- `/admin/node/queryOnlineDataByUser`

### 参数

当前已挂载 7 个路径，但 Java 有 8 个接口。缺少或未正确绑定：

- `/admin/parameter/queryParameterByNode`

### 船只功能

当前已挂载 7 个路径。缺少：

- `/admin/nodeFuncation/queryNodeFuncationByTaskId`

### 业务模块未统一挂载

当前 Rust 路由中的以下模块没有统一使用前端的 `/admin` 前缀，可能导致前端 404：

- `/blockPlan/*`
- `/mavlinkOperation/*`
- `/mango/*`
- `/mapObsPlan/*`
- `/mqttmessage/*`
- `/fileserver/upload/*`

## 已确认的响应差异

### 统一响应

Java 的 `LayUiTableTemplateLay` 字段是：

```json
{
  "code": 1,
  "msg": "",
  "data": null,
  "count": null,
  "obj": null
}
```

当前 Rust 仍存在以下差异：

- 部分查询接口没有 `count`。
- 部分接口把 `count` 设置成当前页长度，而 Java 使用总记录数。
- 部分接口使用 `message` 或不同消息内容。
- 部分接口返回 `ApiResponse`，部分直接返回 JSON，格式没有统一。
- Java 的绑定接口使用 `BaseResultResponse`，字段是 `success/type/message/data/count`，不能用 `code/msg` 替代。
- Java `queryUserByUserName` 返回裸 `List<User>`，不能统一包成 `LayUiTableTemplateLay`。

### 请求方式和参数来源

- Java 管理 Controller 的大多数操作是 `@RequestMapping`，线上管理端通过 POST 调用。
- Java `@RequestBody` 参数必须从 JSON body 读取。
- Java `@RequestParam` 参数必须从 query string 读取。
- `updataNode2/updataNodeGet/deteleNodeGet` 使用普通请求参数，不是 JSON body。
- `queryDockyard/querySingleDockyard/deleteDockyard` 使用 query 参数。
- `verifyToken` 使用 query 参数 `token`。
- `logout` 使用 request header `token`。
- Rust 当前有多处把 Java query 参数实现成 JSON body，或把 Java 的 JSON body实现成其他形式。

## 当前最重要的行为差异

1. Rust 当前不是所有 Java Controller 接口都有对应路由。
2. Rust 多个模块的 handler 仍是简化实现或占位实现。
3. 船只查询没有完整返回 Java `Node` 字段，也没有完整动态 SQL。
4. 用户查询尚未生成 Java Service 中的 `modulenames`，且全量字段和时间格式需要继续核验。
5. 船坞任务没有复现 Java 的 Redis Hash 结构 `DOCKYAR_YEY_36 -> boat_id -> task map`。
6. MongoDB handler 没有调用 `src/mongodb.rs`，当前查询结果不能代表线上 Java 查询。
7. 白名单查询没有读取 Redis 中已有数据。
8. MQTT、地图观测、MapLink、YS7 视频模块尚未完成等价迁移。
9. 当前存在多个历史 handler 和兼容路由版本，必须清理重复绑定，避免修改一个 handler 但实际请求命中另一个 handler。

## 建议的验证标准

每个模块完成后必须同时满足：

1. Java Controller 与 Rust route 的路径、方法、参数来源一致。
2. Java 实体字段与 Rust 请求/响应字段逐项一致。
3. Java Mapper SQL 与 Rust SQL 的筛选条件、JOIN、排序、分页、删除语义一致。
4. Java 与 Rust 使用同一组请求参数进行 JSON 快照比较。
5. 写操作使用隔离测试数据，并验证影响行数及数据库最终状态。
6. 完成后再把模块状态从“部分一致”改为“已验证一致”。

## 当前判断

当前项目属于：

> **已完成模块拆分，已迁移部分业务接口，当前可编译，但尚未达到 Java 版本的全接口等价兼容。**

不能将当前状态描述为“全部接口已经与 Java 一样”。
