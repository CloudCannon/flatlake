# Changelog

<!-- 
    Add changes to the Unreleased section during development.
    Do not change this header — the GitHub action that releases
    this project will edit this file and add the version header for you.
    The Unreleased block will also be used for the GitHub release notes.
-->

## Unreleased

* BREAKING: Changed collection configuration format to nest `path` within `inputs`
  * Allows multiple inputs to merge into one collection
  * Add ability to put each input under a sub key
  * Allows fixed metadata to be attacked to each input

## v0.2.1 (May 2, 2024)

* Flatlake now looks into symlinks when finding content

## v0.2.0 (November 3, 2023)

* A new output of `flat_data` can now be set, which writes front matter keys at the root object rather than within `data`.
* A new setting `outputs` has been added globally and per collection, allowing you to enable/disable single, list, and aggregate endpoints individually.

## v0.1.2 (October 12, 2023)

* Single endpoints are output relative to their containing collection path
* Default globs to `**/*.{md}`

## v0.1.1 (October 12, 2023)

* Resolve binary npm packages correctly

## v0.1.0 (October 12, 2023)

* Base release — configuration surface and output syntax considered unstable prior to a `1.0` release.
