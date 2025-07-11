import * as path from 'node:path'
import { fileURLToPath } from 'node:url'
import react from '@vitejs/plugin-react-swc'
import { RsxPlugin } from 'rsx-plugin-vite/dist/index'
import { defineConfig } from 'vite'

const dirname = path.dirname(fileURLToPath(import.meta.url))

export default defineConfig({
    root: dirname,
    publicDir: path.join(dirname, 'public'),
    plugins: [RsxPlugin(), react()]
})
