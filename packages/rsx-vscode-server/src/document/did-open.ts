import type { NotificationMessage } from 'vscode-languageserver'
import type { DocumentUri } from '../workspace/documents'

type TextDocumentItem = {
    uri: DocumentUri
    languageId: string
    version: number
    text: string
}

interface DidOpenTextDocumentParams {
    textDocument: TextDocumentItem
}

export const didOpen = async (message: NotificationMessage) => {
    const params = message.params as DidOpenTextDocumentParams
    if (params.textDocument.uri) {
    }
}
