Feature: Output Tests

    Background:
        Given I have the environment variables:
            | FLATLAKE_VERBOSE | true |

    Scenario: Output paths that differ from source paths
        Given I have a "flatlake.yaml" file with the content:
            """
            collections:
              - output_key: "creatures"
                path: "collections/animals"
                glob: "**/*.{md}"
            """
        Given I have a "collections/animals/cat.md" file with the content:
            """
            ---
            name: Cat
            ---
            """
        When I run my program
        Then I should see "flatlake running" in stdout
        Then I should see "api/creatures/cat.json" containing the values:
            | data.name | Cat |
