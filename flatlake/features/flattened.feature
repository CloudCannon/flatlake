Feature: Flattened Data Tests

    Background:
        Given I have the environment variables:
            | FLATLAKE_SOURCE  | content |
            | FLATLAKE_DEST    | api     |
            | FLATLAKE_VERBOSE | true    |
        Given I have a "content/animals/cat.md" file with the content:
            """
            ---
            _schema: animal
            uuid: abc
            content: Front matter content
            published_date: 2023-09-01T00:00:00+0000
            info:
              title: Cat
            tags:
              - mammal
              - carnivore
            ---
            # Content about cats
            """

    Scenario: Output flat data in endpoints
        Given I have a "flatlake.yaml" file with the content:
            """
            collections:
              - output_key: "animals"
                inputs:
                  - path: "animals"
                    glob: "**/*.{md}"
                sort_key: published_date
                sort_direction: desc
                single_elements: [ "flat_data", "content" ]
            """
        When I run my program
        Then I should see "flatlake running" in stdout
        Then I should see "api/animals/cat.json" containing the values:
            | info.title | Cat                    |
            | content    | # Content about cats\n |

    Scenario: Element ordering controls precendence
        Given I have a "flatlake.yaml" file with the content:
            """
            collections:
              - output_key: "animals"
                inputs:
                  - path: "animals"
                    glob: "**/*.{md}"
                sort_key: published_date
                sort_direction: desc
                single_elements: [ "content", "flat_data" ]
            """
        When I run my program
        Then I should see "flatlake running" in stdout
        Then I should see "api/animals/cat.json" containing the values:
            | info.title | Cat                  |
            | content    | Front matter content |
