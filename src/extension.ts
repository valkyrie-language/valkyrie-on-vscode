import { spawn } from "node:child_process";
import * as path from "node:path";
import {
    ConfigurationTarget,
    type Disposable,
    type ExtensionContext,
    type OutputChannel,
    StatusBarAlignment,
    type StatusBarItem,
    Uri,
    ViewColumn,
    commands,
    window,
    workspace,
} from "vscode";
import {
    type Executable,
    LanguageClient,
    type LanguageClientOptions,
    type ServerOptions,
    State,
    Trace,
} from "vscode-languageclient/node";

/**
 * Valkyrie VSCode Extension Entry Point
 * Provides LSP client integration for the Valkyrie programming language
 */
let client: LanguageClient | undefined;
let statusBarItem: StatusBarItem;
let outputChannel: OutputChannel;
let traceChannel: OutputChannel;
let disposables: Disposable[] = [];

/**
 * Server status enumeration
 */
enum ServerStatus {
    Starting = "starting",
    Running = "running",
    Stopped = "stopped",
    Error = "error",
}

/**
 * Current server status
 */
let currentStatus: ServerStatus = ServerStatus.Stopped;

/**
 * Extension activation handler
 * Called when the extension is activated on Valkyrie files
 */
export function activate(context: ExtensionContext): void {
    initializeOutputChannels(context);
    initializeStatusBar(context);
    initializeCommands(context);
    startLanguageServer(context);
}

/**
 * Initialize output channels for LSP logging
 */
function initializeOutputChannels(context: ExtensionContext): void {
    outputChannel = window.createOutputChannel("Valkyrie LSP");
    traceChannel = window.createOutputChannel("Valkyrie LSP Trace");
    context.subscriptions.push(outputChannel, traceChannel);
    logToOutput("Valkyrie LSP extension activated");
}

/**
 * Log message to output channel with timestamp
 */
function logToOutput(message: string): void {
    const timestamp = new Date().toISOString();
    outputChannel.appendLine(`[${timestamp}] ${message}`);
}

/**
 * Initialize the status bar item
 */
function initializeStatusBar(context: ExtensionContext): void {
    statusBarItem = window.createStatusBarItem(StatusBarAlignment.Right, 100);
    updateStatusBar(ServerStatus.Stopped, "Valkyrie Language Server");
    statusBarItem.command = "valkyrie.showServerInfo";
    statusBarItem.show();
    context.subscriptions.push(statusBarItem);
}

/**
 * Update status bar based on server status
 */
function updateStatusBar(status: ServerStatus, details?: string): void {
    currentStatus = status;
    switch (status) {
        case ServerStatus.Starting:
            statusBarItem.text = "$(sync~spin) Valkyrie";
            statusBarItem.tooltip = "Valkyrie Language Server is starting...";
            break;
        case ServerStatus.Running:
            statusBarItem.text = "$(check) Valkyrie";
            statusBarItem.tooltip = details || "Valkyrie Language Server is active";
            break;
        case ServerStatus.Stopped:
            statusBarItem.text = "$(circle-outline) Valkyrie";
            statusBarItem.tooltip = "Valkyrie Language Server is stopped. Click to start.";
            break;
        case ServerStatus.Error:
            statusBarItem.text = "$(error) Valkyrie";
            statusBarItem.tooltip = details || "Valkyrie Language Server error. Click for details.";
            break;
    }
}

/**
 * Check if the LSP server executable exists in PATH or at configured path
 */
async function checkServerExists(
    serverPath: string
): Promise<{ exists: boolean; resolvedPath: string; error?: string }> {
    return new Promise((resolve) => {
        const isSubcommand = serverPath.includes(" ");
        const [cmd, ...args] = serverPath.split(" ");
        const testArgs = isSubcommand ? [...args, "--help"] : ["--help"];

        const proc = spawn(cmd, testArgs, {
            shell: true,
            timeout: 5000,
        });

        let stdout = "";
        let stderr = "";

        proc.stdout.on("data", (data) => {
            stdout += data.toString();
        });

        proc.stderr.on("data", (data) => {
            stderr += data.toString();
        });

        proc.on("error", (err) => {
            resolve({
                exists: false,
                resolvedPath: serverPath,
                error: `Failed to execute server: ${err.message}`,
            });
        });

        proc.on("close", (code) => {
            if (code === 0 || code === 1 || stdout.length > 0) {
                resolve({
                    exists: true,
                    resolvedPath: serverPath,
                });
            } else {
                resolve({
                    exists: false,
                    resolvedPath: serverPath,
                    error: `Server exited with code ${code}. ${stderr || stdout}`,
                });
            }
        });
    });
}

/**
 * Detect LSP server and return appropriate path or error
 */
async function detectServer(): Promise<{ path: string; error?: string }> {
    const config = workspace.getConfiguration("valkyrie");
    const configuredPath = config.get<string>("lsp.path");

    const candidates = configuredPath
        ? [configuredPath]
        : ["legion lsp", "valkyrie-lsp", "legion-lsp", "valkyrie", "legion"];

    logToOutput(`Searching for LSP server. Candidates: ${candidates.join(", ")}`);

    for (const candidate of candidates) {
        logToOutput(`Checking for server at: ${candidate}`);
        const result = await checkServerExists(candidate);
        if (result.exists) {
            logToOutput(`Found server at: ${candidate}`);
            window.showInformationMessage(`Valkyrie LSP server found: ${candidate}`);
            return { path: candidate };
        }
        if (result.error) {
            logToOutput(`Check failed for ${candidate}: ${result.error}`);
        }
    }

    const errorMsg = configuredPath
        ? `Configured server path '${configuredPath}' not found or not executable.`
        : `No Valkyrie LSP server found. Tried: ${candidates.join(", ")}`;

    logToOutput(errorMsg);
    return { path: "", error: errorMsg };
}

/**
 * Show server configuration quick pick
 */
async function showServerConfiguration(): Promise<void> {
    const action = await window.showErrorMessage(
        "Valkyrie LSP server not found. Please configure the server path.",
        "Configure Path",
        "Open Settings",
        "View Logs"
    );

    switch (action) {
        case "Configure Path": {
            const newPath = await window.showInputBox({
                prompt: "Enter the path to the Valkyrie LSP server executable",
                placeHolder: "valkyrie-lsp",
                value: "",
            });
            if (newPath) {
                const config = workspace.getConfiguration("valkyrie");
                await config.update("lsp.path", newPath, ConfigurationTarget.Global);
                window.showInformationMessage(`Server path updated to: ${newPath}. Restarting...`);
                commands.executeCommand("valkyrie.restartServer");
            }
            break;
        }
        case "Open Settings":
            commands.executeCommand("workbench.action.openSettings", "valkyrie.lsp");
            break;
        case "View Logs":
            outputChannel.show();
            break;
    }
}

/**
 * Initialize extension commands
 */
function initializeCommands(context: ExtensionContext): void {
    const restartCommand = commands.registerCommand("valkyrie.restartServer", async () => {
        logToOutput("Restarting language server...");
        updateStatusBar(ServerStatus.Starting);

        if (client) {
            try {
                await client.stop();
                logToOutput("Previous client stopped");
            } catch (e) {
                const errorMessage = e instanceof Error ? e.message : String(e);
                logToOutput(`Error stopping client: ${errorMessage}`);
            }
        }

        await startLanguageServer(context);
    });

    const showAstCommand = commands.registerCommand("valkyrie.showAst", async () => {
        const editor = window.activeTextEditor;
        if (!editor || editor.document.languageId !== "valkyrie") {
            window.showWarningMessage("Please open a Valkyrie file first");
            return;
        }

        if (!client || currentStatus !== ServerStatus.Running) {
            window.showErrorMessage("Language server is not running");
            return;
        }

        const uri = editor.document.uri.toString();
        try {
            const ast = await client.sendRequest("valkyrie/getAst", { uri });
            const doc = await workspace.openTextDocument({
                content: JSON.stringify(ast, null, 2),
                language: "json",
            });
            await window.showTextDocument(doc, ViewColumn.Beside);
        } catch (e) {
            const errorMessage = e instanceof Error ? e.message : String(e);
            window.showErrorMessage(`Failed to get AST: ${errorMessage}`);
        }
    });

    const showServerInfoCommand = commands.registerCommand("valkyrie.showServerInfo", async () => {
        const config = workspace.getConfiguration("valkyrie");
        const serverPath = config.get<string>("lsp.path") || "auto-detect";

        const items = [`Status: ${currentStatus}`, `Server Path: ${serverPath}`, "View Logs"];

        const selection = await window.showQuickPick(items, {
            placeHolder: "Valkyrie Language Server Info",
        });

        if (selection === "View Logs") {
            outputChannel.show();
        }
    });

    const showLogsCommand = commands.registerCommand("valkyrie.showLogs", () => {
        outputChannel.show();
    });

    const stopServerCommand = commands.registerCommand("valkyrie.stopServer", async () => {
        if (client) {
            logToOutput("Stopping language server...");
            await client.stop();
            updateStatusBar(ServerStatus.Stopped);
            window.showInformationMessage("Valkyrie Language Server stopped");
        }
    });

    context.subscriptions.push(
        restartCommand,
        showAstCommand,
        showServerInfoCommand,
        showLogsCommand,
        stopServerCommand
    );
}

/**
 * Start the language server
 */
async function startLanguageServer(context: ExtensionContext): Promise<void> {
    updateStatusBar(ServerStatus.Starting);
    logToOutput("Starting language server...");

    const detection = await detectServer();
    if (detection.error) {
        logToOutput(`Server detection failed: ${detection.error}`);
        updateStatusBar(ServerStatus.Error, detection.error);
        await showServerConfiguration();
        return;
    }

    const config = workspace.getConfiguration("valkyrie");
    const serverArgs = config.get<string[]>("lsp.args") || [];
    const traceLevel = config.get<string>("lsp.trace.server") || "off";

    const run: Executable = {
        command: detection.path,
        args: ["--stdio", ...serverArgs],
        options: {
            env: {
                ...process.env,
                RUST_LOG: "info",
            },
        },
    };

    logToOutput(`Server command: ${detection.path} --stdio ${serverArgs.join(" ")}`);

    const serverOptions: ServerOptions = {
        run,
        debug: run,
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: "file", language: "valkyrie" }],
        synchronize: {
            fileEvents: [
                workspace.createFileSystemWatcher("**/*.vk"),
                workspace.createFileSystemWatcher("**/legion.json"),
                workspace.createFileSystemWatcher("**/legions.json"),
            ],
        },
        traceOutputChannel: traceChannel,
        outputChannel: outputChannel,
    };

    client = new LanguageClient("valkyrieLanguageServer", "Valkyrie Language Server", serverOptions, clientOptions);

    client.setTrace(Trace.fromString(traceLevel));

    client.onDidChangeState((event) => {
        logToOutput(`Client state changed: ${State[event.oldState]} -> ${State[event.newState]}`);

        if (event.newState === State.Running) {
            updateStatusBar(ServerStatus.Running, "Valkyrie Language Server is active");
            logToOutput("Language server is now running");
        } else if (event.newState === State.Stopped) {
            if (currentStatus !== ServerStatus.Stopped) {
                updateStatusBar(ServerStatus.Error, "Language server stopped unexpectedly");
            }
            logToOutput("Language server stopped");
        }
    });

    try {
        await client.start();
        logToOutput("Language server started successfully");
    } catch (error) {
        const errorMessage = error instanceof Error ? error.message : String(error);
        logToOutput(`Failed to start language server: ${errorMessage}`);
        updateStatusBar(ServerStatus.Error, errorMessage);

        const action = await window.showErrorMessage(
            `Failed to start Valkyrie Language Server: ${errorMessage}`,
            "View Logs",
            "Restart Server",
            "Configure"
        );

        switch (action) {
            case "View Logs":
                outputChannel.show();
                break;
            case "Restart Server":
                commands.executeCommand("valkyrie.restartServer");
                break;
            case "Configure":
                commands.executeCommand("workbench.action.openSettings", "valkyrie.lsp");
                break;
        }
    }

    context.subscriptions.push({
        dispose: async () => {
            if (client) {
                await client.stop();
            }
        },
    });
}

/**
 * Extension deactivation handler
 * Called when the extension is deactivated
 */
export async function deactivate(): Promise<void> {
    logToOutput("Extension deactivating...");

    for (const disposable of disposables) {
        disposable.dispose();
    }
    disposables = [];

    if (client) {
        await client.stop();
        client = undefined;
    }

    logToOutput("Extension deactivated");
}
