import * as path from 'node:path'
import type { ViteUserConfig } from 'vitest/config'

/**
 * 服务端数据id
 */
export const RSX_SERVER_PROPS_ID = '__rsx_script__'
/**
 * rsx客户端组件
 */
export const RSX_CLIENT_TAG_NAME = 'rsx'

/**
 * 客户端组件指令
 */
export const CLIENT_DIRECTIVE = {
    LOAD: 'client:load',
    IDLE: 'client:idle',
    VISIBLE: 'client:visible',
    MEDIA: 'client:media',
    ONLY: 'client:only'
} as const

/**
 * 通用指令
 */
export const COMMON_DIREACTIVE = {
    HTML: 'set:html',
    TEXT: 'set:text'
}

export type UserRsxConfig = RsxConfig & ViteUserConfig

export interface RsxConfig {
    root: string
    pages?: string
    output?: string
    cache?: string
}

export const getDefault = (): RsxConfig => {
    const root = process.cwd()
    return {
        root,
        pages: path.join(root, 'src/pages'),
        output: path.join(root, 'dist'),
        cache: path.join(root, 'node_modules/.cache/rsx')
    }
}
