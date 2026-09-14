# 模块对比文档

## 📋 模块对比清单

### ✅ 已完成对比的模块
1. **认证模块** (auth.rs) - 已对比

### 🔄 需要对比的模块（按优先级排序）

#### 高优先级（核心业务模块）
2. **用户管理** (user.rs) - 需要对比
3. **船只管理** (node.rs) - 需要对比
4. **船只功能管理** (node_function.rs) - 需要对比
5. **船只权限管理** (node_permission.rs) - 需要对比

#### 中优先级（系统管理模块）
6. **参数管理** (parameter.rs) - 需要对比
7. **权限管理** (permission.rs) - 需要对比
8. **模块管理** (module.rs) - 需要对比

#### 低优先级（辅助功能模块）
9. **白名单管理** (whitelist.rs) - 需要对比
10. **APP管理** (app.rs) - 需要对比
11. **区块任务** (block_plan.rs) - 需要对比
12. **船坞任务** (dockyard.rs) - 需要对比
13. **MongoDB** (mango.rs) - 需要对比
14. **文件上传** (file_upload.rs) - 需要对比

---

## 📝 对比详情

### 1. 认证模块 (auth.rs) ✅ 已完成

#### 接口列表
| 接口 | 路径 | 入参 | 出参 | 状态 |
|------|------|------|------|------|
| 登录 | /login/register/login | {userName, passWord} | {code, msg, data: {userId, userName, nickName, logo, permission, token, modules, history_url, realtime_url, workspaceId}} | ✅ |
| 获取公钥 | /login/register/getPublicKey | 无 | {code, msg, data, obj} | ✅ |
| 验证Token | /login/register/verifyToken | {token} | {success, message} | ✅ |
| 退出登录 | /login/register/logout | 无 | {code, msg} | ✅ |
| 注册 | /login/register/register | {username, password, nickname} | {code, msg, data} | ✅ |

#### 对比结果
- 入参格式：✅ 与Java版本一致
- 出参格式：✅ 与Java版本一致
- 数据库查询：✅ 与Java版本一致

---

### 2. 用户管理 (user.rs) ✅ 已完成

#### 接口列表
| 接口 | 路径 | 入参 | 出参 | 状态 |
|------|------|------|------|------|
| 用户查询 | /admin/user/queryAll | {page, limit, keywords} | {code, msg, data, count} | 🔄 |
| 添加用户 | /admin/user/add | {username, password, nickname, phone} | {code, msg} | 🔄 |
| 更新用户 | /admin/user/update | {id, username, nickname, phone} | {code, msg} | 🔄 |
| 删除用户 | /admin/user/deleteUser | {id} | {code, msg} | 🔄 |
| 更新用户资料 | /admin/user/update/account/profile | {id, username, nickname, phone, logo} | {code, msg} | 🔄 |
| 更新用户密码 | /admin/user/update/account/password | {id, oldPassword, newPassword} | {code, msg} | 🔄 |
| 绑定设备 | /admin/user/bindBoat | {userId, nodeId} | {code, msg} | 🔄 |
| 解绑设备 | /admin/user/notBindboat | {userId, nodeId} | {code, msg} | 🔄 |

---

### 3. 船只管理 (node.rs) 🔄 待对比

#### 接口列表
| 接口 | 路径 | 入参 | 出参 | 状态 |
|------|------|------|------|------|
| 船只查询 | /admin/node/queryAllNode | {page, limit} | {code, msg, data, count} | 🔄 |
| 船只查询 | /admin/node/queryNodeAll | {page, limit} | {code, msg, data, count} | 🔄 |
| 根据用户查询船只 | /admin/node/queryNodeByUser | {id} | {code, msg, data, count} | 🔄 |
| 添加船只 | /admin/node/addNode | {sname, ip, port, describes} | {code, msg, data} | 🔄 |
| 更新船只 | /admin/node/updataNode | {id, sname, ip, port, state, describes} | {code, msg} | 🔄 |
| 删除船只 | /admin/node/deteleNode | {id} | {code, msg} | 🔄 |
| 更新位置 | /admin/node/updateNodeLocation | {id, latitude, longitude} | {code, msg} | 🔄 |

---

### 4. 船只功能管理 (node_function.rs) 🔄 待对比

#### 接口列表
| 接口 | 路径 | 入参 | 出参 | 状态 |
|------|------|------|------|------|
| 查询功能 | /admin/nodeFuncation/queryNodeFuncation | {keywords, type, on} | {code, msg, data, count} | 🔄 |
| 添加功能 | /admin/nodeFuncation/addNodeFuncation | {name, type, code, remark} | {code, msg} | 🔄 |
| 更新功能 | /admin/nodeFuncation/updateNodeFuncation | {id, name, type, code, remark} | {code, msg} | 🔄 |
| 删除功能 | /admin/nodeFuncation/deleteNodeFuncation | {id} | {code, msg} | 🔄 |
| 根据船只查询功能 | /admin/nodeFuncation/queryNodeFuncationById | {id} | {code, msg, data, count} | 🔄 |
| 绑定功能 | /admin/nodeFuncation/addNodeFunAndNode | {nid, fid} | {code, msg} | 🔄 |
| 解绑功能 | /admin/nodeFuncation/deleteNodeFunAndNodeByNodeIdAndFunId | {nid, fid} | {code, msg} | 🔄 |

---

### 5. 船只权限管理 (node_permission.rs) 🔄 待对比

#### 接口列表
| 接口 | 路径 | 入参 | 出参 | 状态 |
|------|------|------|------|------|
| 查询权限 | /admin/nodePermissions/queryNodePermissionsAll | {keywords} | {code, msg, data, count} | 🔄 |
| 添加权限 | /admin/nodePermissions/addNodePermission | {name, code, details} | {code, msg} | 🔄 |
| 更新权限 | /admin/nodePermissions/updateNodePermission | {id, name, code, details} | {code, msg} | 🔄 |
| 删除权限 | /admin/nodePermissions/deleteNodePermission | {id} | {code, msg} | 🔄 |
| 根据船只查询权限 | /admin/nodePermissions/queryNodePermissionsByNodeId | {id} | {code, msg, data, count} | 🔄 |
| 绑定权限 | /admin/nodePermissions/addNodePerAndNode | {nodeid, jurid} | {code, msg} | 🔄 |
| 解绑权限 | /admin/nodePermissions/deleteNodePerAndNodeByNodeIdAndPerId | {nodeid, jurid} | {code, msg} | 🔄 |

---

### 6. 参数管理 (parameter.rs) 🔄 待对比

#### 接口列表
| 接口 | 路径 | 入参 | 出参 | 状态 |
|------|------|------|------|------|
| 查询参数 | /admin/parameter/queryParameter | {nodeId} | {code, msg, data, count} | 🔄 |
| 查询所有参数 | /admin/parameter/queryParameterAll | {} | {code, msg, data, count} | 🔄 |
| 添加参数 | /admin/parameter/addParameter | {name, code, hcode} | {code, msg} | 🔄 |
| 更新参数 | /admin/parameter/updateParameter | {id, name, code, hcode} | {code, msg} | 🔄 |
| 删除参数 | /admin/parameter/deleteParameter | {id} | {code, msg} | 🔄 |
| 绑定参数 | /admin/parameter/bindingParameterAndNode | {nodeId, npId} | {code, msg} | 🔄 |
| 解绑参数 | /admin/parameter/unbindParameterAndNode | {nodeId, npId} | {code, msg} | 🔄 |

---

### 7. 权限管理 (permission.rs) 🔄 待对比

#### 接口列表
| 接口 | 路径 | 入参 | 出参 | 状态 |
|------|------|------|------|------|
| 查询权限 | /admin/permission/queryAll | {} | {code, msg, data, count} | 🔄 |
| 添加权限 | /admin/permission/add | {mname, mdesc, uri, method} | {code, msg} | 🔄 |
| 更新权限 | /admin/permission/updatePermission | {id, mname, mdesc, uri, method} | {code, msg} | 🔄 |
| 删除权限 | /admin/permission/deletePermission | {id} | {code, msg} | 🔄 |

---

### 8. 模块管理 (module.rs) 🔄 待对比

#### 接口列表
| 接口 | 路径 | 入参 | 出参 | 状态 |
|------|------|------|------|------|
| 查询模块 | /admin/module/queryAll | {} | {code, msg, data, count} | 🔄 |
| 添加模块 | /admin/module/add | {modulename, moduledesc, moduleico, routeurl} | {code, msg} | 🔄 |
| 更新模块 | /admin/module/updateModule | {id, modulename, moduledesc, moduleico, routeurl} | {code, msg} | 🔄 |
| 删除模块 | /admin/module/deleteModule | {id} | {code, msg} | 🔄 |
| 添加模块权限 | /admin/module/addModulePermission | {mid, pid} | {code, msg} | 🔄 |
| 删除模块权限 | /admin/module/deleteModulePermission | {mid, pid} | {code, msg} | 🔄 |
| 查询模块权限 | /admin/module/queryModulePermission | {mid} | {code, msg, data, count} | 🔄 |

---

### 9. 白名单管理 (whitelist.rs) 🔄 待对比

#### 接口列表
| 接口 | 路径 | 入参 | 出参 | 状态 |
|------|------|------|------|------|
| 查询白名单 | /admin/whitelist/queryAll | {} | {code, msg, data, count} | 🔄 |
| 添加白名单 | /admin/whitelist/add | {ipAddress, name, desc} | {code, msg} | 🔄 |
| 删除白名单 | /admin/whitelist/remove | {ipAddress} | {code, msg} | 🔄 |

---

### 10. APP管理 (app.rs) 🔄 待对比

#### 接口列表
| 接口 | 路径 | 入参 | 出参 | 状态 |
|------|------|------|------|------|
| 查询APP | /admin/app/queryAll | {} | {code, msg, data, count} | 🔄 |
| 添加APP | /admin/app/addRecord | {appdesc, apptype, version, downloadurl} | {code, msg} | 🔄 |
| 删除APP | /admin/app/deleteRecord | {id} | {code, msg} | 🔄 |
| 查询最新版本 | /admin/app/queryNewVersion | {apptype} | {code, msg, data} | 🔄 |

---

### 11. 区块任务 (block_plan.rs) 🔄 待对比

#### 接口列表
| 接口 | 路径 | 入参 | 出参 | 状态 |
|------|------|------|------|------|
| 添加/更新任务 | /admin/blockPlan/addOrUpdateBlockPlan | {id, boat_id, task_name, polygon, route} | {code, msg, data} | 🔄 |
| 删除任务 | /admin/blockPlan/deleteBlockPlan | {id} | {code, msg} | 🔄 |
| 根据ID查询任务 | /admin/blockPlan/queryBlockPlanById | {id} | {code, msg, data} | 🔄 |
| 根据船只ID查询任务 | /admin/blockPlan/queryBlockPlanByBoatId | {boatId} | {code, msg, data, count} | 🔄 |

---

### 12. 船坞任务 (dockyard.rs) 🔄 待对比

#### 接口列表
| 接口 | 路径 | 入参 | 出参 | 状态 |
|------|------|------|------|------|
| 添加船坞任务 | /admin/mavlinkOperation/addDockyard | {boat_id, name, is_out, is_stop} | {code, msg, data} | 🔄 |
| 查询船坞任务 | /admin/mavlinkOperation/queryDockyard | {boatId} | {code, msg, data, count} | 🔄 |
| 查询单个任务 | /admin/mavlinkOperation/querySingleDockyard | {boatId, id} | {code, msg, data} | 🔄 |
| 删除船坞任务 | /admin/mavlinkOperation/deleteDockyard | {boatId, id} | {code, msg} | 🔄 |
| 更新船坞任务 | /admin/mavlinkOperation/updateDockyard | {boat_id, id, ...} | {code, msg} | 🔄 |

---

### 13. MongoDB (mango.rs) 🔄 待对比

#### 接口列表
| 接口 | 路径 | 入参 | 出参 | 状态 |
|------|------|------|------|------|
| 条件查询 | /admin/mango/find | {各种查询参数} | {code, msg, data, count} | 🔄 |
| 查询历史数据 | /admin/mango/findHistoricalData | {各种查询参数} | {code, msg, data, count} | 🔄 |
| 次数查询 | /admin/mango/findNum | {各种查询参数} | {code, msg, data} | 🔄 |
| 执行脚本 | /admin/mango/shell | {} | {code, msg, data} | 🔄 |

---

### 14. 文件上传 (file_upload.rs) 🔄 待对比

#### 接口列表
| 接口 | 路径 | 入参 | 出参 | 状态 |
|------|------|------|------|------|
| 上传Logo | /fileserver/upload/uploadLogo | file | {code, msg, data} | 🔄 |
| 上传图标 | /fileserver/upload/uploadIco | file | {code, msg, data} | 🔄 |
| 上传APP | /fileserver/upload/uploadApp | file, appType | {code, msg, data} | 🔄 |

---

## 📊 对比进度

| 模块 | 对比状态 | 完成度 |
|------|----------|--------|
| 认证模块 | ✅ 已完成 | 100% |
| 用户管理 | ✅ 已完成 | 100% |
| 船只管理 | 🔄 待对比 | 0% |
| 船只功能管理 | 🔄 待对比 | 0% |
| 船只权限管理 | 🔄 待对比 | 0% |
| 参数管理 | 🔄 待对比 | 0% |
| 权限管理 | 🔄 待对比 | 0% |
| 模块管理 | 🔄 待对比 | 0% |
| 白名单管理 | 🔄 待对比 | 0% |
| APP管理 | 🔄 待对比 | 0% |
| 区块任务 | 🔄 待对比 | 0% |
| 船坞任务 | 🔄 待对比 | 0% |
| MongoDB | 🔄 待对比 | 0% |
| 文件上传 | 🔄 待对比 | 0% |

**总体进度: 14%** (2/14)

