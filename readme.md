# Bismuth

## This is still in beta and is not anywhere near done

## Install
To install this run the following commands:
```bash
cd ./bismuth-core/
cargo install --path .
```

## Contributing:
Look at [overview](overview.md) and [architecture](docs/architecture.md)

## Blogs:
```
%{{
    name: blog list
    dir: /path/to/blogs/
}}
```
For each blog: frontmatter examlpe
```md
---
values:
    - title: Title of the blog post
    - date: Date of the blog post
---
...
```

## Navbar:
Auto added if enabled in the config
To include something:
```
---
values:
    - navbar_include: true
    - navbar_title: Title (will use title as a fallback)
    - navbar_order: 1 (to order the navbar)
---
```
