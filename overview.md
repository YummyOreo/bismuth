# Steps:

- Get a list of all the markdown files and their paths [Done]
- Parse markdown files and update path based on info in markdown
    - This will be stored in a File struct with a Markdown struct and a Metadata struct
    - For this you can use [insta.rs](https://insta.rs/) for testing the parsing
- Make html files using the tree
    - The html file template is basted no the File struct `type` field (or in Metadat)
- Make Rss based on the markdown files

# Support:
## Langs:
- Rust (compile the rust down to wasm)
- Python ([pystript](https://pyscript.net/))
- Js/Ts
- [Codepen](https://codepen.io/)
- [Jsfiddle](https://jsfiddle.net/)
- [Codesandbox](https://codesandbox.io/)
- [Replit](https://replit.com/)
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
