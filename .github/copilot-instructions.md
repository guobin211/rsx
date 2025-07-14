# 编码规范

## 通用规则

- 使用中文进行交流沟通
- 代码问题直接给出答案和代码实现，不需要解释推理过程
- 代码注释规范：
    - 不使用行尾注释
    - 只给函数添加注释，不要使用JsDoc注释，使用Typescript类型系统来描述函数参数和返回值
    - 只在必要的判断逻辑处添加注释
- 忽略版本控制的目录：
    - node_modules
    - dist
    - build
    - target
    - uploads
    - pkg
    - npm
    - public
    - assets
- 忽略编译生成的文件：
    - .d.ts
    - .wasm
    - .cjs
    - .mjs
- 忽略配置文件：
    - .env
    - .env.local
    - .env.development
    - .env.production
    - .env.test
- 函数命名规范：
    - `ts`代码和`js`代码使用小驼峰命名法，例如：`getUserInfo`。`rust`代码使用蛇形命名法，例如：`get_user_info`。
    - 函数名应简洁明了，描述函数的功能，不添加行尾注释
    - 不要使用缩写，除非是非常常见的缩写
- 文件命名规范：
    - `ts`代码和`js`代码使用`angular`框架的命名法，例如：`use-window-state.ts`和`user.service.ts`。`rust`代码使用蛇形命名法，例如：`user_service.rs`。
    - 文件名应简洁明了，描述文件的功能

## 技术栈

### 后端技术

- Rust
    - actix-web (Web框架)
    - tokio (异步运行时)
    - clap (命令行参数解析)
    - reqwest (HTTP客户端)
    - serde (序列化/反序列化)
    - serde_json (JSON处理)
    - anyhow (错误处理)
    - lazy_static (静态变量)

- TypeScript/Node.js
    - ofetch (HTTP客户端)
    - vitest (测试框架)
    - zod (数据验证)
    - dotenv (环境变量管理)
    - dayjs (日期时间处理)
    - es-toolkit (常用函数库)
    - execa (命令行执行)
    - pino (日志处理)

### 前端技术

- React (前端框架)
- Zustand (状态管理)
- Scss (样式处理)

### Typescript构建工具

- vite (Web构建工具)
- vitest (测试框架)
- tsdown (库构建工具)
- rsbuild (Webpack构建工具)
- esbuild (Nodejs服务端打包工具)

# TypeScript编码规范

- 使用单引号
- 使用ES6模块化
- 使用async/await处理异步操作
- 使用try/catch处理异常
- 使用vitest进行单元测试
- 使用hono编写服务端代码
- 使用mongoose操作MongoDB
- 使用ioredis操作Redis
- 使用ofetch发送HTTP请求
- 使用zod进行数据验证
- 使用dotenv管理环境变量
- 使用dayjs处理日期时间
- 使用es-toolkit作为常用函数库
- 使用execa来执行命令行命令
- 使用pino来处理日志
- react使用使用clsx来处理类名
- react使用ahooks库作为常用hooks
- react使用react-hook-form库来处理表单
- 导入nodejs内部模块时带上`node:`前缀，如`import fs from 'node:fs/promises'`

## 基本规范

### 语法规则

- 使用4个空格缩进
- 使用单引号作为字符串定界符
- 行宽不超过100字符
- 优先使用`const`声明变量，避免使用`let`
- 异步函数使用`async`关键字声明
- 文件名使用kebab-case（短横线分隔）、采用`angular`的命名风格

### 函数声明

- 函数名使用camelCase
- 函数有返回值时使用明确的返回类型

```typescript
function getUserInfo(id: string): Promise<UserInfo> {
    return fetchUser(id);
}
```

### 异步编程

- 使用`async/await`处理异步操作
- 使用`try/catch`处理异常
- 函数省略`void`返回类型

```typescript
async function fetchData() {
    try {
        const result = await api.getData();
        processResult(result);
    } catch (error) {
        handleError(error);
    }
}
```

### 循环和遍历

- 使用`for...of`遍历数组

```typescript
for (const item of items) {
    processItem(item);
}
```

- 使用`Object.entries`遍历对象

```typescript
for (const [key, value] of Object.entries(obj)) {
    processKeyValue(key, value);
}
```

### 类型定义

- 使用`interface`定义对象类型

```typescript
interface UserProfile {
    id: string;
    name: string;
    age: number;
}
```

- 使用`type`定义联合类型

```typescript
type Status = 'active' | 'inactive' | 'pending';
```

### 枚举定义

- 不使用`enum`，使用`const`对象和类型推导

```typescript
export const STATUS = {
    ACTIVE: 'active',
    INACTIVE: 'inactive',
    PENDING: 'pending'
} as const;

export type Status = (typeof STATUS)[keyof typeof STATUS];
```

### 注释规范

- 不使用JSDoc
- 不使用行尾注释
- 只给函数和复杂逻辑添加注释

```typescript
// 处理用户认证并返回token
function authenticateUser(credentials: Credentials): Promise<Token> {
    // 验证用户凭证并检查权限
    if (isValidCredentials(credentials) && hasPermission(credentials)) {
        return generateToken(credentials);
    }
    throw new Error('Authentication failed');
}
```

### 文件命名

- 使用kebab-case（短横线分隔）
- 遵循NestJS命名风格

```
user.controller.ts
user.service.ts
user.dto.ts
user-profile.interface.ts
```

## React前端规范

### 文件命名

- 使用kebab-case（短横线分隔）

```
user-list.component.tsx
user-profile.component.tsx
```

### 组件定义

- 使用函数组件
- 组件名使用PascalCase，props名称使用`${Name}Props`
- 函数组件省略返回类型

```tsx
interface UserListProps {
    users: User[];
}
function UserList(props: UserListProps) {
    return (
        <div>
            <h1>User List</h1>
            <ul>
                <li>User 1</li>
                <li>User 2</li>
            </ul>
        </div>
    );
}
```

### 事件处理

- 使用`const`和`useCallback`定义事件处理函数

```typescript
const handleClick = useCallback(() => {
    processClick();
}, [processClick]);
```

### 状态管理

- 使用`zustand`管理状态

```typescript
const useStore = create<Store>((set) => ({
    count: 0,
    increment: () => set((state) => ({ count: state.count + 1 }))
}));
```

### 样式规范

- 使用SCSS和CSS Modules
- 类名使用camelCase

```scss
.userContainer {
    display: flex;
}

.userProfile {
    margin: 16px;
}
```

```tsx
import styles from './user-list.module.scss';

function UserList() {
    return (
        <div className={styles.userContainer}>
            <div className={styles.userProfile}>User Profile</div>
        </div>
    );
}
```

# Rust编码规范

- 使用4个空格缩进
- 使用actix-web编写服务端代码
- 使用reqwest发送HTTP请求
- 使用tokio、tokio-stream、tokio-util来处理异步编程
- 使用anyhow处理异常
- 使用lazy_static处理静态变量
- 使用serde和serde_json处理序列化
- 使用tauri处理桌面应用程序
- 使用clap处理命令行参数
- 使用mongodb和redis库操作MongoDB和Redis
- 使用log和env_logger处理日志
- 使用thiserror处理错误
- 使用regex来处理正则表达式
- 使用validator来验证数据
- 使用walkdir来遍历目录
- 使用futures和async_trait来处理异步编程接口
- 使用dayjs处理日期时间，代码在`crates/dayjs/src/lib.rs`
- 使用afs来处理文件系统操作，代码在`crates/afs/src/lib.rs`
- 使用codec来处理数据编解码，代码在`crates/codec/src/lib.rs`

## 代码风格

### 缩进和格式化

- 使用4个空格缩进
- 最大行宽限制为100字符
- 使用Unix风格的换行符
- 使用rustfmt进行代码格式化，配置文件`.rustfmt.toml`内容如下：
    ```toml
    max_width = 100
    tab_spaces = 4
    newline_style = "Unix"
    ```

### 命名规范

- 变量和函数使用`snake_case`

```rust
fn main() {
    let user_name = "john";
    fn process_data() { }
}
```

- 结构体和枚举使用`PascalCase`

    ```rust
    struct UserProfile { }
    enum ConnectionState { }
    ```

- 枚举序列化为String

    ```rust
    #[derive(Serialize, Deserialize)]
    enum UserRole {
        #[serde(rename = "admin")]
        Admin,
        #[serde(rename = "user")]
        User,
        #[serde(rename = "guest")]
        Guest,
    }
    ```

- 常量使用`SCREAMING_SNAKE_CASE`
    ```rust
    const MAX_CONNECTIONS: u32 = 100;
    ```

## 依赖库使用规范

### Web服务开发

- 使用`actix-web`作为Web框架
    ```rust
    use actix_web::{web, App, HttpResponse, HttpServer};
    ```

### HTTP客户端

- 使用`reqwest`发送HTTP请求
    ```rust
    use reqwest::Client;
    ```

### 异步编程

- 使用`tokio`处理异步操作
    ```rust
    use tokio::runtime::Runtime;
    ```

### 错误处理

- 使用`anyhow`处理异常
    ```rust
    use anyhow::{Result, Context};
    ```

### 静态变量

- 使用`lazy_static`处理静态变量
    ```rust
    use lazy_static::lazy_static;
    ```

### 序列化/反序列化

- 使用`serde`和`serde_json`处理数据序列化
    ```rust
    use serde::{Serialize, Deserialize};
    ```

### 桌面应用开发

- 使用`tauri`开发桌面应用程序
    ```rust
    use tauri::Builder;
    ```

## 注释规范

- 不使用行尾注释
- 只在函数定义处添加文档注释
    ```rust
    /// 处理用户登录请求
    fn handle_login(credentials: Credentials) -> Result<Token> {
        // 实现代码
    }
    ```
- 只在复杂的判断逻辑处添加必要注释
    ```rust
    fn main() {
        // 检查用户权限并验证token有效性
        if check_permission() && validate_token() {
            // 实现代码
        }
    }
    ```

## 文件组织

- 使用`mod.rs`组织模块
- 每个功能模块放在独立的文件中
- 相关的类型定义放在同一个模块中

## 性能建议

- 避免不必要的`.clone()`
- 考虑使用`Arc`/`Rc`共享所有权
- 大数据结构考虑使用`Cow`实现写时复制
- 热点路径避免动态分配
- 使用`#[inline]`谨慎地优化小函数

## 错误处理规范

### 错误类型设计

- 为模块或库定义专门的错误类型

```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    Database(#[from] mongodb::error::Error),

    #[error("验证错误: {0}")]
    Validation(String),

    #[error("未授权访问")]
    Unauthorized,

    #[error("资源不存在: {0}")]
    NotFound(String),
}
```

### 错误传播

- 使用`?`操作符传播错误，而不是使用`match`或`unwrap`

```rust
fn process_data(data: &str) -> Result<ProcessedData, AppError> {
    let parsed = parse_input(data)?;
    let validated = validate_data(&parsed)?;
    Ok(transform_data(validated)?)
}
```

### 错误处理策略

- 优先使用`Result`类型处理错误，避免使用`panic!`
- 只在不可恢复的情况下使用`panic!`或`unwrap`
- 使用`thiserror`定义错误类型，使用`anyhow`处理上层应用错误
- 在库代码中返回精确的错误类型，在应用代码中可以使用`anyhow::Error`

## 并发编程规范

### 线程安全

- 使用`Arc`在线程间共享数据
- 使用`Mutex`或`RwLock`保护共享状态
- 避免使用`Rc`和`RefCell`在多线程环境中

### 异步编程

- 使用`async/await`而不是手动管理`Future`
- 使用`tokio::spawn`创建异步任务

### 通道使用

- 使用`tokio::sync::mpsc`在异步任务间通信
- 使用`std::sync::mpsc`在线程间通信
- 适当设置通道缓冲区大小

## 宏使用规范

### 宏定义

- 宏名称使用`snake_case`
- 为宏提供详细的文档和示例
- 使用`$crate`前缀引用当前crate中的项

### 宏使用

- 优先使用函数和泛型，只在必要时使用宏
- 使用`macro_rules!`定义声明式宏
- 复杂情况下使用过程宏

## 安全性考虑

### 输入验证

- 不信任外部输入，总是验证所有输入
- 使用`regex`验证字符串格式
- 限制输入大小，防止DoS攻击

### 敏感数据处理

- 使用`secrecy`或`zeroize`处理敏感数据
- 避免将敏感数据写入日志
- 使用`constant_time_eq`比较敏感数据

### 不安全代码

- 将`unsafe`代码封装在安全的API后面
- 为每个`unsafe`块添加详细注释，解释为什么它是安全的
- 使用`unsafe`的替代方案，如`bytemuck`处理类型转换

# Rsx编码规范

## 基本规范

### 语法规则

- rsx文件用`.rsx`后缀，文件内容分为4个部分：`rust`、`script`、`template`和`style`
- `rust`部分使用Rust语言编写，以`---`开头，以`---`结尾
- `script`部分使用JavaScript语言编写，以`<script>`开头，以`</script>`结尾
- `template`部分使用HTML模板语法，以`<template>`开头，以`</template>`结尾
- `style`部分使用CSS样式，以`<style>`开头，以`</style>`结尾
- 使用4个空格缩进
- 行宽不超过100字符

### Rust 部分

- 使用`async fn get_server_side_props(req: Request) -> Response`获取服务器端属性
- 使用`async fn get_server_side_props(req: Request, ctx: Context) -> Response`获取服务器端属性，携带上下文

```rust
async fn get_server_side_props(req: Request) -> Response {
    // 获取服务器端属性
    let items = vec![
        "Apple".to_string(),
        "Banana".to_string(),
    ];
    Response::json!({
        "data": items,
    })
}
```

### JavaScript 部分

- 使用`defineProps()`接收组件属性

```ts
const { data } = defineProps<{ data: string[] }>();
```

### Template 部分

- 使用类似`svelte`的模板语法

```html
<ul>
    {#each data as item, i}
    <li>{i + 1}: {item}</li>
    {/each}
</ul>
```
