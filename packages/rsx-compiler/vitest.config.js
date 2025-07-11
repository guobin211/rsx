import { defineConfig } from 'vitest/config'

export default defineConfig({
    root: __dirname,
    test: {
        environment: 'node',
        testTimeout: 120 * 1000
    }
})
