Feature: Pagination Tests

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
        Given I have a "content/animals/dolphin.md" file with the content:
            """
            ---
            _schema: animal
            uuid: def
            published_date: 2023-09-04T00:00:00+0000
            info:
              title: Dolphin
            tags:
              - mammal
              - carnivore
            ---
            # Content about dolphins
            """
        Given I have a "content/animals/beaver.md" file with the content:
            """
            ---
            _schema: animal
            uuid: def
            published_date: 2023-09-05T00:00:00+0000
            info:
              title: Beaver
            tags:
              - mammal
              - herbivore
            ---
            # Content about beavers
            """
        Given I have a "content/animals/aardvark.md" file with the content:
            """
            ---
            _schema: animal
            uuid: def
            published_date: 2023-09-06T00:00:00+0000
            info:
              title: Aardvark
            tags:
              - mammal
              - carnivore
            ---
            # Content about aardvarks
            """
        Given I have a "content/animals/platypus.md" file with the content:
            """
            ---
            _schema: animal
            uuid: def
            published_date: 2023-09-07T00:00:00+0000
            info:
              title: Platypus
            tags:
              - mammal
              - carnivore
            ---
            # Content about platypuses
            """
        Given I have a "content/animals/sheep.md" file with the content:
            """
            ---
            _schema: animal
            uuid: def
            published_date: 2023-09-08T00:00:00+0000
            info:
              title: Sheep
            tags:
              - mammal
              - herbivore
            ---
            # Content about sheep
            """

    Scenario: Default to large pages
        Given I have a "flatlake.yaml" file with the content:
            """
            collections:
              - output_key: "animals"
                path: "animals"
                glob: "**/*.{md}"
                sort_key: published_date
                sort_direction: desc
            """
        When I run my program
        Then I should see "flatlake running" in stdout
        Then I should see "api/animals/aggregate/tags/mammal/page-1.json" containing the values:
            | page                     | int:1              |
            | total_pages              | int:1              |
            | has_more                 | bool:false         |
            | values.0.url             | animals/sheep.json |
            | values.0.data.info.title | Sheep              |
            | values.6.url             | animals/cat.json   |
            | values.6.data.info.title | Cat                |

    Scenario: Paginate per collection setting
        Given I have a "flatlake.yaml" file with the content:
            """
            collections:
              - output_key: "animals"
                path: "animals"
                glob: "**/*.{md}"
                sort_key: published_date
                sort_direction: desc
                page_size: 2
            """
        When I run my program
        Then I should see "flatlake running" in stdout
        Then I should see "api/animals/aggregate/tags/mammal/page-1.json" containing the values:
            | page                     | int:1                                     |
            | total_pages              | int:4                                     |
            | has_more                 | bool:true                                 |
            | next_page                | animals/aggregate/tags/mammal/page-2.json |
            | values.0.url             | animals/sheep.json                        |
            | values.0.data.info.title | Sheep                                     |
            | values.1.url             | animals/platypus.json                     |
            | values.1.data.info.title | Platypus                                  |
        Then I should see "api/animals/aggregate/tags/mammal/page-4.json" containing the values:
            | page                     | int:4            |
            | total_pages              | int:4            |
            | has_more                 | bool:false       |
            | values.0.url             | animals/cat.json |
            | values.0.data.info.title | Cat              |
        Then I should see "api/animals/aggregate/tags/herbivore/page-1.json" containing the values:
            | page                     | int:1                                        |
            | total_pages              | int:2                                        |
            | has_more                 | bool:true                                    |
            | next_page                | animals/aggregate/tags/herbivore/page-2.json |
            | values.0.url             | animals/sheep.json                           |
            | values.0.data.info.title | Sheep                                        |
            | values.1.url             | animals/beaver.json                          |
            | values.1.data.info.title | Beaver                                       |
        Then I should see "api/animals/aggregate/tags/herbivore/page-2.json" containing the values:
            | page                     | int:2               |
            | total_pages              | int:2               |
            | has_more                 | bool:false          |
            | values.0.url             | animals/iguana.json |
            | values.0.data.info.title | Iguana              |

    Scenario: Paginate global setting
        Given I have a "flatlake.yaml" file with the content:
            """
            global:
              page_size: 2
            collections:
              - output_key: "animals"
                path: "animals"
                glob: "**/*.{md}"
                sort_key: published_date
                sort_direction: desc
            """
        When I run my program
        Then I should see "flatlake running" in stdout
        Then I should see "api/animals/aggregate/tags/mammal/page-1.json" containing the values:
            | page                     | int:1                                     |
            | total_pages              | int:4                                     |
            | has_more                 | bool:true                                 |
            | next_page                | animals/aggregate/tags/mammal/page-2.json |
            | values.0.url             | animals/sheep.json                        |
            | values.0.data.info.title | Sheep                                     |
            | values.1.url             | animals/platypus.json                     |
            | values.1.data.info.title | Platypus                                  |
        Then I should see "api/animals/aggregate/tags/mammal/page-4.json" containing the values:
            | page                     | int:4            |
            | total_pages              | int:4            |
            | has_more                 | bool:false       |
            | values.0.url             | animals/cat.json |
            | values.0.data.info.title | Cat              |

    Scenario: Page size of zero means all
        Given I have a "flatlake.yaml" file with the content:
            """
            global:
              page_size: 2 # set a small global page size that we can override
            collections:
              - output_key: "animals"
                path: "animals"
                glob: "**/*.{md}"
                sort_key: published_date
                sort_direction: desc
                page_size: 0 # override this collection to put all items on one page
            """
        When I run my program
        Then I should see "flatlake running" in stdout
        Then I should see "api/animals/aggregate/tags/mammal/page-1.json" containing the values:
            | page                     | int:1              |
            | total_pages              | int:1              |
            | has_more                 | bool:false         |
            | values.0.url             | animals/sheep.json |
            | values.0.data.info.title | Sheep              |
            | values.6.url             | animals/cat.json   |
            | values.6.data.info.title | Cat                |
