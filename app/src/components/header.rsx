<script>
    import { defineProps } from 'rsx'

    const { userInfo } = defineProps();
</script>

<template>
    <header class="header">
        <ul class="menu">
            <li>
                <a href="/app/app-rsx/public">Home</a>
            </li>
            <li>
                <a href="/ssr">Ssr</a>
            </li>
            <li>
                <a href="/csr">Csr</a>
            </li>
        </ul>
        <div class="user">
            <img src={{userInfo.avatar}} loading="lazy" />
        </div>
    </header>
</template>

<style>
    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        flex-direction: row;
        padding: 10px;
        background-color: #333333;
        color: #fff;
    }

    .menu {
        display: flex;
        justify-content: space-between;
        align-items: center;
        flex-direction: row;
        margin: 0;
        padding: 0;
        list-style: none;
    }
</style>
