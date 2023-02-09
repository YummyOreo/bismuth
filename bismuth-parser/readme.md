## Rendering/Converting to html:
```
loop over them:
Check the start:
    if it is something like Blockquote:
        set the current element to block quote
    else:
        set it to P
    Then go through and add text or urls or smthing else like italic
    Once you hit the linebreak, then re-set this and set the current element to none,
    Repeate
```

Have this be a trait so it can me imlped on TEMPLATE so they can be remperested as a tree of elements
