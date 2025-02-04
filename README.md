CLI functionality is simple. It reads a config that specifies a list of files and nodes. The user can then apply or unapply patches.

Currently, it only supports modifying nodes in Yaml files. The spec & criteria sheet for these modifications is straightforward:
- Only REPLACE the VALUE of the specified node
- Don't add missing nodes to the file
- It is not allowed to "reformat" the file at all
- Respect previous whitespace/indentation
- Respect comments in applicable config formats (Including header, footer and trailing comments)
- Must be able to traverse & modify complex data structures in supported config formats, such as tables
- Editing a file should never break the syntax of the config
