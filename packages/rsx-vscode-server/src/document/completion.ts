import type { RequestMessage } from 'vscode-languageserver'
import type { Position } from '../types'
import { getDocument, type TextDocumentIdentifier } from '../workspace/documents'

const MAX_LENGTH = 1000

const words = ['Header', 'Footer', 'Login']

type CompletionItem = {
    label: string
}

interface CompletionList {
    isIncomplete: boolean
    items: CompletionItem[]
}

interface TextDocumentPositionParams {
    textDocument: TextDocumentIdentifier
    position: Position
}

export interface CompletionParams extends TextDocumentPositionParams {}

/**
 * 代码自动补全
 */
export const completion = (message: RequestMessage): CompletionList | null => {
    const params = message.params as CompletionParams
    const content = getDocument(params.textDocument.uri)

    if (!content) {
        return null
    }

    const currentLine = content.split('\n')[params.position.line]
    const lineUntilCursor = currentLine.slice(0, params.position.character)
    const currentPrefix = lineUntilCursor.replace(/.*[\W ](.*?)/, '$1')

    const items = words
        .filter((word) => {
            return word.startsWith(currentPrefix)
        })
        .slice(0, MAX_LENGTH)
        .map((word) => {
            return { label: word }
        })

    return {
        isIncomplete: items.length === MAX_LENGTH,
        items
    }
}
