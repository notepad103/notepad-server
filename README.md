# notepad-server

Rust 后端服务：Axum + SQLx (PostgreSQL)，提供用户等 API。

## 技术栈

- **Runtime**: Tokio
- **Web**: Axum
- **数据库**: PostgreSQL (SQLx，带迁移)
- **缓存**: Redis（可选，通过 `REDIS_URL` 启用）

## 环境要求

- Rust 1.70+
- PostgreSQL

## 配置

在项目根目录创建 `.env`：

```env
DATABASE_URL=postgres://user:password@localhost:5432/notepad
```

可选：

```env
BIND_ADDR=0.0.0.0:3000
REDIS_URL=redis://127.0.0.1:6379
```

未设置 `REDIS_URL` 时服务照常启动，仅不连接 Redis；`GET /` 中 `redis` 字段为 `null`。

默认监听 `0.0.0.0:3000`。

## 运行

```bash
cargo run
```

也可以使用项目根目录的 `Makefile`：
```bash
make dev    # 运行服务（等价于 cargo run）
make check  # 编译检查（cargo check）
make watch  # 监听 src/ 和 migrations/ 变化后自动重启（需要 cargo-watch）
```

首次使用 `make watch` 前，先安装：
```bash
cargo install cargo-watch
```

首次运行会自动执行 `migrations/` 下的数据库迁移。

## API

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/` | 根路径，返回 `{ "ok": true, "db": 1, "redis": "PONG" \| null \| false }`（配置了 Redis 且连通时为 `"PONG"`，未配置为 `null`，配置了但失败为 `false`） |
| GET | `/health` | 健康检查，返回 `ok` |
| POST | `/users` | 创建用户，请求体 `{ "username": "xxx", "email": "xxx@example.com" }`，成功返回 201 及用户信息 |

## 项目结构

```
src/
├── main.rs       # 入口
├── lib.rs        # 应用组装与启动
├── config.rs     # 配置（环境变量）
├── error.rs      # 统一错误类型
├── state.rs      # 应用状态（DB 连接池）
├── models/       # 请求/响应 DTO
├── routes/       # 路由注册
├── handlers/     # HTTP 处理（参数解析、调用 service）
└── services/     # 业务与数据库逻辑
```
