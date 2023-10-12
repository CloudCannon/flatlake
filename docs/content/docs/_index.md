---
title: "Getting Started with Flatlake"
nav_title: "Quick Start"
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

> Donec id elit non mi porta gravida at eget metus.
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

Flatlake needs to know what your logical collections are, and how to process them. To do so, create a Flatlake configuration file at the root of your repository. Flatlake support configuration via `flatlake.yml`, `flatlake.toml`, or `flatlake.json` — these docs will show usage with YAML.

The most simple configuration to start with for the above folder structure is:

`flatlake.yml`:
```yml
collections:
  - output_key: "posts"
    path: "collections/posts"
  - output_key: "people"
    path: "collections/authors"
```

This denotes our `posts` and `authors` folders as collections, each output under the same name in our final API.  
For each folder, we specify that Flatlake should process all markdown files ending in `.md`.

## Running Flatlake

The easiest way to run flatlake is through npx. If you don't have Node and npm installed, or want to install Flatlake another way, see the [Installing Flatlake](/docs/installation/) guide.

In the root of your repository, run Flatlake:

```bash
npx -y flatlake
```

After completion, you should now find a new directory exists. By default, this directory is named `api`.

Flatlake endeavors to support most use cases out of the box, so this directory contains many qays to query your data. Depending on the contents of each file, the full output directory for our simple file example above might look like:

{{< tree >}}
api/
>> posts/
>  >> post_a.json
>  >> post_b.json
>  >> post_c.json
>  >> aggregate/
>     >> tags/
>     >  >> article/
>     >  >  >> page-1.json
>     >  >> tech/
>     >     >> page-1.json
>     >> date/
>        >> 2023-10-12/
>        >  >> page-1.json
>        >> 2023-10-13/
>           >> page-1.json
>> authors/
>  >> jane.json
>  >> john.json
>  >> aggregate/
>     >> tags/
>        >> tech/
>           >> page-1.json
>> aggregate/
   >> tags/
      >> article/
      >  >> page-1.json
      >> tech/
         >> page-1.json
{{< /tree >}}

Looking through this structure, we'll find a few notable endpoints.

For each input file, we have a matching single endpoint for its content. For example, the `collections/posts/post_a.md` file now has a JSON representation available at `api/posts/post_a.json`. Looking inside this file, we see:

```json
{
  "content": "\nCurabitur **blandit** tempus porttitor. Vestibulum id ligula porta felis euismod semper. Maecenas faucibus mollis interdum.\n\n> Donec id elit non mi porta gravida at eget metus.\n",
  "data": {
    "date": "2023-10-12",
    "tags": [ "article", "tech" ],
    "title": "Post A"
  }
}
```

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