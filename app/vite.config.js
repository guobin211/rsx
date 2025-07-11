import * as path from 'node:path'
import { fileURLToPath } from 'node:url'
import react from '@vitejs/plugin-react-swc'
import { defineConfig } from 'vite'
import { RsxPlugin } from '../packages/rsx-plugin-vite/src/index'

const dirname = path.dirname(fileURLToPath(import.meta.url))

export default defineConfig({
    root: dirname,
    publicDir: path.join(dirname, 'public'),
    plugins: [RsxPlugin(), react()]
})
