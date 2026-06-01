# Contributing

Do not push to main. Avoid pushing directly to dev.

Create a branch off dev, add your commits, and push the branch. Then create a pull request to merge into dev.

Prefer to rebase merge. Only use a squash merge if necessary.

## Formatting Guidelines

> **Use tab indentation.**

Code should use Black Formatting, but with tab indentation.

### Vscode

If you are using Vscode, you can use Ruff to handle formatting easily: https://marketplace.visualstudio.com/items?itemName=charliermarsh.ruff

Settings:

```json
"[python]": {
	"editor.codeActionsOnSave": {
		"source.organizeImports.ruff": "explicit"
	},
	"editor.defaultFormatter": "charliermarsh.ruff"
},
"ruff.configuration": {
	"format": {
		"indent-style": "tab"
	},
	"line-length": 88,
	"lint": {
		"isort": {
			"combine-as-imports": true,
			"lines-after-imports": 2
		},
		"select": [
			"E",
			"F",
			"I"
		]
	}
},
```

## Docstring formatting

Docstrings follow the NumPy styling.

Read: https://numpydoc.readthedocs.io/en/latest/format.html
