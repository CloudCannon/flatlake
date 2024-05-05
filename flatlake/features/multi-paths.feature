Feature: Multi-Path Tests

  Background:
    Given I have the environment variables:
      | FLATLAKE_SOURCE  | content |
      | FLATLAKE_DEST    | api     |
      | FLATLAKE_VERBOSE | true    |
    Given I have a "content/source-a/animals/cat.md" file with the content:
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
    Given I have a "content/source-a/animals/dog.md" file with the content:
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
    Given I have a "content/source-b/animals/iguana.md" file with the content:
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

  Scenario: Multiple sources can aggregate together
    Given I have a "flatlake.yaml" file with the content:
      """
            collections:
              - output_key: "animals"
                inputs:
                  - path: "source-a/animals"
                    glob: "**/*.{md}"
                  - path: "source-b/animals"
                    glob: "**/*.{md}"
                sort_key: published_date
                sort_direction: desc
      """
    When I run my program
    Then I should see "flatlake running" in stdout
    Then I should see "api/animals/cat.json" containing the values:
      | data.uuid | abc |
    Then I should see "api/animals/iguana.json" containing the values:
      | data.uuid | ghi |

  Scenario: Multiple sources can aggregate with sub-keys
    Given I have a "flatlake.yaml" file with the content:
      """
            collections:
              - output_key: "animals"
                inputs:
                  - path: "source-a/animals"
                    sub_key: "a"
                    glob: "**/*.{md}"
                  - path: "source-b/animals"
                    sub_key: "b"
                    glob: "**/*.{md}"
                sort_key: published_date
                sort_direction: desc
      """
    When I run my program
    Then I should see "flatlake running" in stdout
    Then I should see "api/animals/a/cat.json" containing the values:
      | data.uuid | abc |
    Then I should see "api/animals/b/iguana.json" containing the values:
      | data.uuid | ghi |

  Scenario: Multiple sources can merge unique keys
    Given I have a "flatlake.yaml" file with the content:
      """
            collections:
              - output_key: "animals"
                inputs:
                  - path: "source-a/animals"
                    sub_key: "a"
                    glob: "**/*.{md}"
                    merge_data:
                      label: left
                  - path: "source-b/animals"
                    sub_key: "b"
                    glob: "**/*.{md}"
                    merge_data:
                      label: right
                sort_key: published_date
                sort_direction: desc
      """
    When I run my program
    Then I should see "flatlake running" in stdout
    Then I should see "api/animals/a/cat.json" containing the values:
      | data.uuid  | abc  |
      | data.label | left |
    Then I should see "api/animals/b/iguana.json" containing the values:
      | data.uuid  | ghi   |
      | data.label | right |


