# Pseudocode Assignment in Rust

## Student Details

- Name: Your Name
- Subject: Pseudocode
- Professor: Your Professor Name
- Semester: Your Semester

---

## Assignment Overview

| No. | Topic                      | Pseudocode                            | Rust Code                               |
| --- | -------------------------- | ------------------------------------- | --------------------------------------- |
| 1   | Add Digits of a Number     | [View](#1-add-digits-of-a-number)     | [Open](./src/solutions/add_digits.rs)   |
| 2   | Search a Digit in a Number | [View](#2-search-a-digit-in-a-number) | [Open](./src/solutions/search_digit.rs) |

---

## 1. Add Digits of a Number

### Question

Write an algorithm and pseudocode to add all digits of a number.

### Pseudocode

```text
START
INPUT number
sum ← 0

WHILE number > 0 DO
    digit ← number MOD 10
    sum ← sum + digit
    number ← number DIV 10
END WHILE

PRINT sum
STOP
```
