---
title: "Getting Started with Flatlake"
nav_title: "Quick start"
nav_section: Root
weight: 1
---

Flatlake runs as a standalone tool, reading folders of content or data files and generating a static API to query your data.

To start, you'll need a repository with content files. For example, the folder structure:

{{< tree >}}
collections/
>> posts/
>  >> post_a.md
>  >> post_b.md
>  >> post_c.md
>> authors/
   >> jane.md
   >> john.md
{{< /tree >}}

Where the contents of `collections/posts/post_a.md` might look like:

```md
---
title: Post A
tags:
  - article
  - tech
date: 2023-10-12
---

Curabitur **blandit** tempus porttitor. Vestibulum id ligula porta felis euismod semper. Maecenas faucibus mollis interdum.

1. Donec id elit non mi porta gravida at eget metus.
```

And the contents of `collections/authors/jane.md` might look like:

```md
---
name: Jane
tags:
  - tech
---
```

## Configuring Flatlake

Flatlake needs to know what your content collections are, and how to process them. Flatlake allows you to specify your own collection structures, which makes it easy to map Flatlake over existing source files from any static site generator.

To start, create a Flatlake configuration file at the root of your repository. Flatlake supports configuration via `flatlake.yml`, `flatlake.toml`, or `flatlake.json` files — these docs will show usage with YAML.

A simple configuration to start with for the above folder structure is:

`flatlake.yml`:
```yml
collections:
  - output_key: "posts"
    path: "collections/posts"
  - output_key: "people"
    path: "collections/authors"
```

This denotes our `posts` and `authors` folders as collections, each output under the same name in our final API.  

By default, Flatlake processes all files ending with `.md` in any directory within the collection.

## Running Flatlake

The easiest way to run flatlake is through npx. If you don't have Node and npm installed, or want to install Flatlake another way, see the [Installing Flatlake](/docs/installation/) guide.

In the root of your repository, run Flatlake:

```bash
npx -y flatlake
```

After completion, you should now find a new directory. By default, this directory is named `api`.

Flatlake endeavors to support most use cases out of the box, so this directory contains many ways to query your data. Depending on the contents of each file, the full output directory for our simple example above might look like:

{{< tree >}}
api/
>> posts/
>  >> post_a.json
>  >> post_b.json
>  >> post_c.json
>  >> all/
>  >  >> page-1.json           # An endpoint listing all posts
>  >> aggregate/
>     >> tags/
>        >> article/
>        >  >> page-1.json     # An endpoint listing all posts with a tag of article
>        >> tech/
>           >> page-1.json
>> authors/
>  >> jane.json
>  >> john.json
>  >> all/
>  >  >> page-1.json
>  >> aggregate/
>     >> tags/
>        >> tech/
>           >> page-1.json
>> all/
>  >> page-1.json
>> aggregate/
   >> tags/
      >> article/
      >  >> page-1.json
      >> tech/
         >> page-1.json        # An endpoint listing content within all collections with a tag of tech
{{< /tree >}}

Looking through this structure, we'll find a few notable endpoints.

### Single endpoints

For each input file, we have a matching single endpoint for its content. For example, the `collections/posts/post_a.md` file now has a JSON representation available at `api/posts/post_a.json`. Looking inside this file, we see:

```json
{
  "content": "\nCurabitur **blandit** tempus porttitor. Vestibulum id ligula porta felis euismod semper. Maecenas faucibus mollis interdum.\n\n1. Donec id elit non mi porta gravida at eget metus.\n",
  "data": {
    "date": "2023-10-12",
    "tags": [ "article", "tech" ],
    "title": "Post A"
  }
}
```

### Aggregate endpoints

Additionally, we have a range of aggregate files within each collection, and across all of our data. These aggregate files are created for all data keys automatically, allowing you to query for any single facet of your data out of the box. Looking inside `api/posts/aggregate/tags/tech/page-1.md` we might see:

```json
{
  "page": 1,
  "total_pages": 1,
  "has_more": false,
  "next_page": null,
  "values": [
    {
      "data": {
        "date": "2023-10-12",
        "tags": [ "article", "tech" ],
        "title": "Post A"
      },
      "url": "posts/post_a.json"
    },
    {
      "data": {
        "date": "2023-10-10",
        "tags": [ "tech" ],
        "title": "Post B"
      },
      "url": "posts/post_b.json"
    }
  ]
}
```

These are paginated lists of your content aggregated by attributes of their front matter. By default, the content is omitted in this view and a link to the single endpoint is provided.

### List endpoints

We also have listing files — e.g. `api/posts/all/page-1.md` — containing the same structure as the aggregate files, but listing all items in that collection.

## Next steps

To further configure the setup of your Flatlake site, see the [Setting up collections](/docs/collections/) page.