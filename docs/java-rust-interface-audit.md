# Java 到 Rust 接口契约审计

> 审计目的：将当前 Rust 迁移版本与 `/tmp/hover_temp` 中 Java `zhudep` 分支进行对照，检查请求方式、请求参数、SQL、响应结构和业务行为。
>
> 本文只记录已经从 Java Controller、实体类、MyBatis XML 和当前 Rust 源码确认的事实。`通过编译`或`返回 code=1`不代表接口已经与 Java 版本兼容。

## 1. 审计范围与基准

### Java 基准

- Java 工程：`hover`，`zhudep` 分支
- 管理端 Controller：`3sai-admin`
- 登录认证：`3sai-ucenter`
- 船只操作：`3sai-repeater`
- 文件上传：`3sai-fileserver`
- 地图观测：`3sai-mapobsplan`
- SQL 基准：`3sai-admin/src/main/resources/com/hongwanweipeng/mapping/*.xml`
- 统一响应：`LayUiTableTemplateLay<T>`

Java 统一响应类的字段为：

```json
{
  "code": 1,
  "msg": "",
  "data": null,
  "count": null,
  "obj": null
}
```

其中：

- `code=1` 表示成功，`code=0` 表示业务失败。
- `msg` 是消息字段，不是 `message`。
- `count` 是分页总数，不是当前页数组长度。
- `obj` 是扩展字段，Java 序列化时通常仍会输出 `null`。

### 当前 Rust 基线

当前 Rust 代码已经拆分到 `src/handlers/`，路由集中在 `src/routes.rs`。当前 `cargo check` 可以通过，但仍有大量未使用代码警告。

当前可见路由数量与 Java Controller 接口数量不能直接比较，因为：

- 部分 Rust 路由仍使用旧的 `/admin` 前缀方案，部分路由没有 `/admin`。
- 部分 Java 接口没有被挂到当前路由。
- 部分 Rust handler 仍是简化或占位实现。
- 部分 Rust 路由绑定到了旧 handler，而不是最近新增的模块 handler。

## 2. 总体结论

当前迁移版本**不能认定为 Java 版本的兼容替代品**。主要差异如下：

| 类别 | 当前状态 | 严重性 |
|---|---|---|
| 路由路径 | `/admin/*`、无前缀路径、旧路径并存 | 高 |
| HTTP 方法 | Java 管理接口基本使用 POST；Rust 部分曾使用 GET/PUT/DELETE，当前仍需逐条校验 | 高 |
| 统一响应 | 查询接口部分缺少 `count`；部分接口消息不一致；部分错误响应结构不同 | 高 |
| 分页 | Rust 多处将当前页长度作为 `count`，未查询总数 | 高 |
| 用户查询 | 字段不完整，过滤条件与 Java 不一致，未生成 `modulenames` 和时间字段 | 高 |
| 船只查询 | 当前只返回少量字段，缺少 Java `Node` 的完整字段和条件查询 | 高 |
| 参数查询 | `queryParameter` 与 `queryParameterAll` 被错误复用，字段和关联查询不一致 | 高 |
| 船坞任务 | 查询实现为空或 Redis key 结构不同，不能兼容 Java 数据 | 高 |
| MongoDB | 当前 handler 是占位实现，尚未接入已写的 MongoDB 服务 | 高 |
| 白名单 | 查询返回空数组，没有实现 Java Redis Hash 遍历 | 高 |
| 认证 | RSA、Token、Session 仍需严格核对；Rust 允许明文回退，Java 不允许 | 高 |
| 文件上传 | 路径、文件校验、返回字段和静态文件服务未完全匹配 | 中 |
| MQTT | 当前 Rust 侧没有等价实现 | 高 |
| 地图观测 | 当前 Rust 侧没有等价实现或只是占位 | 高 |

## 3. 认证模块

### Java 接口

Controller：`3sai-ucenter/.../LoginController.java`

| 接口 | Java 方法 | Java 入参 | Java 出参 |
|---|---|---|---|
| `/register/login` | POST/`@RequestMapping` | `@RequestBody LoginMessage` | `LayUiTableTemplateLay<LoginReturnMessage>` |
| `/register/getPublicKey` | GET/`@RequestMapping` | 无 | `code=1,msg="公钥信息",data=Base64 DER 公钥,obj="key"` |
| `/register/verifyToken` | 请求参数 `token` | `String token` | `BaseResultResponse`，字段为 `success/message/data` |
| `/register/logout` | 请求头 `token` | Header `token`，不是 JSON body | `LayUiTableTemplateLay`，`code=1,data="登出成功！"` |

`LoginMessage` 字段：

```text
userName
passWord
verifyCode
userKey
```

登录成功的 `data` 为：

```text
userId
userName
permission
modules
token
nickName
logo
history_url
realtime_url
workspaceId
```

### Java 登录行为

1. `userName` 和 `passWord` 为空时返回 `code=0,msg="参数异常..."`。
2. `passWord` 始终使用 RSA 私钥解密；解密失败返回 `code=0,msg="解析异常..."`。
3. 查询用户，并要求结果数量为 1。
4. 使用 BCrypt 校验解密后的密码。
5. Token 明文内容为：`时间戳,HOVERWEAPON,权限字符串`。
6. 使用 RSA 公钥加密上述内容，Token 返回 Base64 字符串。
7. Session 写入 Redis：`login:user:{token}`，过期时间 1800 秒。
8. 根据用户 `moduleids` 查询有效模块。
9. 根据模块关联查询权限并缓存到 `login:permission:{username}`。
10. 成功消息为 `获取成功`。

### 当前 Rust 差异

文件：`src/handlers/auth.rs`、`src/rsa_util.rs`、`src/session.rs`

- 请求结构曾只定义 `username/password`；Java `LoginMessage` 还包含 `verifyCode/userKey`，这两个字段应保留兼容。
- Java 对 `passWord` 始终执行 RSA 私钥解密；Rust 曾根据长度猜测，并在解密失败时回退明文，行为不一致。
- Java RSA 密钥对生成后写入 Redis `resKetContest/keyContext`；Rust 当前每次进程启动随机生成，重启后旧公钥加密的密码和旧 Token 无法解密。
- Java `getPublicKey` 返回 Base64 编码的 X509 DER 公钥，响应为 `code=1,msg="公钥信息",data=<base64>,obj="key"`；Rust 当前曾返回 PEM，格式不一致。
- Java 登录查询模块权限并写入 `login:permission:{username}`；Rust 当前 `permission` 固定为空数组，未完整执行模块权限查询和缓存。
- Java Token 明文为 `timestamp,HOVERWEAPON,jurisdiction`，再用 RSA 公钥加密；Session key 为 `login:user:{token}`，TTL 为 1800 秒。Rust 需要保持密钥持久化及 Redis key/TTL 约定。
- Java `verifyToken` 接收请求参数 `token`，RSA 解密后校验 `HOVERWEAPON` 和 7200000ms 过期时间；Rust 当前主要检查 Session，校验逻辑不完整。
- Java `logout` 从请求头 `token` 读取；Rust 当前主要从 query 参数读取，参数来源不一致。
- Java 登录成功外层必须包含 `code/msg/data/count/obj`，其中 `msg="获取成功"`，`count=null,obj=null`；认证模块需要保留这两个 null 字段。

## 4. 用户模块

### Java SQL 基准

`ManageUserMapper.xml` 的条件查询字段：

```sql
SELECT id, username, moduleids, logo, phone, created, updated, nickname, img,
       remarks, user_type AS userType, workspace_id AS workspaceId,
       mqtt_username AS mqttUsername, mqtt_password AS mqttPassword,
       history_url, realtime_url
FROM tb_user
WHERE ... AND removecode = 1
```

Java 查询条件包括：

- `username = ?`
- `workspace_id = ?`
- `moduleids LIKE '%...%'`
- `phone = ?`
- `created = ?`
- `updated = ?`
- `nickname = ?`
- `keywords` 同时匹配 `username/nickname/remarks`
- 永远附加 `removecode=1`

Java `queryAllUser` 使用 PageHelper：

- 默认 `page=1, limit=10`
- `data` 是当前页
- `count` 是 PageInfo 的总记录数
- 每个用户补充 `createTime/updateTime`
- 根据 `moduleids` 查询并拼接 `modulenames`

### Java 用户接口契约

| 接口 | Java 入参 | Java 行为 |
|---|---|---|
| `/user/add` | `User` JSON 全字段 | 检查用户名；由 Service/Mapper 写入完整 User 字段 |
| `/user/update` | `User` JSON 全字段 | 检查用户名唯一性；更新完整用户字段 |
| `/user/update/account/profile` | `User` JSON | 与 update 相同的用户名检查和更新逻辑 |
| `/user/update/account/password` | `UserPassword{id,oldPassword,newPassword}` | 校验旧密码后更新新密码 |
| `/user/update/account/profileAndPasswrd` | `User` JSON | 同时更新资料和密码 |
| `/user/deleteUser` | `User` JSON | Java Mapper 删除用户记录 |
| `/user/queryAll` | `User` JSON | 条件查询、分页、总数、模块名称和时间补充 |
| `/user/queryUserByUserName` | 普通请求参数 `username` | 直接返回 `List<User>`，不是统一包装对象 |
| `/user/queryUserByUserNameX` | 普通请求参数/`User` | 返回统一包装，`data` 为用户查询结果 |
| `/user/updateAutAndOrgAndRoleAndNodeByUser` | `UserTable` | 更新角色、机构、船只、权限关系 |
| `/user/addUserAndRoleAndParim` | `UserTable` | 新增用户及角色、机构、船只、权限关系 |
| `/user/bindCommonPermission` | `UserAndCommonPermission` | 返回 `BaseResultResponse` |
| `/user/notBindCommonPermission` | `UserAndCommonPermission` | 返回 `BaseResultResponse` |
| `/user/bindBoat` | `UserAndNode` | 返回 `BaseResultResponse` |
| `/user/notBindboat` | `UserAndNode` | 返回 `BaseResultResponse` |

`User` 字段包括：

```text
id, username, moduleids, modulenames, password, newPassword, oldPassword,
logo, phone, created, updated, nickname, img, page, limit, remarks,
removecode, keywords, createTime, updateTime, userType, workspaceId,
mqttUsername, mqttPassword, history_url, realtime_url
```

### 当前 Rust 差异与本轮修正

文件：`src/handlers/user.rs`

本轮已完成：

- 查询 SQL 已使用 Java `queryUserBycondition` 的主要字段集合，包括 `moduleids/logo/phone/created/updated/nickname/img/remarks/userType/workspaceId/mqttUsername/mqttPassword/history_url/realtime_url`。
- 查询固定过滤 `removecode=1`。
- `keywords` 已同时匹配 `username/nickname/remarks`。
- `count` 已单独执行无分页 `COUNT(*)`，不再返回当前页长度。
- `queryUserByUserName` 已改为 Java 的裸数组响应，而不是统一包装。
- 更新用户名时已增加“不能被其他用户占用”的检查。
- `update/account/password` 已使用旧密码校验后再 BCrypt 更新。

仍需继续修正：

- `modulenames` 尚未像 Java Service 一样根据 `moduleids` 查询 `tb_module` 并拼接。
- `created/updated` 需要在数据库时间类型和 Java `yyyy-MM-dd HH:mm:ss` 序列化之间做快照验证。
- `add/update` 仍只写入部分 User 字段；Java Mapper 支持 `moduleids/password/created/img/logo/remarks/removecode/userType/workspaceId/mqttUsername/mqttPassword/history_url/realtime_url` 的动态更新。
- Rust 使用 UUID 生成用户 ID；Java 使用 Service 中的 ID 生成策略，需与线上数据策略确认后再改。
- `addUserAndRoleAndParim`、`updateAutAndOrgAndRoleAndNodeByUser` 尚未按 Java `UserTable` 扁平字段和事务逻辑实现。
- `bindCommonPermission/notBindCommonPermission`、`bindBoat/notBindboat` 需要对齐 Java `BaseResultResponse`，不能只返回 `code/msg`。
- `queryUserByUserNameX` 的参数来源应按 Java Controller 的普通参数绑定方式验证；当前 Rust 使用 JSON body。
- 当前路由统一为 `/admin/user/*` 以兼容管理端，但 Java Controller 原始路径是 `/user/*`，部署网关前缀需要最终确认。

## 5. 船只模块

### Java `Node` 字段

Java 查询返回实体 `Node`，字段远多于当前 Rust 返回：

```text
id, sname, ip, port, video,
url, urlname, url2, urlname2, url3, urlname3,
sonorurl1name, sonorurl1, sonorurl2name, sonorurl2,
imei, state, taskid, message, removecode, charge,
functions, jurisdictions, nodeParameters, tcpPort, describes,
remote_port, fourthGCardNumber, fourthGCardNumberExpirationTime,
rtkcardNumber, rtkcardNumberExpirationTime,
createTime, latitude, longitude
```

### Java 查询行为

- `/node/queryAllNode`：`LayuiNodeTable` 条件查询，默认 `page=1,limit=10`，PageHelper 分页，`count` 为总数。
- 查询条件由 `ManageNodeMapper.xml` 中动态 SQL 决定，至少包括 ID、名称、用户、关键词和删除标记等条件。
- `/node/queryNodeAll`：查询全部无分页，但仍返回 Java 统一包装。
- `/node/queryNodeByUser`：接收 `User`，按用户关联表查询船只。
- `/node/queryAllByUserAndOrganization`：按用户/机构关联查询。
- `/node/queryNodeAndFunctionByUserId`：查询船只后继续补充视频、Redis 在线状态、功能、船只权限、参数和位置。
- `/node/queryNodeStatusByTcpport`：调用转发服务检查 TCP 端口在线状态，不是简单读取数据库 `state`。
- `/node/updateNodeLocation`：按 Java `NodeMapper.xml` 的更新 SQL 处理位置，字段关系必须核对。
- `addNode` 会检查同名船只，使用时间戳 ID，设置 `state=1`、`removecode=1`。
- `updataNode`、`deteleNode` 使用 `LayuiNodeTable`，删除接口实际走更新逻辑将 `removecode` 设为 `0`。
- `updataNode2/updataNodeGet/deteleNodeGet` 的参数来源不是 JSON body，而是普通请求参数。

### 当前 Rust 差异

文件：`src/handlers/node.rs`

- `query_all` 当前只查询：`id,sname,ip,port,state,taskid,describes`，缺少 Java Node 的绝大多数字段。
- 当前 `query_all` 不读取 `page/limit` 条件，也没有 Java 的动态条件 SQL。
- 当前 `queryNodeAll` 与 `queryAllNode` 绑定到同一个查询 handler，无法表达 Java 的“分页查询”和“无分页查询”区别。
- 当前 `query_by_user` 只接收 `id/userId`，Java 接收的是 `User`，且应按 Java Mapper 的用户条件查询。
- 当前 `queryAllByUserAndOrganization`、`queryNodeAndFunctionByUserId`、`queryOnlineDataByUser` 在当前路由中没有完整绑定到模块 handler。
- 当前没有 `updataNode2`、`updataNodeGet`、`deteleNodeGet` 三个 Java 接口。
- 当前删除接口曾使用硬删除；Java `deteleNode` 是逻辑删除，应改为 `removecode=0`。
- 当前添加船只没有同名检查，ID 生成策略也与 Java 不一致。
- 当前位置更新插入一条新记录，Java 代码存在按设备 ID 更新/查询位置的逻辑，不能直接假设为 UUID 插入。

## 6. 参数模块

### Java 基准

Controller：`NodeAndNodeParameterController.java`

| 接口 | Java 入参 | Java 出参/行为 |
|---|---|---|
| `/parameter/queryParameter` | `LayuiNodeTable` JSON | 条件查询参数，统一包装 |
| `/parameter/queryParameterAll` | 无参数 | 查询全部 `NodeParameter` |
| `/parameter/updateParameter` | `NodeParameter` JSON | 更新参数 |
| `/parameter/deleteParameter` | `NodeParameter` JSON | 删除参数 |
| `/parameter/addParameter` | `NodeParameter` JSON | 新增参数 |
| `/parameter/queryParameterByNode` | `LayuiNodeTable` JSON | 通过关联表查询船只参数 |
| `/parameter/bindingParameterAndNode` | `NodeParameterAndNode` JSON | 新增关联 |
| `/parameter/unbindParameterAndNode` | `NodeParameterAndNode` JSON | 更新关联删除标记，需以 XML 为准 |

Java `NodeParameter` 字段至少包括：

```text
id, name, code, hcode, removecode, unit, createTime
```

Java `queryParameter` 的 SQL 来自 `NodeAndNodeParameterMapper.xml`，不是简单查询中间表：

```sql
SELECT id,name,code,hcode,unit FROM tb_node_parameter ...
```

`queryParameterByNode` 使用 `tb_node_parameter` 与 `tb_node_and_node_parameter` 的 JOIN。

### 当前 Rust 差异

文件：`src/handlers/parameter.rs`

- `/admin/parameter/queryParameter` 与 `/admin/parameter/queryParameterAll` 当前绑定到同一个 `parameter::query`，不符合 Java 两个接口的入参和查询区别。
- 当前参数查询返回中间表 `id,node_id,npid` 或简单参数字段，不能覆盖 Java `NodeParameter` 结构。
- 缺少 `unit`、`removecode`、`createTime` 等字段。
- `unbind` 当前执行物理删除；Java XML 使用更新语句，必须保留 Java 的删除标记语义。
- 参数添加、更新、删除的返回体和影响行数处理尚未按 Java Service 对齐。

## 7. 船只功能模块

### Java 基准

实体 `NodeFunction`：

```text
id, taskid, name, remark, code, type, page, limit, on, keywords, createTime
```

接口全部使用 JSON body，Controller 上的 `@RequestMapping` 实际由网关暴露为管理端路径：

```text
addNodeFuncation
updateNodeFuncation
deleteNodeFuncation
queryNodeFuncation
queryNodeFuncationById
queryNodeFuncationByTaskId
addNodeFunAndNode
deleteNodeFunAndNodeByNodeIdAndFunId
```

Java SQL：

- 船只关联查询通过 `tb_node -> tb_node_and_function -> tb_node_function` JOIN。
- 任务查询根据 `tb_node.taskid` 关联。
- 删除功能还需要核对 Java Service 是否同步处理关联表。

### 当前 Rust 差异

文件：`src/handlers/node_function.rs`

- 当前实体/返回字段没有完整覆盖 `taskid/page/limit/keywords/createTime`。
- 查询分页和 `count` 尚未按 Java Service/PageHelper 对齐。
- `queryNodeFuncationById`、`queryNodeFuncationByTaskId` 需要逐条核对 Java JOIN 和参数对象，不能只按一个字符串字段实现。
- 绑定/解绑关系的重复检查、影响行数、错误消息与 Java 不一定一致。
- `src/routes.rs` 当前缺少 `queryNodeFuncationByTaskId` 路由。

## 8. 船只权限模块

### Java 基准

实体 `NodePermission`：

```text
id, code, message, name, details, page, limit, keywords, createTime
```

关联实体 `NodeAndPermission`：

```text
id, nodeid, jurid
```

当前 Java Mapper 使用：

- `tb_node_jurisdiction`
- `tb_node_and_jurisdiction`
- 船只与权限的 JOIN 查询
- 权限删除及关联删除逻辑

### 当前 Rust 差异

文件：`src/handlers/node_permission.rs`

- 当前返回结构主要只有 `id/name/code/details`，缺少 `message/page/limit/keywords/createTime`。
- 查询没有完整实现 Java 的动态筛选和分页。
- 绑定字段名为 `nodeid/jurid`，这一点需要保留；不能与用户绑定的 `nodeId` 混用。
- 关联重复检查、删除语义和 Java Service 的错误消息尚未逐条对齐。

## 9. 系统权限模块

### Java 基准

实体 `Permission` 字段：

```text
id, mname, mdesc, uri, method, parentid, removecode,
page, limit, keywords, createTime
```

Java 查询：

```sql
SELECT id,mname,mdesc,uri,method,parentid,removecode
FROM tb_permission
WHERE ...
```

### 当前 Rust 差异

文件：`src/handlers/permission.rs`

- 查询返回字段缺少 `removecode`、分页查询字段和关键词条件。
- 当前添加/更新/删除只覆盖部分字段。
- 删除语义需要确认：Java Mapper 是物理删除 `delete from tb_permission where id=?`，不能擅自改成逻辑删除。
- 返回消息、`count`、`obj` 需要按 Java Service 的实际设置逐接口核对。

## 10. 模块管理

### Java 基准

实体 `Module` 字段：

```text
id, modulename, moduledesc, moduleico, routeurl, removecode,
page, limit, keywords, createTime, permissionids
```

模块权限关联：

```text
id, mid, pid
```

Java Mapper：

- `queryModuleBycondition` 动态条件查询，过滤 `removecode`。
- `queryById` 查询有效模块。
- 模块权限新增/删除使用关联表。
- `queryModulePermission` 需要根据 Java Service 真实返回对象核对，不能只返回权限基础字段。

### 当前 Rust 差异

文件：`src/handlers/module.rs`

- 当前模块返回缺少 `page/limit/keywords/createTime/permissionids`。
- 查询未实现 Java 动态条件和总数语义。
- 模块删除、权限关联删除的物理/逻辑删除行为需要按 Java XML 对齐。
- 当前模块权限查询只返回部分 Permission 字段，需按 Java Controller 实际数据进行确认。

## 11. 白名单模块

### Java 基准

实体 `WhiteListDTO`：

```text
ipAddress, name, username, token, desc, createTime
```

Java Redis 行为：

- 添加：写入白名单配置，自动设置当前时间。
- 删除：按 IP 删除。
- 查询：读取白名单全部 Hash 值，反序列化 DTO，按 `createTime` 倒序。
- 参数为空时返回 `code=0` 和对应中文错误消息。

### 当前 Rust 差异

文件：`src/handlers/whitelist.rs`

- `query_all` 当前固定返回空数组，没有遍历 Redis，也不会返回线上白名单。
- 当前 Redis key 方案为 `whitelist:{ip}`，Java 使用的是 `IPWhiteList` 的配置和 Hash 方案，必须核对 Java `IPWhiteList` 的真实 key。
- 当前 DTO 缺少 `username/token`。
- 当前没有按 Java 时间倒序。
- 当前添加/删除成功响应字段和 Java 的 `LayUiTableTemplateLay` 需要逐条对齐。

## 12. APP 管理模块

### Java SQL 基准

字段：

```text
id, appdesc, apptype, version, downloadurl, createtime, updatetime, removecode
```

Java Mapper：

- `queryAppBycondition` 条件查询。
- `queryById` 要求 `removecode='1'`。
- `queryNewVersionApp` 条件为 `apptype`、`createtime > ?`、`removecode='1'`。
- Java Controller 的删除接口调用 Service，最终删除策略必须以 Service/Mapper 为准。

### 当前 Rust 差异

文件：`src/handlers/app.rs`

- 查询字段使用 `Option<String>` 读取 MySQL `DATETIME`，这是已出现过的运行时错误来源；应使用 `NaiveDateTime` 或统一日期转换。
- `query_new_version` 当前只是按创建时间倒序取一条，缺少 Java 的 `createtime` 入参条件。
- 当前分页和总数逻辑不完整。
- 当前返回字段缺少 `updatetime/removecode`。
- 当前 `app` 路由已挂载 `/admin/app/*`，但历史代码仍存在旧 handler，必须保证不会误绑定旧实现。

## 13. 区块任务模块

### Java 基准

Controller 接口：

```text
addOrUpdateBlockPlan(BlockPlan JSON)
updateBlockPlan(BlockPlan JSON)
deleteBlockPlan(@RequestParam id)
queryBlockPlanById(@RequestParam id)
queryBlockPlanByBoatId(@RequestParam boatId)
```

Java SQL 来自 `BlockPlanMapper.xml`，包括：

- `countBlockPlan`
- 按任务名称和 ID 查询
- 按 ID 查询完整 BlockPlan
- 按 boat_id 查询
- 新增、删除、更新

### 当前 Rust 差异

文件：`src/handlers/block_plan.rs`

- 当前路由没有 `/admin` 前缀，而前端管理端使用 `/admin/...` 规范时会出现 404。
- Java 删除、按 ID 查询、按船只查询使用 Query 参数；Rust 当前 handler 使用 JSON body。
- 当前 BlockPlan 字段只覆盖少量字段，需以 Java `BlockPlan` 实体和 SQL 全字段为准。
- 当前 `add_or_update` 直接通过是否存在 `id` 判断，Java Service 还有任务名/数量校验，未对齐。
- 当前路由缺少 `updateBlockPlan` 独立接口。

## 14. 船坞任务模块

### Java 基准

接口：

```text
POST /mavlinkOperation/addDockyard      JSON DockyardNew
POST /mavlinkOperation/queryDockyard   Query boatId
POST /mavlinkOperation/querySingleDockyard Query boatId,id
POST /mavlinkOperation/deleteDockyard  Query boatId,id
POST /mavlinkOperation/updateDockyard  JSON DockyardNew
```

Java 使用 Redis Hash：

- Hash key：`DOCKYAR_YEY_36`
- field：`boat_id`
- value：任务 Map，Map key 为任务 ID
- 支持任务名唯一检查、立即执行、定时任务、更新时间排序、单任务查询和删除。

`DockyardNew` 字段：

```text
id, name, is_out, is_stop, start_time, air_route, boat_id, port,
state, target_destination, errorCause, boatName, addDate, updateDate,
taskAttribute
```

### 当前 Rust 差异

文件：`src/handlers/dockyard.rs`

- 当前 Redis key 是 `dockyard:{boat_id}:{task_id}`，与 Java Hash 结构不兼容。
- `query` 当前固定返回空数组，无法读取已创建任务。
- Query 参数与 JSON body 的接口约定未按 Java 分开。
- 当前字段不完整，缺少 `target_destination/errorCause/boatName/addDate/updateDate/taskAttribute` 等。
- 当前没有任务名唯一检查和定时调度。
- 当前成功/失败消息、`count`、`obj` 未按 Java 返回。

## 15. MongoDB 模块

### Java 基准

配置：

```text
ipAddress=47.104.240.65
port=8017
username=root
password=Hover_66weapon
dbName=hr
collectionName=boat1
```

`/mango/find`：

- 从 `HttpServletRequest` 读取 query 参数。
- 支持字段：`taskid,heading_cd,oxygen,NH34,X,ox2,nh5,time,nh4,lon,nh1,boatid,lat,battery_cv,Depth,ox5,nh2,current_ca,ph,Y,ox3,taskname,nh3,speed_cms,temperature,cdv,ox1,Ctmd,timestr,ox4,skip,limit`。
- 支持 `gte/gt/lte/lt/within/正则` 操作符。
- 默认按 `time` 升序，最多 3000 条。
- 支持 `requestFields` projection。

`/mango/findHistoricalData`：

- 额外支持 `datatype/pointname/routename`。
- `datatype=1` 映射 `type=2`。
- `datatype=2` 映射 `type=1`。
- `datatype=3` 增加 `taskid > 0` 和 `reach=1`。
- 支持 `or/` 条件。
- 查询结果移除 `_id`。

`/mango/findNum`：

- 使用相同条件构建查询。
- 查询结果中的 `taskname` 去重后返回 `HashSet<String>`。

`/mango/shell`：

- Java 执行：`sh dev.sh`。
- 工作目录：`/data/3sai/work/`。
- 返回 `data.result=Success/Fail`。

### 当前 Rust 差异

文件：`src/handlers/mango.rs`

- 当前四个 handler 均为占位实现，固定返回空数组或 Success。
- 当前没有使用已经存在的 `src/mongodb.rs` 查询服务。
- 请求参数来源应是 query string，因为 Java 使用 `HttpServletRequest.getParameter`，不能改成只接收 JSON body。
- MongoDB 查询操作符、projection、排序、3000 条限制、历史数据 datatype 逻辑均未接入。
- `shell` 不能用无条件的本地 `echo` 冒充 Java 的脚本执行；应配置工作目录、脚本路径和权限策略。

## 16. 文件上传模块

### Java 基准

Controller：`3sai-fileserver/.../FileController.java`

- `/fileserver/upload/uploadLogo`：Multipart 字段 `file`，图片最大 2 MB，只允许 png/jpg/jpeg/gif。
- `/fileserver/upload/uploadIco`：Multipart 字段 `file`，同样的图片校验。
- `/fileserver/upload/uploadApp`：Multipart 字段 `file` + `appType`，文件规则和存储目录需要以 Java 方法后半段继续核对。
- Java 使用配置项 `local.file.dir` 和 `local.file.path`。
- 返回 `id/type/url` 等字段，路径格式由配置和时间戳决定。

### 当前 Rust 差异

文件：`src/handlers/file_upload.rs`

- 当前上传目录写死为相对路径 `uploads/...`，没有对应 Java 的配置项。
- 当前 `upload_ico`、`upload_app` 直接复用 logo 逻辑，未实现各自 Java 校验和目录规则。
- 当前未严格校验 APP 的 `appType`。
- 返回 URL 是否能被 Axum 静态文件路由访问尚未实现验证。
- 需要增加安全文件名处理，避免原始文件名路径穿越。

## 17. 地图观测模块

### Java 基准

Controller：`MapObsPlanController` 的 `/mapObsPlan/polygonPoint` 使用多个可选请求参数，不是统一 JSON 对象。具体参数和返回对象需要继续从该 Controller 调用的 Service/Native 库实现核对。

### 当前 Rust 差异

- 当前路由缺失或没有稳定绑定到等价 handler。
- 不能用读取 `block_plan` 最新记录的简化逻辑代替 Java 地图观测算法。
- `MapObsPlanLib.so`、Java Service 参数和返回结构必须单独核对。

## 18. MQTT 消息模块

### Java 基准

Controller：`MqttMessageController`

```text
addTimeTask(MqttTaskInfo JSON)
exeTimeTaskNow(MqttTaskInfo JSON)
queryTimeTask(Query boatId)
querySingleTimeTask(Query boatId,id)
deleteTimeTask(Query boatId,id)
updateTimeTask(MqttTaskInfo JSON)
```

Java 还涉及 MQTT 连接、定时任务和 Redis/任务上下文，不能只做 HTTP 空响应。

### 当前 Rust 差异

- 当前 Rust 没有完整的 MQTT 客户端、任务调度和消息下发实现。
- 如果前端或设备侧依赖 MQTT，这是功能缺失，不是格式差异。
- 需要先确认 MQTT broker、Topic、消息格式和任务状态流转，再实现。

## 19. 当前已确认的路由问题

当前 `src/routes.rs` 中存在以下已确认风险：

1. 前端管理端接口使用 `/admin/...`，但区块任务、船坞任务、MongoDB、文件上传等路由没有统一 `/admin` 前缀。
2. Java 的 `/register/*` 与前端当前使用的 `/login/register/*` 存在部署层前缀差异，需要通过兼容路由或网关规则明确处理，不能混用后再猜测。
3. Java 管理接口大多由前端以 POST 调用，Rust 路由必须以 Java Controller 的参数注解为准：JSON body 和 query 参数不能互换。
4. 当前路由绑定了若干简化 handler，新增的 `*_v2` 或旧 handler 可能同时存在，必须清理重复实现，保证每个路径只对应一个明确实现。
5. Java Controller 中的 `updataNode2/updataNodeGet/deteleNodeGet`、`queryNodeFuncationByTaskId`、用户批量关系接口等不能遗漏。
6. `MapLinkController` 和 `YS7Controller` 也在 Java 工程中，不能因为当前前端 API 目录没有调用就直接标记为完成；需要按迁移目标决定是否纳入兼容范围。

## 20. 推荐实施顺序

每次只完成一个模块，并在完成后执行三类验证：

1. **静态契约检查**：Controller 注解、实体字段、Mapper SQL、Rust route/handler 签名逐项对照。
2. **响应快照检查**：使用同一组请求参数，分别保存 Java 101 和 Rust 响应 JSON，逐字段比较。
3. **数据库行为检查**：对新增、更新、删除、绑定、解绑使用测试数据验证影响行数、删除标记、关联表和分页总数。

建议顺序：

1. 认证模块
2. 用户模块
3. 船只模块
4. 参数模块
5. 船只功能模块
6. 船只权限模块
7. 系统权限模块
8. 模块管理
9. APP 管理
10. 白名单
11. 区块任务
12. 船坞任务
13. MongoDB
14. 文件上传
15. 地图观测
16. MQTT

## 21. 审计状态

| 模块 | 已完成 Java/Rust 契约核对 | 当前判定 |
|---|---:|---|
| 认证 | 已修正主要契约，待快照验证 | 已补齐 LoginMessage 字段、Java Base64 DER 公钥格式和 token header 读取；RSA 密钥持久化、权限缓存、verifyToken 全流程仍需线上快照验证 |
| 用户 | 第一轮已修正，待完整快照验证 | 已补齐 Java 查询字段、remarks 关键词、removecode 过滤、总数和裸 List 查询；批量角色/机构/船只操作及完整 User 写入仍需继续核对 |
| 船只 | 部分 | 当前字段和查询逻辑明显不完整 |
| 参数 | 部分 | 两种查询被错误复用，JOIN/删除语义未对齐 |
| 船只功能 | 部分 | 字段、分页、任务查询和路由仍不完整 |
| 船只权限 | 部分 | 字段和分页未完整对齐 |
| 系统权限 | 部分 | 条件字段和删除语义未完成核对 |
| 模块 | 部分 | 字段、条件和关联返回未完成核对 |
| 白名单 | 已发现明确差异 | 查询是空实现，Redis 结构未兼容 |
| APP | 已发现明确差异 | DATETIME、版本条件、字段不完整 |
| 区块任务 | 已发现明确差异 | 路径、参数来源、接口缺失 |
| 船坞任务 | 已发现明确差异 | Redis 结构和查询为空 |
| MongoDB | 已发现明确差异 | 四个接口仍是占位实现 |
| 文件上传 | 已发现明确差异 | 配置、校验、目录和返回未完全对齐 |
| 地图观测 | 未完成 | 参数和算法尚未核对 |
| MQTT | 未完成 | 设备通信和任务调度尚未实现 |

**结论：当前项目处于“已拆分、可编译、部分接口可访问”的迁移中间状态，不应标记为 Java 版本的完整等价实现。**
