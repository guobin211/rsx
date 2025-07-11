# rsx

> A Rust framework for building web applications with a focus on server-side rendering and component-based architecture.

## The Rsx Framework

`rsx` is a server-side web framework. it's like `jsx` + `rust`.

```rsx
---
use rsx::{Request, Response};
use api::user::{UserInfo};

async fn get_server_props(req: Request) -> Response {
    let cookies = req.cookies();
    match UserInfo::from_cookies(cookies).await {
        Ok(user) {
            Response::json!({
                "code": 0,
                "user": user,
                "msg": ""
            })
        }
        Err(err) {
            Response::json!({
                "code": -1,
                "user": None,
                "msg": format!("{}", err)
            })
        }
    }
}
---

<script>
    import { defineProps } from 'rsx';
    import Header from 'components/header.tsx';
    const { user } = defineProps({});
</script>

<template>
    <div>
        <h1 class="title">Hello {user.name}!</h1>
        <Header client:only="react" user={user}></Header>
    </div>
</template>

<style>
    .title {
        font-size: 1.5em;
        text-align: center;
    }
</style>
```

## Quick Start

```bash
    pnpm install
    cargo check
```

## Setup

```bash
rustup default nightly
rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi
cargo install cargo-watch wasm-pack
```
