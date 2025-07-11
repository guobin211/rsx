<script>
    import { defineProps } from 'rsx'

    const { version } = defineProps();
</script>

<template>
    <footer>
        <div>
            <h6>rsx version: {{version}}</h6>
        </div>
        <div>
            <div>rsx-compiler</div>
            <div>rsx-runtime</div>
            <div>rsx-server</div>
        </div>
    </footer>
</template>

<style>
    footer {
        display: flex;
        align-items: center;
        flex-direction: row;
    }
</style>
