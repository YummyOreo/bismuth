# Lexer:
The main thing that this does is tokenize a markdown file.

## Tokens:
A token can be more than one character. All tokens (with the exception of EOL tokens) are combined when needed.
- `Text`: All characters that do not fall in the other tokens
- `EndOfFile`: The end of the file
- `EndOfLine`: The end of a line, this will never be combined with previous EOL tokens. Characters are always `\n`. *But in the snapshots/tests, these are replaced with '↲' because it is easyer to see. If yout just see a question mark in a box, then you should install a [nerdfont](https://www.nerdfonts.com/font-downloads)
- `StartOfFile`: The start of the file.
- `DollarSign`: Any dollar sign in the file. This is done for LaTeX
- `Whitespace`: This is for any whitespace that is at the start of the file. This is done for people that use spaces instead of tabs. See the `Lexer::handle_whitespace` function for more detials about this
- `Tab`: Any tabs in the program
- `Asterisk`: Any `*` in the program
- `Underscore`: Any `_` in the program
- `Hash`: Any hashes in the program that have a space after it (somewhat). See the `Lexer::handle_hash` function for more detials
- `BracketLeft` + `BracketRight:` Any `[` or `]` in the program
- `ParenthesisLeft` + `ParenthesisRight:` Any `(` or `)` in the program
- `Exclamation`: Same imlp as `Hash`, but with exclamation marks
- `GreaterThan`: Any greaterthan that is at the start of a line and has a space after it. See the `Lexer::handle_greaterthan` function for more detials
- `Dash`: Could be: fontmatter, 3 dashes, or just text. This is really complicated because of the fontmatter so see [dash.md in the docs](docs/dash.md) for info on this
- `ListNumber`: Any number that has a `.` after it. See the `Lexer::handle_number` function for mare detials
- `Backtick`: Any ` char in the program
- `CurlybraceLeft` + `CurlybraceRight:` Any `{` or `}` in the program
- `Percent`: Any `%` char in the program
- `FontmatterStart` + `FontmatterInside` + `FontmatterEnd`: [See here for info](docs/dash.md)
