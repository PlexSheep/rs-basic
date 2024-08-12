Feature: inverted square root calculation

  Scenario: fast inverted sqrt is about the same as the regular inverted sqrt
    Given a number
    When we calculate the inverted square root of it using the fast inverted square root algorithm
    Then the result is about the same as if we calculate it normally

  Scenario: Can the fast inverted sqrt be calculated?
    Given a number
    When we calculate the inverted square root of it using the fast inverted square root algorithm
    Then the result can be calculated

  Scenario: Can the regular inverted sqrt be calculated?
    Given a number
    When we calculate the inverted square root of it normally
    Then the result can be calculated

  Scenario: Calculate regular inverted sqrt with specific numbers
    Given the number n
      | 1            |
      | 1.1          |
      | 100          |
      | 1337         |
      | 123.45678900 |
      | 1337.1337    |
    When we calculate the inverted square root of it normally
    Then the result is m
      | 1                    |
      | 0.9534625892455922   |
      | 0.1                  |
      | 0.02734854943722097  |
      | 0.0900000004095      |
      | 0.027347182112297627 |
