---
title: "Setting up Flatlake collections"
nav_title: "Setting up collections"
nav_section: References
weight: 42
---

Collections are how files are grouped in Flatlake. Each collection specifies:
- A set of source files to read,
- An API location to make the data available at,
- Settings for how to sort, aggregate, and paginate the generated data.

## Example

`flatlake.yml`:
```yml
collections:
  - path: "collections/posts"
    output_key: "posts"
  - path: "collections/authors"
    output_key: "people"
```

## Options

Each collection requires a `path` and an `output_key` to be defined.

### Path

The path from the global `source` to this collection.

Files within this collection will be treated relative to this path.  
In the below example, a file at `<source>/collections/posts/post-a.md` will be output at `<dest>/posts/post-a.json`.

{{< diffcode >}}
```yml
collections:
+  - path: "collections/posts"
    output_key: "posts"
```
{{< /diffcode >}}

### Output key

The folder to use when writing single and list files for this collection.

In the below example, a file at `<source>/collections/posts/post-a.md` will be output at `<dest>/files/post-a.json`,
and list files for this collection will be available at `<dest>/files/all/page-1.json`.

{{< diffcode >}}
```yml
collections:
  - path: "collections/posts"
+    output_key: "files"
```
{{< /diffcode >}}

### Glob

The glob expression Flatlake should use when finding files within this collection. Defaults to `**/*.{md}` to select all markdown files in any directory.

{{< diffcode >}}
```yml
collections:
  - path: "collections/posts"
    output_key: "posts"
+    glob: "**/*.{md}"
```
{{< /diffcode >}}


### Page size

The page size to use when writing paginated endpoints within this collection.
Defaults to the global page size, or `100` if that is not set.

A value of `0` will output all items on the first page.

{{< diffcode >}}
```yml
collections:
  - path: "collections/posts"
    output_key: "posts"
+    page_size: 20
```
{{< /diffcode >}}

### Sort key

The front matter value that should be used when sorting list endpoints in this collection.

{{< diffcode >}}
```yml
collections:
  - path: "collections/posts"
    output_key: "posts"
+    sort_key: "date"
```
{{< /diffcode >}}

### Sort direction

The direction to sort items in. Must be `asc` or `desc`.

{{< diffcode >}}
```yml
collections:
  - path: "collections/posts"
    output_key: "posts"
+    sort_direction: "desc"
```
{{< /diffcode >}}

### Single elements

For each file's individual JSON endpoint, what Flatlake should include in the file.

Available elements are:

| Output element | Description                                                                                                                                  |
|----------------|----------------------------------------------------------------------------------------------------------------------------------------------|
| `data`         | For data files, represents the entire file. For files with front matter (e.g. `.md`), represents the front matter.                           |
| `content`      | The raw text content after any front matter.                                                                                                 |
| `content_ast`  | The content after any front matter, parsed as markdown and saved as a structured AST. Useful for rendering this content in non-web contexts. |

Single endpoints output `data` and `content` by default.

{{< diffcode >}}
```yml
collections:
  - path: "collections/posts"
    output_key: "posts"
+    single_elements:
+      - "data"
+      - "content"
```
{{< /diffcode >}}


### List elements

For lists and aggregations within this collection, what Flatlake should include alongside each item in the list.

Available elements are the same as those described in [Single elements](#single-elements) section above.

List endpoints only output `data` by default.

{{< diffcode >}}
```yml
collections:
  - path: "collections/posts"
    output_key: "posts"
+    list_elements:
+      - "data"
+      - "content"
```
{{< /diffcode >}}
