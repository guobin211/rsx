import {
    type CompletionItem,
    CompletionItemKind,
    createConnection,
    type Diagnostic,
    DiagnosticSeverity,
    DidChangeConfigurationNotification,
    type DocumentDiagnosticReport,
    DocumentDiagnosticReportKind,
    type InitializeParams,
    type InitializeResult,
    ProposedFeatures,
    type TextDocumentPositionParams,
    TextDocumentSyncKind,
    TextDocuments
} from 'vscode-languageserver/node'

import { TextDocument } from 'vscode-languageserver-textdocument'

const connection = createConnection(ProposedFeatures.all)

const documents = new TextDocuments(TextDocument)

let hasConfigurationCapability = false
let hasWorkspaceFolderCapability = false
let hasDiagnosticRelatedInformationCapability = false

connection.onInitialize((params: InitializeParams) => {
    const capabilities = params.capabilities
    hasConfigurationCapability = !!(
        capabilities.workspace && !!capabilities.workspace.configuration
    )
    hasWorkspaceFolderCapability = !!(
        capabilities.workspace && !!capabilities.workspace.workspaceFolders
    )
    hasDiagnosticRelatedInformationCapability =
        !!capabilities.textDocument?.publishDiagnostics?.relatedInformation

    const result: InitializeResult = {
        capabilities: {
            textDocumentSync: TextDocumentSyncKind.Incremental,
            completionProvider: {
                resolveProvider: true
            },
            diagnosticProvider: {
                interFileDependencies: false,
                workspaceDiagnostics: false
            }
        }
    }
    if (hasWorkspaceFolderCapability) {
        result.capabilities.workspace = {
            workspaceFolders: {
                supported: true
            }
        }
    }
    return result
})

connection.onInitialized(() => {
    if (hasConfigurationCapability) {
        connection.client.register(DidChangeConfigurationNotification.type, undefined)
    }
    if (hasWorkspaceFolderCapability) {
        connection.workspace.onDidChangeWorkspaceFolders((_event) => {
            connection.console.log('Workspace folder change event received.')
        })
    }
})

// The example settings
interface ExampleSettings {
    maxNumberOfProblems: number
}

// The global settings, used when the `workspace/configuration` request is not supported by the client.
// Please note that this is not the case when using this server with the client provided in this example
// but could happen with other clients.
const defaultSettings: ExampleSettings = { maxNumberOfProblems: 1000 }
let globalSettings: ExampleSettings = defaultSettings

// Cache the settings of all open documents
const documentSettings = new Map<string, Thenable<ExampleSettings>>()

connection.onDidChangeConfiguration((change) => {
    if (hasConfigurationCapability) {
        documentSettings.clear()
    } else {
        globalSettings = change.settings.languageServerExample || defaultSettings
    }
    connection.languages.diagnostics.refresh()
})

function getDocumentSettings(resource: string): Thenable<ExampleSettings> {
    if (!hasConfigurationCapability) {
        return Promise.resolve(globalSettings)
    }
    let result = documentSettings.get(resource)
    if (!result) {
        result = connection.workspace.getConfiguration({
            scopeUri: resource,
            section: 'RsxLanguageServer'
        })
        documentSettings.set(resource, result)
    }
    return result
}

documents.onDidClose((e) => {
    documentSettings.delete(e.document.uri)
})

connection.languages.diagnostics.on(async (params) => {
    const document = documents.get(params.textDocument.uri)
    if (document !== undefined) {
        return {
            kind: DocumentDiagnosticReportKind.Full,
            items: await validateTextDocument(document)
        } satisfies DocumentDiagnosticReport
    }
    return {
        kind: DocumentDiagnosticReportKind.Full,
        items: []
    } satisfies DocumentDiagnosticReport
})

documents.onDidChangeContent((change) => {
    validateTextDocument(change.document)
})

async function validateTextDocument(textDocument: TextDocument): Promise<Diagnostic[]> {
    const settings = await getDocumentSettings(textDocument.uri)
    const text = textDocument.getText()
    const pattern = /\b[A-Z]{2,}\b/g
    let m: RegExpExecArray | null

    let problems = 0
    const diagnostics: Diagnostic[] = []
    while ((m = pattern.exec(text)) && problems < settings.maxNumberOfProblems) {
        problems++
        const diagnostic: Diagnostic = {
            severity: DiagnosticSeverity.Warning,
            range: {
                start: textDocument.positionAt(m.index),
                end: textDocument.positionAt(m.index + m[0].length)
            },
            message: `${m[0]} is all uppercase.`,
            source: 'ex'
        }
        if (hasDiagnosticRelatedInformationCapability) {
            diagnostic.relatedInformation = [
                {
                    location: {
                        uri: textDocument.uri,
                        range: Object.assign({}, diagnostic.range)
                    },
                    message: 'Spelling matters'
                },
                {
                    location: {
                        uri: textDocument.uri,
                        range: Object.assign({}, diagnostic.range)
                    },
                    message: 'Particularly for names'
                }
            ]
        }
        diagnostics.push(diagnostic)
    }
    return diagnostics
}

connection.onDidChangeWatchedFiles((_change) => {
    connection.console.log('We received a file change event')
})

// This handler provides the initial list of the completion items.
connection.onCompletion((_textDocumentPosition: TextDocumentPositionParams): CompletionItem[] => {
    return [
        {
            label: 'TypeScript',
            kind: CompletionItemKind.Text,
            data: 1
        },
        {
            label: 'JavaScript',
            kind: CompletionItemKind.Text,
            data: 2
        }
    ]
})

connection.onCompletionResolve((item: CompletionItem): CompletionItem => {
    if (item.data === 1) {
        item.detail = 'TypeScript details'
        item.documentation = 'TypeScript documentation'
    } else if (item.data === 2) {
        item.detail = 'JavaScript details'
        item.documentation = 'JavaScript documentation'
    }
    return item
})

connection.onHover((params) => {
    console.log(`hover ${params.textDocument.uri} ${params.position}`)
    return null
})
documents.listen(connection)

connection.listen()
