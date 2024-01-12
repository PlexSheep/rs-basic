Feature: inverted square root feature

  Scenario: If we calculate the inverted square root of a number using fast inverted square root, it's about the same as if we calculate it normally
    Given a number
    When We calculate the the inverted square root of a number using fast inverted square root
    Then The result is about the same as if we calculate it normally
