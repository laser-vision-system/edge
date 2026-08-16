# Contributing

Do not push to `main`; only push directly to `dev` if it is a minor change.

For most development work: create a branch off `dev`, add your commits, and push the branch. Then create a pull request to merge into `dev`. Once the pull request has been accepted, make a pull request into `main` to synchronise.

Prefer to rebase merge if possible. Only use a squash merge if necessary.

## Formatting Guidelines

> **Use tab indentation.**

Formatting is handled by `rustfmt`.

### Vscode

If you are using Vscode, you will need these extensions:

| Name | Link |
| - | - |
| rust-analyzer | https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer |
| Dependi | https://marketplace.visualstudio.com/items?itemName=fill-labs.dependi |

These extensions are also recommended:

| Name | Link |
| - | - |
| Rust Syntax | https://marketplace.visualstudio.com/items?itemName=dustypomerleau.rust-syntax |
| CodeLLDB | https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb |
| Even Better TOML | https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml |
| Rust Doc Viewer | https://marketplace.visualstudio.com/items?itemName=JScearcy.rust-doc-viewer |
| Rust Mod Generator | https://marketplace.visualstudio.com/items?itemName=ZhangYue.rust-mod-generator |


Recommend settings:

```json
"[rust]": {
	"editor.formatOnSave": true,
	"editor.quickSuggestions": {
		"comments": false,
		"other": true,
		"strings": true
	}
},
"editor.inlayHints.enabled": "onUnlessPressed", // Press `ctrl + alt` to hide.
"editor.inlayHints.maximumLength": 0,
"editor.semanticTokenColorCustomizations": {
	"rules": {
		"function.unsafe": "#efb063",
		"method.unsafe": "#efb063",
		"operator.unsafe": "#efb063"
	}
},
"rust-analyzer.assist.preferSelf": true,
"rust-analyzer.imports.granularity.enforce": true,
"rust-analyzer.imports.prefix": "self",
"rust-analyzer.inlayHints.closingBraceHints.minLines": 16,
"rust-analyzer.inlayHints.closureStyle": "rust_analyzer",
"rust-analyzer.inlayHints.maxLength": null,
"rust-analyzer.inlayHints.parameterHints.missingArguments.enable": true,
// Matches the inlay hints to the "Dark Modern" theme.
"workbench.colorCustomizations": {
	"editorInlayHint.background": "#00000017",
	"editorInlayHint.foreground": "#4e6a85",
	"editorInlayHint.parameterBackground": "#00000017",
	"editorInlayHint.parameterForeground": "#858555",
	"editorInlayHint.typeBackground": "#00000017",
	"editorInlayHint.typeForeground": "#4f8277",
}
```
