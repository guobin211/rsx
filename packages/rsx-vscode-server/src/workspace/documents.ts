export type DocumentUri = string
export type DocumentBody = string

export interface TextDocumentIdentifier {
    uri: DocumentUri
}

export interface VersionedTextDocumentIdentifier extends TextDocumentIdentifier {
    version: number
}

export interface TextDocumentContentChangeEvent {
    text: string
}

const documents = new Map<DocumentUri, DocumentBody>()

export function clearDocuments() {
    documents.clear()
}

export function updateDocuments(uri: DocumentUri, body: string) {
    documents.set(uri, body)
}

export function getDocument(uri: DocumentUri): DocumentBody | undefined {
    return documents.get(uri)
}
