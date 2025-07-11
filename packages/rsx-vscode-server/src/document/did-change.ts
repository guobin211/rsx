import type { NotificationMessage } from 'vscode-languageserver/node'
import type {
    TextDocumentContentChangeEvent,
    VersionedTextDocumentIdentifier
} from '../workspace/documents'

interface DidChangeTextDocumentParams {
    textDocument: VersionedTextDocumentIdentifier
    contentChanges: TextDocumentContentChangeEvent[]
}

export const didChange = async (message: NotificationMessage) => {
    const params = message.params as DidChangeTextDocumentParams
    if (params.textDocument.uri) {
    }
}
