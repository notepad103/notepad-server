# notepad-server

Rust 后端服务：Axum + SQLx (PostgreSQL)，提供用户与验证码相关 API。

## 技术栈

- **Runtime**: Tokio
- **Web**: Axum
- **数据库**: PostgreSQL（SQLx，带迁移）
- **缓存**: Redis（验证码写入 Redis，发码前须配置 `REDIS_URL`）
- **邮件**: Lettre（SMTP，用于发送验证码邮件）

## 环境要求

- Rust toolchain（本仓库 `edition = "2024"`，建议使用当前 stable）
- PostgreSQL
- Redis（使用「发送验证码」接口时需要）
- 可访问的 SMTP（QQ/163 等需开启 SMTP 并使用**授权码**，非登录密码）

## 配置

1. 复制环境变量模板并编辑：

```bash
cp .env.example .env
```

2. **必填**

| 变量 | 说明 |
|------|------|
| `DATABASE_URL` | PostgreSQL 连接串 |
| `SMTP_FROM_EMAIL` | 发件邮箱（须与邮箱里开启 SMTP 的账号一致） |
| `SMTP_AUTH_CODE` | SMTP 授权码 |

3. **常用可选**

| 变量 | 默认值 | 说明 |
|------|--------|------|
| `BIND_ADDR` | `0.0.0.0:3000` | HTTP 监听地址 |
| `REDIS_URL` | 未设置 | 不设则服务能启动，但发验证码会返回业务错误提示未启用 Redis |
| `SMTP_HOST` | `smtp.qq.com` | SMTP 主机（163 可填 `smtp.163.com`） |
| `SMTP_PORT` | `465` | SMTP 端口（与 SMTPS/TLS 方式一致） |
| `JWT_SECRET` | （无默认，必填） | HS256 签名密钥，建议 ≥32 字节的随机串 |
| `JWT_EXP_SECS` | `604800`（7 天） | 访问令牌过期时间（秒） |

**安全**：不要把真实 `.env` 提交到 Git；仓库内仅保留 `.env.example` 占位。敏感信息泄露后请尽快在邮箱侧**重置授权码**，并**轮换 `JWT_SECRET`**（轮换后已签发令牌全部失效）。

## 运行

```bash
cargo run
```

也可使用项目根目录的 `Makefile`：

```bash
make dev    # 运行服务（等价于 cargo run）
make check  # 编译检查（cargo check）
make watch  # 监听 src/ 与 migrations/ 后自动重启（需 cargo-watch）
```

首次使用 `make watch` 前：

```bash
cargo install cargo-watch
```

首次运行会自动执行 `migrations/` 下的数据库迁移。

## API

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/` | 根路径：`{ "ok": true, "db": 1, "redis": "PONG" \| null \| false }` |
| GET | `/health` | 健康检查，纯文本 `ok` |
| POST | `/users` | 创建用户，JSON：`username`、`email`、`password`、`verification_code`；成功 **201** 及用户信息 |
| POST | `/users/login` | 登录，JSON：`email`、`password`；成功 **200** 返回 `access_token`（Bearer）、`user` |
| GET | `/users/:id` | 查询用户；须在请求头携带 `Authorization: Bearer <access_token>`，且 **只能查询与 token 中用户 id 相同的路径** |
| POST | `/users/:email/verify` | 用户须已存在；写 Redis `verify:email:{email}`（约 300s）并发验证码邮件。路径里的邮箱建议 URL 编码，例如 `@` 写为 `%40` |

### 请求示例

创建用户：

```bash
curl -X POST "http://127.0.0.1:3000/users" \
  -H "Content-Type: application/json" \
  -d '{"username":"demo","email":"user@example.com","password":"secret"}'
```

发送验证码（邮箱含 `@` 时使用编码）：

```bash
curl -X POST "http://127.0.0.1:3000/users/user%40example.com/verify"
```

登录并访问受保护接口：

```bash
TOKEN="$(curl -s -X POST "http://127.0.0.1:3000/users/login" \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","password":"secret"}' | jq -r .access_token)"

curl -s "http://127.0.0.1:3000/users/1" -H "Authorization: Bearer $TOKEN"
```

## 项目结构

```
src/
├── main.rs       # 入口
├── lib.rs        # 应用组装与启动
├── config.rs     # 配置（环境变量）
├── error.rs      # 统一错误类型
├── state.rs      # 应用状态（DB、可选 Redis）
├── models/       # 请求/响应 DTO
├── routes/       # 路由注册
├── handlers/     # HTTP 处理
├── services/     # 业务与数据库逻辑
└── utils/        # 工具（如邮件）
```
