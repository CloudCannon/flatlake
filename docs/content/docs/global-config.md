---
title: "Setting global Flatlake configuration"
nav_title: "Global configuration"
nav_section: References
weight: 45
---

Flatlake supports configuration via any of:
- `flatlake.yml`
- `flatlake.yaml`
- `flatlake.toml`
- `flatlake.json`

Additionally, some options can be provided as CLI flags or environment variables.

## Top-level options

### Source
The location of your source files.

| CLI Flag          | ENV Variable      | Config Key |
|-------------------|-------------------|------------|
| `--source <PATH>` | `FLATLAKE_SOURCE` | `source`   |

### Destination
The location Flatlake should write your output files. Defaults to `api`.

| CLI Flag        | ENV Variable    | Config Key |
|-----------------|-----------------|------------|
| `--dest <PATH>` | `FLATLAKE_DEST` | `dest`     |

### Verbose
Print verbose logging while generating files. Does not affect the contents of the output files.

| CLI Flag    | ENV Variable       | Config Key |
|-------------|--------------------|------------|
| `--verbose` | `FLATLAKE_VERBOSE` | `verbose`  |

### Logfile
Writes logs to the given logfile, in addition to the console. Replaces the file on each run.

| CLI Flag           | ENV Variable       | Config Key |
|--------------------|--------------------|------------|
| `--logfile <PATH>` | `FLATLAKE_LOGFILE` | `logfile`  |

## Global defaults

### Page size
Configure the default page size for all paginated endpoints.  
Defaults to `100`. A value of `0` will output all items on the first page.

| Config Key         |
|--------------------|
| `global.page_size` |

```yml
global:
  page_size: 50
```

### Sort key
Configure the default front matter key which is used to sort items.

| Config Key        |
|-------------------|
| `global.sort_key` |

```yml
global:
  sort_key: date
```

### Sort direction
Configure which direction to sort items in. Must be `asc` or `desc`.

| Config Key              |
|-------------------------|
| `global.sort_direction` |

```yml
global:
  sort_direction: desc
```

## Collection configuration

The `collections` key in your config will contain a list of collections for Flatlake to process.  
See [Setting up collections](/docs/collections-config/) for all options.

| Config Key    |
|---------------|
| `collections` |
