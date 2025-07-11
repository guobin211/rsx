<script>
    // ssr components
    import Meta from '../components/meta.rsx';
    // csr components
    import CsrApp from '../react/csr.app.tsx';

    export default {
        components: [Meta, CsrApp]
    }
</script>

<template>
    <head>
        <Meta></Meta>
        <title>客户端渲染</title>
        <meta charset="UTF-8">
        <link rel="icon" type="image/svg+xml" href="/logo.svg" />
    </head>
    <body>
        <div id="app">
            <CsrApp client:load></CsrApp>
        </div>
    </body>
</template>

<style>
    #app {
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
    }
</style>
