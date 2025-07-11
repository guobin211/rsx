import { defineConfig } from 'wxt'

// See https://wxt.dev/api/config.html
export default defineConfig({
    srcDir: 'src',
    modulesDir: 'modules',
    outDir: 'dist',
    publicDir: 'public',
    entrypointsDir: 'entrypoints',
    modules: ['@wxt-dev/module-react']
})
