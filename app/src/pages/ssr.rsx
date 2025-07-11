---
use rsx::{Request, Response, json};

pub async fn get_server_props(req: Request) -> Response {
    Response::json!({
        "code": 0,
        "userInfo": json!({ "name": "jack" }),
        "newsInfo": json!({ "title": "测试新闻" }),
        "newsList": json!([]),
        "title": "服务端渲染"
    })
}
---

<script>
    import { defineProps } from 'rsx';
    import Meta from '../components/meta.rsx';
    import Skeleton from '../components/skeleton.rsx';
    // csr components
    import NewsApp from '../react/news.app.tsx';
    // the server props
    const { userInfo, newsInfo, title } = defineProps<{}>({});

    export default {
        components: [Meta, Skeleton, NewsApp],
    }
</script>

<template>
    <html>
        <head>
            <Meta></Meta>
            <title>{title}</title>
            <meta charset="UTF-8">
            <link rel="icon" type="image/svg+xml" href="/logo.svg" />
        </head>
        <body>
            <div id="app">
                <Skeleton>
                    <NewsApp client:only="react" newsInfo={newsInfo}></NewsApp>
                </Skeleton>
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
