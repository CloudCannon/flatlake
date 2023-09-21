Feature: Base Tests

    Scenario: Default message
        When I run my program
        Then I should see "flatlake running" in stdout
