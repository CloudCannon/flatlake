Feature: Aggregation Tests

    Background:
        Given I have the environment variables:
            | FLATLAKE_SOURCE  | content |
            | FLATLAKE_DEST    | api     |
            | FLATLAKE_VERBOSE | true    |
        Given I have a "content/animals/cat.md" file with the content:
            """
            ---
            _schema: animal
            warm: true
            uuid: abc
            published_date: 2023-09-01T00:00:00+0000
            info:
              title: Cat
            tags:
              - mammal
              - carnivore
            ---
            """
        Given I have a "content/animals/dog.md" file with the content:
            """
            ---
            _schema: animal
            warm: true
            uuid: def
            published_date: 2023-09-02T00:00:00+0000
            info:
              title: Dog
            tags:
              - mammal
              - carnivore
            ---
            """
        Given I have a "content/animals/iguana.md" file with the content:
            """
            ---
            _schema: animal
            warm: false
            uuid: ghi
            published_date: 2023-09-03T00:00:00+0000
            info:
              title: Iguana
            tags:
              - reptile
              - herbivore
            ---
            """

    Scenario: Aggregate files are enabled by default
        Given I have a "flatlake.yaml" file with the content:
            """
            collections:
              - output_key: "animals"
                inputs:
                  - path: "animals"
                    glob: "**/*.{md}"
                sort_key: published_date
                sort_direction: desc
            """
        When I run my program
        Then I should see "flatlake running" in stdout
        Then I should see the file "api/animals/aggregate/tags/mammal/page-1.json"

    Scenario: Aggregate files can be disabled
        Given I have a "flatlake.yaml" file with the content:
            """
            collections:
              - output_key: "animals"
                inputs:
                  - path: "animals"
                    glob: "**/*.{md}"
                sort_key: published_date
                sort_direction: desc
                outputs: [ "single" ]
            """
        When I run my program
        Then I should see "flatlake running" in stdout
        Then I should not see the file "api/animals/aggregate/tags/mammal/page-1.json"

    Scenario: Booleans generate aggregate files
        Given I have a "flatlake.yaml" file with the content:
            """
            collections:
              - output_key: "animals"
                inputs:
                  - path: "animals"
                    glob: "**/*.{md}"
                sort_key: published_date
                sort_direction: desc
            """
        When I run my program
        Then I should see "flatlake running" in stdout
        Then I should see the file "api/animals/aggregate/warm/true/page-1.json"
        Then I should see the file "api/animals/aggregate/warm/false/page-1.json"
