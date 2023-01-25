| Crate   | Description    |
|--------------- | --------------- |
| socrates-core | The entry point |
| socrates-error | Error handling |
| socrates-md | The main interface for markdown files |
| socrates-lexer | Tokenizing markdown files |

# Socrates-Core:
The main entry point for the app. This handles cli argument and configuration details.
# Socrates-Error:
This is the error handler for the program. It handles all errors that can be resolved/saved. This gives the user options and allows them to chose between them.
> This will be rewritten soon
# Socrates-MD:
This is the Markdown file handling. This loads the markdown files from a dir, or a single file from a path. The paths are stored relative to the config dir.
## Socrates-Lexer:
This is handling for the tokenizing a markdown file. Read more about this [here](socrates-lexer/readme.md)
