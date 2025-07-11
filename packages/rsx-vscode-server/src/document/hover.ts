import type { RequestMessage } from 'vscode-languageserver'
import type { Position, Range } from '../types'
import type { DocumentUri } from '../workspace/documents'

type HoverParams = {
    textDocument: { uri: DocumentUri }
    position: Position
}

type Hover = {
    contents: {
        kind: 'markdown'
        value: string
    }
    range: Range
}

/**
 * hover效果
 */
export const hover = (message: RequestMessage): Hover | null => {
    const params = message.params as HoverParams
    console.log('[hover] params:', params)
    return null
}
