Feature: Base Tests

    Scenario: Default message
        When I run my program
        Then I should see "There is nothing to do" in stdout
