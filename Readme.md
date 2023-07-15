# Valkyrie Language Support for VS Code

Comprehensive language support for the Valkyrie programming language, including syntax highlighting, code completion,
diagnostics, and more via the Legion Language Server Protocol.

## Features

- **Syntax Highlighting**: Full syntax support for Valkyrie and related file formats
- **Code Completion**: Intelligent code completion powered by Legion LSP
- **Diagnostics**: Real-time error detection and warnings
- **Code Formatting**: Built-in code formatter
- **AST Viewer**: Inspect the abstract syntax tree of your code

## Supported Languages

| Language | Extensions                      | Description                          |
|----------|---------------------------------|--------------------------------------|
| Valkyrie | `.v`, `.vk`, `.vx`, `.valkyrie` | Main programming language            |
| VOML     | `.voml`                         | TOML-style configuration files       |
| VOC      | `.vc`, `.voc`                   | Vue SFC-style single file components |
| VON      | `.von`                          | JSON5-style data files               |

## Requirements

- VS Code 1.75.0 or higher
- Legion LSP server (auto-detected from PATH)

## Installation

### From VSIX

```bash
pnpm run package
code --install-extension valkyrie4vscode-0.1.0.vsix
```

### From Source

```bash
git clone https://github.com/nyar-lang/valkyrie
cd valkyrie4vscode
pnpm install
pnpm run compile
```

## Configuration

This extension contributes the following settings:

| Setting                        | Default | Description                                                  |
|--------------------------------|---------|--------------------------------------------------------------|
| `valkyrie.lsp.path`            | `""`    | Path to Legion LSP server binary. Leave empty to auto-detect |
| `valkyrie.lsp.args`            | `[]`    | Additional arguments for the language server                 |
| `valkyrie.lsp.trace.server`    | `"off"` | Trace communication between VS Code and LSP                  |
| `valkyrie.format.enabled`      | `true`  | Enable code formatting                                       |
| `valkyrie.format.tabSize`      | `4`     | Tab size for formatting                                      |
| `valkyrie.format.insertSpaces` | `true`  | Insert spaces instead of tabs                                |

## Commands

| Command                             | Description                      |
|-------------------------------------|----------------------------------|
| `Valkyrie: Restart Language Server` | Restart the Legion LSP server    |
| `Valkyrie: Show AST`                | Display the abstract syntax tree |

## Development

### Build

```bash
pnpm run compile
```

### Watch Mode

```bash
pnpm run watch
```

### Lint

```bash
pnpm run lint
```

### Format

```bash
pnpm run format
```

### Package

```bash
pnpm run package
```

## License

MPL2.0 License - see [LICENSE](LICENSE) for details.

## Links

- **Repository**: https://github.com/nyar-lang/valkyrie
- **Publisher**: nyar-lang
