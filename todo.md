# Add std
> This will be compiled through a feature flag
> You should enable this through the config
- Navbar (Customizable + auto added)
    - Can customize this by plugin override or toml file
    - Toml file: pass through list of (end) paths
- Footer (Customizable + auto added) [Somewhat done](The autoadding needs to be there)
    - simmilar to navbar
- Blog List
    - pass through a path, dirs = blog series (DONE)
- Blog series

# Move premade assest (such as css) [2/2 Done](The framework is there)

# Make header links things [Done]
like the thing that shows a # next to the header, then click it and it takes you to a link
## Allow links to headers [Done]

# Allow to change the title without changing the name of the file
useful for index

+ rethink how to do stuff like this
possibly for fm:
title/name
filename

this would allow for speficifation of filename vs title/name
title/name should fall back on filename
