---
use rsx::{Request, Response, json};

pub async fn get_server_props(req: Request) -> Response {
    Response::json!({
        "data": json!(["a", "b", "c"]),
        "title": "列表渲染"
    })
}
---

<script>
    import { defineProps } from 'rsx';
    import Meta from '../components/meta.rsx';
    const { data, title } = defineProps<{}>({});

    export default {
        components: [Meta]
    }
</script>

<template>
    <html lang="en-us">
        <head>
            <Meta />
            <title>{title}</title>
            <meta charset="UTF-8">
            <link rel="icon" type="image/svg+xml" href="/logo.svg" />
        </head>
        <body>
        <div id="app">
            <ul>
                {#each data as item, index}
                    <li bind:key={index}>
                        {item}
                    </li>
                {/each}
            </ul>
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
