import type { RequestMessage } from 'vscode-languageserver'
import type { Range } from '../types'

import { getDocument, type TextDocumentIdentifier } from '../workspace/documents'

interface DocumentDiagnosticParams {
    textDocument: TextDocumentIdentifier
}

const DiagnosticSeverity = {
    Error: 1,
    Warning: 2,
    Information: 3,
    Hint: 4
} as const

type DiagnosticSeverityValue = 1 | 2 | 3 | 4

interface SpellingSuggestionData {
    wordSuggestions: string[]
    type: 'spelling-suggestion'
}

export interface Diagnostic {
    range: Range
    severity: DiagnosticSeverityValue
    source: 'LSP From Scratch'
    message: string
    data: SpellingSuggestionData
}

interface FullDocumentDiagnosticReport {
    kind: 'full'
    items: Diagnostic[]
}

export const diagnostic = (message: RequestMessage): FullDocumentDiagnosticReport | null => {
    const params = message.params as DocumentDiagnosticParams
    const content = getDocument(params.textDocument.uri)

    console.log('DiagnosticSeverity', DiagnosticSeverity)

    if (!content) {
        return null
    }

    const items: Diagnostic[] = []

    return {
        kind: 'full',
        items
    }
}
