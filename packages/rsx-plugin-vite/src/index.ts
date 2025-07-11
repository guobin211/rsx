import * as path from 'node:path'
import { RsxCompiler, RsxCompilerContext } from 'rsx-compiler/src'
import { createLogger, type Plugin } from 'vite'

const logger = createLogger('info')

export const RsxPlugin: () => Plugin = () => {
    let ctx: RsxCompilerContext, compiler: RsxCompiler
    return {
        name: 'rsx',
        enforce: 'pre',
        configResolved: async (userConfig) => {
            const root = userConfig.root || process.cwd()
            const config = {
                root,
                pages: path.join(root, 'src', 'pages'),
                output: path.join(root, 'dist'),
                cache: path.join(root, 'node_modules/.cache/rsx')
            }
            ctx = new RsxCompilerContext(config)
            compiler = new RsxCompiler(ctx)
            logger.info(`rsx plugin config resolved: ${JSON.stringify(config)}`)
        },
        configureServer(server) {
            server.middlewares.use(async (req, res, next) => {
                const uri = req.url || ''
                logger.info(`[RSX] Request received: uri: ${uri}`)
                if (uri) {
                    const page = compiler.astPages.find((p) => p.urls.includes(uri))
                    if (page) {
                        res.setHeader('Content-Type', 'text/html')
                        const body = await compiler.getPageHtml(page)
                        res.write(body)
                        res.end()
                        return
                    }
                }
                next()
            })
        }
    }
}
