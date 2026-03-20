# Pseudocode Assignment

![Rust](https://img.shields.io/badge/Rust-1.70%2B-blue.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

---

## Student Details

- **Name:** Prakhar Shrivastav
- **Subject:** Pseudocode
- **Professor:** Amar Nayak
- **Semester:** VI

---

## Table of Contents

1. [Swap Two Numbers (With Temp)](#1-swap-two-numbers-with-temp)
2. [Swap Two Numbers (Without Temp)](#2-swap-two-numbers-without-temp)
3. [Average of Three Numbers](#3-average-of-three-numbers)
4. [Sum of Natural Numbers](#4-sum-of-natural-numbers)
5. [Sum of Squares](#5-sum-of-squares)
6. [Factorial of a Number](#6-factorial-of-a-number)
7. [Armstrong Number](#7-armstrong-number)
8. [Palindrome Number](#8-palindrome-number)
9. [Add Digits of a Number](#9-add-digits-of-a-number)
10. [Search a Digit in a Number](#10-search-a-digit-in-a-number)
11. [Prime Number Check](#11-prime-number-check)
12. [Fibonacci Series](#12-fibonacci-series)
13. [Reverse a Number](#13-reverse-a-number)
14. [Count Occurrences of a Digit](#14-count-occurrences-of-a-digit)
15. [Max Element in an Array](#15-max-element-in-an-array)

---

## Assignment Overview

| No. | Topic                           | Section Link                              |
| --- | ------------------------------- | ----------------------------------------- |
| 1   | Swap Two Numbers (With Temp)    | [#1-swap-two-numbers-with-temp](#1-swap-two-numbers-with-temp) |
| 2   | Swap Two Numbers (Without Temp) | [#2-swap-two-numbers-without-temp](#2-swap-two-numbers-without-temp) |
| 3   | Average of Three Numbers        | [#3-average-of-three-numbers](#3-average-of-three-numbers) |
| 4   | Sum of Natural Numbers          | [#4-sum-of-natural-numbers](#4-sum-of-natural-numbers) |
| 5   | Sum of Squares                  | [#5-sum-of-squares](#5-sum-of-squares) |
| 6   | Factorial of a Number           | [#6-factorial-of-a-number](#6-factorial-of-a-number) |
| 7   | Armstrong Number                | [#7-armstrong-number](#7-armstrong-number) |
| 8   | Palindrome Number               | [#8-palindrome-number](#8-palindrome-number) |
| 9   | Add Digits of a Number          | [#9-add-digits-of-a-number](#9-add-digits-of-a-number) |
| 10  | Search a Digit in a Number      | [#10-search-a-digit-in-a-number](#10-search-a-digit-in-a-number) |
| 11  | Prime Number Check              | [#11-prime-number-check](#11-prime-number-check) |
| 12  | Fibonacci Series                | [#12-fibonacci-series](#12-fibonacci-series) |
| 13  | Reverse a Number                | [#13-reverse-a-number](#13-reverse-a-number) |
| 14  | Count Occurrences of a Digit    | [#14-count-occurrences-of-a-digit](#14-count-occurrences-of-a-digit) |
| 15  | Max Element in an Array          | [#15-max-element-in-an-array](#15-max-element-in-an-array) |

---

## 1. Swap Two Numbers (With Temp)

Write a pseudocode to swap two numbers using a temporary variable.

### Pseudocode

```text
DECLARE a := INTEGER
DECLARE b := INTEGER
DECLARE temp := INTEGER

READ a, b

temp := a
a := b
b := temp

PRINT a, b
```

---

## 2. Swap Two Numbers (Without Temp)

Write a pseudocode to swap two numbers without a temporary variable.

### Pseudocode

```text
DECLARE a := INTEGER
DECLARE b := INTEGER

READ a, b

a := a + b
b := a - b
a := a - b

PRINT a, b
```

---

## 3. Average of Three Numbers

Write pseudocode to calculate the average of three numbers.

### Pseudocode

```text
DECLARE a := INTEGER
DECLARE b := INTEGER
DECLARE c := INTEGER

READ a, b, c

DECLARE AVG := (a + b + c) / 3

PRINT AVG
```

---

## 4. Sum of Natural Numbers

Write a pseudocode to calculate the sum of the series $1 + 2 + 3 + ... + n$.

### Pseudocode

```text
DECLARE sum := INTEGER
DECLARE n := INTEGER
DECLARE i := INTEGER

SET i := 1, sum := 0
READ n

FOR i := 1 TO n LOOP
    sum := sum + i
END FOR

PRINT sum
```

---

## 5. Sum of Squares

Write a pseudocode to calculate the sum of the series $1 + 4 + 9 + ... + n$ (Sum of squares).

### Pseudocode

```text
DECLARE sum := INTEGER
DECLARE n := INTEGER
DECLARE i := INTEGER

SET i := 1, sum := 0
READ n

FOR i := 1 TO n LOOP
    sum := sum + (i * i)
END FOR

PRINT sum
```

---

## 6. Factorial of a Number

Write a pseudocode to calculate the factorial of a given number $n$.

### Pseudocode

```text
DECLARE n := INTEGER
READ n

DECLARE Fact := INTEGER
SET Fact := 1
INTEGER i

FOR i := 1 TO n LOOP
    Fact := Fact * i
END FOR

PRINT Fact
```

---

## 7. Armstrong Number

Write a pseudocode to check if a number is an Armstrong number.

### Pseudocode

```text
DECLARE n : INTEGER
DECLARE num : INTEGER
DECLARE b : INTEGER
DECLARE a : INTEGER

SET b := 0
READ num
n := num

WHILE n > 0 DO
    a := n MOD 10
    n := n / 10
    b := b + a * a * a
ENDWHILE

IF b == num THEN
    PRINT "ARMSTRONG"
ELSE
    PRINT "NOT ARMSTRONG"
END IF
```

---

## 8. Palindrome Number

Write a pseudocode to check if a number is a Palindrome.

### Pseudocode

```text
DECLARE n : INTEGER
DECLARE num : INTEGER
DECLARE b : INTEGER

SET b := 0
READ num
n := num

WHILE n > 0 DO
    a := n MOD 10
    b := (b * 10) + a
    n := n / 10
ENDWHILE

IF b == num THEN
    PRINT "PALINDROME"
ELSE
    PRINT "NOT PALINDROME"
END IF
```

---

## 9. Add Digits of a Number

Write a pseudocode to add all digits of a number.

### Pseudocode

```text
DECLARE n : INTEGER
DECLARE sum : INTEGER
DECLARE a : INTEGER
SET sum := 0
READ n

WHILE n > 0 DO
    a := n MOD 10
    n := n / 10
    sum := sum + a
ENDWHILE

PRINT sum
```

---

## 10. Search a Digit in a Number

Search a digit in a number and find if present or not.

### Pseudocode

```text
DECLARE n : INTEGER
DECLARE target : INTEGER
DECLARE found : BOOLEAN
SET found := FALSE
READ n, target

WHILE n > 0 DO
    IF n MOD 10 == target THEN
        found := TRUE
        BREAK
    END IF
    n := n / 10
ENDWHILE

IF found == TRUE THEN
    PRINT "PRESENT"
ELSE
    PRINT "NOT PRESENT"
END IF
```

---

## 11. Prime Number Check

Write a pseudocode to check if a number is Prime.

### Pseudocode

```text
DECLARE n : INTEGER
READ n

FOR i := 2 TO n - 1 LOOP
    IF n MOD i == 0 THEN
        PRINT "NOT PRIME"
        BREAK
    END IF
END FOR

IF n == i THEN
    PRINT "PRIME"
END IF
```

---

## 12. Fibonacci Series

Write a pseudocode for the Fibonacci Series.

### Pseudocode

```text
DECLARE n : INTEGER
DECLARE prev : INTEGER
DECLARE next : INTEGER
DECLARE sum : INTEGER

SET prev := 0
SET next := 1
SET sum := 0

PRINT prev
PRINT next

WHILE sum <= n DO
    sum := prev + next
    PRINT sum
    prev := next
    next := sum
ENDWHILE
```

---

## 13. Reverse a Number

Write a pseudocode to reverse a given number.

### Pseudocode

```text
DECLARE n : INTEGER
DECLARE b : INTEGER
DECLARE a : INTEGER

SET b := 0
READ n

WHILE n > 0 DO
    a := n MOD 10
    n := n / 10
    b := b * 10 + a
ENDWHILE

PRINT b
```

---

## 14. Count Occurrences of a Digit

Write a pseudocode to count how many times a specific digit appears in a number.

### Pseudocode

```text
DECLARE n : INTEGER
DECLARE a : INTEGER
DECLARE d : INTEGER
DECLARE cnt : INTEGER

SET cnt := 0
READ n
READ d

WHILE n > 0 DO
    a := n MOD 10
    IF a == d THEN
        cnt := cnt + 1
    END IF
    n := n / 10
ENDWHILE

PRINT cnt
```

---

## 15. Max Element in an Array

Write a pseudocode to find the maximum element in an array of size 5.

### Pseudocode

```text
DECLARE a: ARRAY[5] OF INTEGER
DECLARE max: INTEGER

FOR i := 0 TO 4
    READ a[i]
END FOR

SET max := a[0]

FOR i := 0 TO 4
    IF a[i] > max THEN
        max := a[i]
    END IF
END FOR

PRINT max
```

---

## Feedback

Found an error or want to improve the pseudocode? Open an issue or submit a pull request.
