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
  - output_key: "posts"
    inputs:
      - path: "collections/posts"
  - output_key: "people"
    inputs:
      - path: "collections/authors"
```

## Options

Each collection requires an `output_key`, and at least one `inputs` entry containing a `path`.

### Input > Path

A path from the global `source` to add to this collection.

Files within this collection input will be treated relative to this path.  
In the below example, a file at `<source>/collections/posts/post-a.md` will be output at `<dest>/posts/post-a.json`.

{{< diffcode >}}
```yml
collections:
  - output_key: "posts"
    inputs:
+      - path: "collections/posts"
```
{{< /diffcode >}}

### Input > Glob

The glob expression Flatlake should use when finding files within this collection. Defaults to `**/*.{md}` to select all markdown files in any directory.

{{< diffcode >}}
```yml
collections:
  - output_key: "posts"
    inputs:
      - path: "collections/posts"
+        glob: "**/*.{md}"
```
{{< /diffcode >}}

### Input > Meta

Fixed metadata that should be added to each collection item sourced from this input.

{{< diffcode >}}
```yml
collections:
  - output_key: "posts"
    inputs:
      - path: "collections/posts"
+        meta:
+           source: "collections"
+           url: https://example.com
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

### Outputs

For within this collection, what endpoints should be created.

Available endpoints are:

| Endpoint    | Description                                                                                |
|-------------|--------------------------------------------------------------------------------------------|
| `single`    | Creates an individual JSON endpoint for every file                                         |
| `list`      | Creates paginated endpoints at `[collection]/all/page-[number].json`                       |
| `aggregate` | Creates aggregation endpoints at `[collection]/aggregate/[key]/[value]/page-[number].json` |

All endpoints are enabled by default. Specifying a list here will limit the output to those specified.

{{< diffcode >}}
```yml
collections:
  - path: "collections/posts"
    output_key: "posts"
+    outputs:
+      - "single"
+      - "list"
```
{{< /diffcode >}}

### Single elements

For each file's individual JSON endpoint, what Flatlake should include in the file.

Available elements are:

| Output element | Description                                                                                                                                                                       |
|----------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `data`         | For data files, represents the entire file. For files with front matter (e.g. `.md`), represents the front matter. Output inside the key `data`.                                  |
| `flat_data`    | The same as `data` above, but all keys are output at the root of the file rather than inside a `data` object.                                                                     |
| `content`      | The raw text content after any front matter. Output inside the key `content`.                                                                                                     |
| `content_ast`  | The content after any front matter, parsed as markdown and saved as a structured AST. Useful for rendering this content in non-web contexts. Output inside the key `content_ast`. |

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

Elements are processed in-order, which might affect which keys are chosen when using `flat_data` if your front matter contains keys named `content` or `data`. Elements specified later in the list will override keys from earlier elements in this case.

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
