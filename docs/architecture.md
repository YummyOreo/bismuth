| Crate   | Description    |
|--------------- | --------------- |
| bismuth-core | The entry point |
| bismuth-error | Error handling |
| bismuth-md | The main interface for markdown files |
| bismuth-lexer | Tokenizing markdown files |
| bismuth-parser | Parses tokenized files |
| bismuth-custom | Handles running plugins and templates |
| bismuth-html | Handles rendering, templating, and writing of files |

# Bismuth-Core:
The main entry point for the app. This handles cli argument and configuration details.
# Bismuth-Error:
This is the error handler for the program. It handles all errors that can be resolved/saved. This gives the user options and allows them to chose between them.
> This will be rewritten soon
# Bismuth-MD:
This is the Markdown file handling. This loads the markdown files from a dir, or a single file from a path. The paths are stored relative to the config dir.
## Bismuth-Lexer:
This is handling for the tokenizing a markdown file. Read more about this [here](../bismuth-lexer/readme.md)
## Bismuth-Parser:
This is for parsing a tokenized file into a AST
### Bismuth-Custom:
This handles running and inserting templates into custom elements
