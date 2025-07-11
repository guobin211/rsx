# rsx-vscode

## Description

This package provides a language server for the rsx language.

### About rsx

`rsx` is a server side web framework. it's like `jsx` + `rust`.

```rsx
---
use rsx::{ Request, Response };

async fn get_server_props(request: Request) -> Response {
    Response::json!({
        "title": "rsx project",
        "pageName": "index.html"
    })
}
---

<script>
    import { defineProps } from 'rsx';
    import Content from './components/Content.tsx';
    const { title, pageName } = defineProps<{
        title: string;
        pageName: string;
    }>();
    export default {
        components: [Content]
    }
</script>

<template>
    <div>
        <h1 class="title">welcome to {pageName}</h1>
        <Content title={title}></Content>
    </div>
</template>

<style>
    .title {
        font-size: 16px;
        text-align: center;
    }
</style>
```
