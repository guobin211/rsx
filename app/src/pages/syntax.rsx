---
use rsx::{Request, Response};

// hightlight struct
struct User {
    first_name: String,
    last_name: String,
}

// hightlight trait
trait Json {
    fn get_name() -> String
}

// hightlight impl
impl Json for User {
    fn get_name(self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

// hightlight function
pub async fn get_server_props(req: Request) -> Response {
    Response::json!({
        "title": "this is server props",
        "url": req.url,
    })
}
---

<script>
    import { defineProps } from 'rsx';
    const { title } = defineProps<{}>({});
    const text = `welcome to ${title}`;
</script>

<template>
    <div>
        <div>
            <h1>template + style + script + jsx</h1>
            {text}
        </div>
    </div>
</template>

<style>
    div {
        display: flex;
    }
</style>
