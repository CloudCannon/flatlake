Feature: Base Tests

    Background:
        Given I have the environment variables:
            | FLATLAKE_SOURCE  | content |
            | FLATLAKE_DEST    | api     |
            | FLATLAKE_VERBOSE | true    |
        Given I have a "flatlake.yaml" file with the body:
            """
            collections:
              - output_key: "animals"
                path: "content/animals"
                glob: "**/*.{md}"
                sort:
                  - published_date
            """
        Given I have a "content/animals/cat.md" file with the body:
            """
            ---
            _schema: animal
            uuid: abc
            published_date: 2023-09-01T00:00:00+0000
            info:
              title: Cat
            tags:
              - mammal
              - carnivore
            ---
            """
        Given I have a "content/animals/dog.md" file with the body:
            """
            ---
            _schema: animal
            uuid: def
            published_date: 2023-09-02T00:00:00+0000
            info:
              title: Dog
            tags:
              - mammal
              - carnivore
            ---
            """
        Given I have a "content/animals/iguana.md" file with the body:
            """
            ---
            _schema: animal
            uuid: ghi
            published_date: 2023-09-03T00:00:00+0000
            info:
              title: Iguana
            tags:
              - reptile
              - herbivore
            ---
            """

    Scenario: Tests are functional
        When I run my program
        Then I should see "flatlake running" in stdout

    Scenario: Output direct files as JSON
        When I run my program
        Then I should see "flatlake running" in stdout
        Then I should see "api/animals/cat.json" containing the values:
            | uuid           | abc                      |
            | published_date | 2023-09-01T00:00:00+0000 |
            | info.title     | Cat                      |
            | tags.0         | mammal                   |
            | tags.1         | carnivore                |

    Scenario: Output aggregate files from front matter
        When I run my program
        Then I should see "flatlake running" in stdout
        Then I should see "api/aggregate/tags/mammal/01.json" containing the values:
            | 0.uuid       | def |
            | 0.info.title | Dog |
            | 1.uuid       | abc |
            | 1.info.title | Cat |
        Then I should see "flatlake running" in stdout
        Then I should see "api/aggregate/tags/herbivore/01.json" containing the values:
            | 0.uuid       | ghi    |
            | 0.info.title | Iguana |
