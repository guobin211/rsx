import * as path from 'node:path'
import { type ExtensionContext, workspace } from 'vscode'

import {
    LanguageClient,
    type LanguageClientOptions,
    type ServerOptions,
    TransportKind
} from 'vscode-languageclient/node'

let client: LanguageClient

export function activate(context: ExtensionContext) {
    const serverModule = context.asAbsolutePath(path.join('dist', 'server.js'))

    const serverOptions: ServerOptions = {
        run: { module: serverModule, transport: TransportKind.ipc },
        debug: {
            module: serverModule,
            transport: TransportKind.ipc
        }
    }

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'rsx' }],
        synchronize: {
            fileEvents: workspace.createFileSystemWatcher('**/.rsx')
        }
    }

    client = new LanguageClient(
        'RsxLanguageServer',
        'RsxLanguageServer',
        serverOptions,
        clientOptions
    )

    client.start()
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined
    }
    return client.stop()
}
