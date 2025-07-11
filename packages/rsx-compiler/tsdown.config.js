import fs from 'node:fs'
import { NODEJS_BUILDIN_MODULES, VITE_MODULES } from 'configjs'
import { defineConfig } from 'tsdown/config'

const pkg = JSON.parse(fs.readFileSync('./package.json', 'utf-8'))
const dependencies = Object.keys(pkg.dependencies || {})
const peerDependencies = Object.keys(pkg.peerDependencies || {})

const external = [...dependencies, ...peerDependencies, ...NODEJS_BUILDIN_MODULES, ...VITE_MODULES]

export default [
    defineConfig({
        entry: 'src/index.ts',
        platform: 'node',
        format: 'esm',
        outDir: 'dist',
        clean: true,
        dts: true,
        define: {
            __VERSION__: JSON.stringify(pkg.version)
        },
        external
    })
]
