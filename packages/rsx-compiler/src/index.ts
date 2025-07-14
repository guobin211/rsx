import * as fs from 'node:fs/promises'
import * as path from 'node:path'
import { type Module, parseSync } from '@swc/core'
import type { CheerioAPI } from 'cheerio'
import * as cheerio from 'cheerio'
import * as fg from 'fast-glob'
import type { RsxConfig } from './config'

export interface RsxFile {
    /**
     * 文件路径
     */
    file: string
    /**
     * 文件所在目录
     */
    dir: string
    /**
     * 文件内容
     */
    content: string
    /**
     * 当前文件匹配的页面路由
     */
    urls: string[]
}

export interface RsxFileAst extends RsxFile {
    rust: string | null
    script: {
        content: string
        ast: Module | null
    } | null
    template: {
        content: string
        ast: CheerioAPI | null
    } | null
    style: {
        content: string
    } | null
}

function parseRsxFile(file: RsxFile): RsxFileAst {
    const { content: source } = file
    // 提取 Rust Frontmatter
    const rustMatch = source.match(/---\r?\n([\s\S]*?)\r?\n---/)
    const rust = rustMatch ? rustMatch[1].trim() : null

    // 提取 <script> 块
    const scriptMatch = source.match(/<script[^>]*>\r?\n([\s\S]*?)\r?\n<\/script>/)
    const scriptContent = scriptMatch ? scriptMatch[1].trim() : null

    // 提取 <template> 块
    const templateMatch = source.match(/<template[^>]*>\r?\n([\s\S]*?)\r?\n<\/template>/)
    const templateContent = templateMatch ? templateMatch[1].trim() : null

    // 提取 <style> 块
    const styleMatch = source.match(/<style[^>]*>\r?\n([\s\S]*?)\r?\n<\/style>/)
    const styleContent = styleMatch ? styleMatch[1].trim() : null

    // 3. 对每个块的内容进行子解析
    const ast: RsxFileAst = {
        ...file,
        rust,
        script: scriptContent
            ? {
                  content: scriptContent,
                  ast: parseSync(scriptContent, { syntax: 'typescript', tsx: true })
              }
            : null,
        template: templateContent
            ? {
                  content: templateContent,
                  ast: cheerio.load(templateContent, { xmlMode: true })
              }
            : null,
        style: styleContent
            ? {
                  content: styleContent
              }
            : null
    }

    return ast
}

async function getRsxFiles(config: Required<RsxConfig>) {
    const files = await fg.async(`${config.root}/src/**/*.rsx`, {
        cwd: config.root,
        absolute: true,
        onlyFiles: true,
        ignore: ['**/node_modules/**', '**/dist/**']
    })

    const pages: RsxFile[] = []
    const components: RsxFile[] = []

    for (const file of files) {
        const content = await fs.readFile(file, 'utf8')
        if (file.includes('src/pages/')) {
            const absolutePath = file.replace(config.pages, '')
            const urls = [absolutePath.replace('.rsx', ''), absolutePath.replace('.rsx', '.html')]
            pages.push({
                file,
                urls,
                content,
                dir: path.dirname(file)
            })
        } else {
            components.push({
                file,
                content,
                dir: path.dirname(file),
                urls: []
            })
        }
    }

    return {
        pages,
        components
    }
}

export class RsxCompilerContext {
    root: string
    config: Required<RsxConfig>
    pages: RsxFile[] = []
    components: RsxFile[] = []

    constructor(config: RsxConfig) {
        this.root = config.root
        this.config = {
            ...config,
            pages: config.pages || path.join(this.root, 'src/pages'),
            output: config.output || path.join(this.root, 'dist'),
            cache: config.cache || path.join(this.root, 'node_modules/.rsx')
        }
    }

    initContext = async () => {
        const rsxFiles = await getRsxFiles(this.config)
        this.pages = rsxFiles.pages
        this.components = rsxFiles.components
    }
}

export class RsxCompiler {
    root: string
    astPages: RsxFileAst[] = []
    astComponents: RsxFileAst[] = []

    constructor(public ctx: RsxCompilerContext) {
        this.root = ctx.root
    }

    initAst = async () => {
        await this.ctx.initContext()
        this.astPages = this.ctx.pages.map(parseRsxFile)
        this.astComponents = this.ctx.components.map(parseRsxFile)
    }

    async build() {
        await this.initAst()
        for (const page of this.astPages) {
            await this.buildPage(page)
        }
    }

    async buildPage(page: RsxFileAst) {
        if (page.file.includes('csr')) {
        }
    }

    async getPageHtml(page: RsxFileAst): Promise<string> {
        return page.template?.content || ''
    }
}
