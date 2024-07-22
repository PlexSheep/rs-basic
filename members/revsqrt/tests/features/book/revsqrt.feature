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

  Scenario: Calculate fast inverted sqrt with specific numbers
    Given the number n
      | n            |
      | 1            |
      | 1.1          |
      | 100          |
      | 1337         |
      | 123.45678900 |
      | 1337.1337    |
    When we calculate the inverted square root of it using the fast inverted square root algorithm
    Then the result is about the same as m
      | m                    |
      | 1                    |
      | 0.9534625892455922   |
      | 0.1                  |
      | 0.02734854943722097  |
      | 0.0900000004095      |
      | 0.027347182112297627 |

  Scenario: Calculate regular inverted sqrt with specific numbers
    Given the number n
      | n            |
      | 1            |
      | 1.1          |
      | 100          |
      | 1337         |
      | 123.45678900 |
      | 1337.1337    |
    When we calculate the inverted square root of it normally
    Then the result is m
      | m                    |
      | 1                    |
      | 0.9534625892455922   |
      | 0.1                  |
      | 0.02734854943722097  |
      | 0.0900000004095      |
      | 0.027347182112297627 |

  Scenario: Some numbers are about the same (0)
    Given the number n
      | n            |
      | 1            |
      | 0.9          |
      | 1.0001       |
      | 1.1001       |
      | 1.1050       |
      | 0.999        |
      | 0.9999999999 |
    Then they are about the same

  Scenario: Some numbers are about the same (1)
    Given the number n
      | n            |
      | 10           |
      | 10.0001      |
      | 9.997        |
      | 10.025       |
    Then they are about the same

  Scenario: Some numbers are about the same (-3)
    Given the number n
      | n            |
      | -1000        |
      | -1000.1      |
      | -1001.1      |
    Then they are about the same

  Scenario: Some numbers are about the same (3)
    Given the number n
      | n            |
      | -1000        |
      | -1000.1      |
      | -1001.1      |
    Then they are about the same

  Scenario: Some numbers are about the same (7)
    Given the number n
      | n               |
      | 10000000        |
      | 10000000        |
      | 10000300        |
      | 10000000.1      |
      | 10000001.1      |
    Then they are about the same

  Scenario: Some numbers are not about the same (1)
    Given the number n
      | n                  |
      | 2                  |
      | -2                 |
      | 0                  |
      | 20                 |
      | 20000              |
    Then they are not about the same

  Scenario: Some numbers are not about the same (7)
    Given the number n
      | n               |
      | 10000000        |
      | 10001000        |
      | 0               |
      | 20000001.1      |
    Then they are not about the same

  Scenario: Test if we can use the asterisk
    Given a number
    * a number
    * a number
    * a number
    * a number
    When we calculate the inverted square root of it normally
    Then the result can be calculated
