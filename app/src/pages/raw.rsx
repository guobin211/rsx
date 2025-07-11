---
use rsx::{Request, Response, json};

pub async fn get_server_props(req: Request) -> Response {
    Response::json!({
        "data": "<div>hello html</div>",
        "title": "html渲染"
    })
}
---

<script>
    import { defineProps } from 'rsx';
    import Meta from '../components/meta.rsx';
    // the server props
    const { data, title } = defineProps<{}>({});

    export default {
        components: [Meta],
    }
</script>

<template>
    <html lang="en-us">
        <head>
            <Meta></Meta>
            <title>{title}</title>
            <meta charset="UTF-8">
            <link rel="icon" type="image/svg+xml" href="/logo.svg" />
        </head>
        <body>
        <div id="app">
            <div>渲染html</div>
            {@html data}
        </div>
        </body>
    </html>
</template>

<style>
    #app {
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
    }
</style>
