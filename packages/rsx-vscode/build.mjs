import fs from 'node:fs/promises'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import esbuild from 'esbuild'

async function build() {
    /**
     * @type {import('esbuild').BuildOptions}
     */
    const config = {
        entryPoints: {
            'dist/client': './src/client.ts'
        },
        outdir: '.',
        bundle: true,
        sourcemap: true,
        external: ['typescript', 'vscode', 'vscode-languageserver', 'vscode-languageclient'],
        format: 'cjs',
        platform: 'node',
        tsconfig: './tsconfig.json',
        define: { 'process.env.NODE_ENV': '"production"' },
        minify: process.argv.includes('--minify'),
        legalComments: 'none'
    }
    if (process.argv.includes('--watch')) {
        const ctx = await esbuild.context(config)
        console.log('watch rsx-vscode success')
        await ctx.watch()
    } else {
        await esbuild.build(config)
        console.log('build rsx-vscode success')
    }
}

async function copyServer() {
    const dirname = path.dirname(fileURLToPath(import.meta.url))
    const serverDistPath = path.resolve(dirname, '../rsx-vscode-server/dist')
    const files = await fs.readdir(serverDistPath)
    for (const file of files) {
        const srcPath = path.join(serverDistPath, file)
        const destPath = path.join('dist', file)
        await fs.copyFile(srcPath, destPath)
    }
}

await build()
try {
    await copyServer()
} catch (err) {
    console.error('copyServer.err', err)
    console.log('\n\n Please run "pnpm run build:server" to build the server first.')
    process.exit(1)
}
