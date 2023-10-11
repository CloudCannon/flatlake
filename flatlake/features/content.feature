Feature: Content Tests

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
            published_date: 2023-09-01T00:00:00+0000
            info:
              title: Cat
            tags:
              - mammal
              - carnivore
            ---
            # Content about cats
            """
        Given I have a "content/animals/dog.md" file with the content:
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
            # Content about dogs
            """
        Given I have a "content/animals/iguana.md" file with the content:
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

    Scenario: Output full content in list endpoints
        Given I have a "flatlake.yaml" file with the content:
            """
            collections:
              - output_key: "animals"
                path: "animals"
                glob: "**/*.{md}"
                sort_key: published_date
                sort_direction: desc
                list_elements: [ "data", "content" ]
            """
        When I run my program
        Then I should see "flatlake running" in stdout
        Then I should see "api/animals/aggregate/tags/mammal/page-1.json" containing the values:
            | values.0.data.uuid       | def                    |
            | values.0.data.info.title | Dog                    |
            | values.0.content         | # Content about dogs\n |
            | values.1.data.uuid       | abc                    |
            | values.1.data.info.title | Cat                    |
            | values.1.content         | # Content about cats\n |

    Scenario: Output ASTs in single endpoints
        Given I have a "flatlake.yaml" file with the content:
            """
            collections:
              - output_key: "animals"
                path: "animals"
                glob: "**/*.{md}"
                sort_key: published_date
                sort_direction: desc
                single_elements: [ "content_ast" ]
            """
        When I run my program
        Then I should see "flatlake running" in stdout
        Then I should see "api/animals/cat.json" containing the values:
            | content_ast.type                        | root               |
            | content_ast.children.0.type             | heading            |
            | content_ast.children.0.children.0.value | Content about cats |
