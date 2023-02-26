# Steps:

- Get a list of all the markdown files and their paths [Done]
- Parse markdown files and update path based on info in markdown
    - This will be stored in a File struct with a Markdown struct and a Metadata struct [Done]
    - For this you can use [insta.rs](https://insta.rs/) for testing the parsing [Done]
    - For "reactive" files (ie. lists of blogs), they will be parsed. Just put a place holder for the reactive list 0Done
    - Then re-parse the list ones [Done]
    - ALSO: TEST ALL THE LEXER WITH ALL TEH MD FILESS [N/A]
- Make html files using the tree
    - The html file template is basted no the File struct `type` field (or in Metadat)
- Make Rss based on the markdown files

## Rendering:
- Code highlighting will be based on current theme

## implementation details:
For internal/default templates, use:
`include_str!`

Lazyload all images

# Built In Customs:
## Header/Navbar:
Like the footer:
- Customizable for each file
- Can make a "default" one
### Sections:
- page name: link (will auto gray out if it is the current link)
- name
## Footer:
TODO

# Support:
## Langs:
- Wasm
- Rust (compile the rust down to wasm)
- Python ([pystript](https://pyscript.net/))
- Js/Ts
- [Codepen](https://codepen.io/)
- [Jsfiddle](https://jsfiddle.net/)
- [Codesandbox](https://codesandbox.io/)
- [Replit](https://replit.com/)
### Tests snippets in langs:
- Allows you to define a code base (like the whole code) at multiple states
- Then you can take snippets of those
- It will test if it compiles
## Page types:
- Blog
- Main
- Blogs
- Other
## Other:
- series
- syntax highlighting for *most* languages

# Next:
- cashing using serd and [bincode](https://crates.io/crates/bincode)
- project file for info on site
- init project using charm stuff and go (tui)

- ~~widgets/components support~~
