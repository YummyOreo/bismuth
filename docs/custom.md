# How custom elements will work:
First:
    - It will be able to pass through attrs and body
    - Will call a `post_parse` to the plugin w/ the corrosponding name
        - Will have access to all the other files parsed
Then:
    - The `post_parse` will be able to add custom elements to the original custom element
        - These elements will be rendered at `{{elements}}` in the template

## This allows something like this:

Markdown:
```
%{{
    name: list
    dir: /test/
}}
```

Parsed:
```
Element {
    Kind: Custom({
        name: "list",
        values: {
            dir: "/test/"
        }
    })
    ...
}
```

Post Parse:

```
Element {
    Kind: Custom({
        name: "list",
        values: {
            dir: "/test/"
        },
    })
    elemens [
        Element {
            Kind: Custom({
                name: "list-item",
                values: {
                    name: "test",
                }
            }),
            ...
        }
    ]
    ...
}
```

Render:
```html
<div>
    <ul>

        <li>
            <h3> test </h3>
        </li>
    </ul>
</div>
```
