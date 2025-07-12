# RSX 框架编码规范

## 目录

- [概述](#概述)
- [RSX文件规范](#rsx文件规范)
- [Rust部分规范](#rust部分规范)
- [JavaScript/TypeScript部分规范](#javascripttypescript部分规范)
- [Template部分规范](#template部分规范)
- [Style部分规范](#style部分规范)
- [通用编码规范](#通用编码规范)
- [项目结构规范](#项目结构规范)
- [测试规范](#测试规范)
- [文档规范](#文档规范)

## 概述

RSX是一个现代化的全栈Web框架，支持服务端渲染(SSR)和客户端渲染(CSR)。RSX文件采用混合语法，将Rust后端逻辑、TypeScript前端逻辑、HTML模板和CSS样式集成在一个文件中。

### 核心特性

- **服务端渲染**：Rust + Handlebars模板
- **客户端渲染**：TypeScript + React
- **统一开发体验**：单文件组件
- **类型安全**：Rust + TypeScript双重类型检查
- **高性能**：Rust后端 + 现代前端构建工具

## RSX文件规范

### 文件结构

RSX文件使用`.rsx`扩展名，包含四个主要部分：

```rsx
---
// Rust部分：服务端逻辑
---

<script>
// JavaScript/TypeScript部分：客户端逻辑
</script>

<template>
<!-- Template部分：HTML模板 -->
</template>

<style>
/* Style部分：CSS样式 */
</style>
```

### 文件命名规范

- 使用kebab-case命名法
- 页面文件：`user-profile.rsx`、`product-list.rsx`
- 组件文件：`user-card.rsx`、`navigation-bar.rsx`
- 布局文件：`main-layout.rsx`、`admin-layout.rsx`

### 基本语法规则

- 使用4个空格缩进
- 行宽不超过100字符
- 使用UTF-8编码
- 使用Unix风格换行符(LF)

## Rust部分规范

### 语法规则

- 以`---`开头和结尾
- 使用Rust语言编写服务端逻辑
- 支持异步编程和错误处理

### 服务端属性函数

#### 基本格式

```rust
async fn get_server_side_props(req: Request) -> Response {
    // 服务端逻辑
    let data = fetch_data().await?;
    Response::json!({
        "data": data,
    })
}
```

#### 带上下文的格式

```rust
async fn get_server_side_props(req: Request, ctx: Context) -> Response {
    // 使用上下文信息
    let user_id = ctx.get_user_id()?;
    let data = fetch_user_data(user_id).await?;
    Response::json!({
        "data": data,
    })
}
```

## JavaScript/TypeScript部分规范

### 语法规则

- 以`<script>`开头，以`</script>`结尾
- 使用TypeScript编写客户端逻辑
- 支持ES6+语法特性

### 组件属性定义

#### 基本属性

```typescript
const { data, loading, error } = defineProps<{
    data: string[];
    loading: boolean;
    error?: string;
}>();
```

#### 复杂属性

```typescript
interface User {
    id: string;
    name: string;
    email: string;
    avatar?: string;
}

const { users, onUserClick, showAvatar } = defineProps<{
    users: User[];
    onUserClick: (user: User) => void;
    showAvatar: boolean;
}>();
```

## Template部分规范

### 语法规则

- 以`<template>`开头，以`</template>`结尾
- 使用类似Svelte的模板语法
- 编译后生成Handlebars模板文件

### 基本语法

#### 文本插值

```html
<template>
    <h1>Hello, {{ name }}!</h1>
    <p>Welcome to {{ appName }}</p>
</template>
```

#### 条件渲染

```html
<template>
    {#if user}
    <div class="user-profile">
        <h2>{{ user.name }}</h2>
        <p>{{ user.email }}</p>
    </div>
    {:else}
    <div class="login-prompt">
        <p>Please log in to continue</p>
    </div>
    {/if}
</template>
```

#### 列表渲染

```html
<template>
    <ul class="user-list">
        {#each users as user, index}
        <li class="user-item" data-index="{{ index }}">
            <span class="user-name">{{ user.name }}</span>
            <span class="user-email">{{ user.email }}</span>
        </li>
        {/each}
    </ul>
</template>
```

#### 嵌套循环

```html
<template>
    {#each categories as category}
    <div class="category">
        <h3>{{ category.name }}</h3>
        <ul class="items">
            {#each category.items as item}
            <li>{{ item.name }} - {{ item.price }}</li>
            {/each}
        </ul>
    </div>
    {/each}
</template>
```

### 样式绑定

```html
<template>
    <div class="{{ isActive ? 'active' : 'inactive' }}">
        <span class="status-{{ status }}">{{ status }}</span>
    </div>
</template>
```

## Style部分规范

### 语法规则

- 以`<style>`开头，以`</style>`结尾
- 使用CSS或SCSS编写样式
- 支持CSS Modules和Scoped样式

### 基本样式

```css
<style>
.container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
}
</style>
```

### 动画和过渡

```css
<style>
.fade-in {
    opacity: 0;
    animation: fadeIn 0.3s ease-in forwards;
}

@keyframes fadeIn {
    from {
        opacity: 0;
        transform: translateY(20px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}

.button {
    transition: all 0.2s ease;
}

.button:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
}
</style>
```

## 通用编码规范

### 命名规范

#### 文件命名

- RSX文件：kebab-case，如`user-profile.rsx`
- Rust文件：snake_case，如`user_service.rs`
- TypeScript文件：kebab-case，如`user-service.ts`

#### 变量命名

- Rust：snake_case，如`user_name`
- TypeScript：camelCase，如`userName`
- 常量：SCREAMING_SNAKE_CASE，如`MAX_RETRY_COUNT`

#### 函数命名

- Rust：snake_case，如`get_user_info`
- TypeScript：camelCase，如`getUserInfo`

## 项目结构规范

### 目录结构

```
src/
├── pages/           # 页面组件
│   ├── index.rsx
│   ├── about.rsx
│   └── users/
│       ├── index.rsx
│       └── [id].rsx
├── components/      # 可复用组件
│   ├── Header.rsx
│   ├── Footer.rsx
│   └── UserCard.rsx
├── layouts/         # 布局组件
│   ├── MainLayout.rsx
│   └── AdminLayout.rsx
├── api/            # API路由
│   ├── auth.rs
│   └── users.rs
├── utils/          # 工具函数
│   ├── validation.rs
│   └── helpers.ts
└── styles/         # 全局样式
    ├── globals.css
    └── variables.css
```

### 组件组织

#### 页面组件

```rsx
---
// 页面级别的服务端逻辑
async fn get_server_side_props(req: Request) -> Response {
    // 获取页面数据
}
---

<script>
// 页面级别的客户端逻辑
</script>

<template>
<!-- 页面模板 -->
</template>

<style>
/* 页面样式 */
</style>
```

#### 可复用组件

```rsx
---
// 组件级别的服务端逻辑（可选）
---

<script>
// 组件属性定义和客户端逻辑
const { title, content } = defineProps<{
    title: string;
    content: string;
}>();
</script>

<template>
<!-- 组件模板 -->
</template>

<style>
/* 组件样式 */
</style>
```

## 工具和配置

### 开发工具

- **IDE配置**：VS Code + Rust Analyzer + TypeScript
- **代码格式化**：rustfmt + biome
- **代码检查**：clippy + biome + oxlint
- **测试工具**：cargo test + vitest
