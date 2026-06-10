# Hoare Logic Problems

## Section 1: Verification by Hoare Logic (★★)

For each program, find a loop invariant and verify the Hoare triple.

**1.** Integer multiplication via repeated addition:
```
{x ≥ 0 ∧ y ≥ 0}
result := 0;
count := 0;
while count < x do
  result := result + y;
  count := count + 1
{result = x * y}
```
Find the loop invariant and verify each step.

**2.** Integer exponentiation:
```
{b ≥ 0}
result := 1;
exp := b;
while exp > 0 do
  result := result * base;
  exp := exp - 1
{result = base^b}
```
(Pre-condition should also include `base ≥ 0` or handle negatives.)

## Section 2: Finding Bugs via Hoare Logic (★★)

**3.** The following program intends to find the maximum of array A[0..n-1]:
```
{n > 0}
max := A[0];
i := 1;
while i < n do
  if A[i] > max then max := A[i];
  i := i + 1
{max = max element of A}
```
  a. What is the loop invariant?
  b. Is the program correct? If not, find the bug.

## Section 3: Coq Verification (★★★)

**4.** Implement and verify in Coq (or Lean 4) a function that computes the GCD of two
natural numbers using the Euclidean algorithm, proving:
  - The function terminates
  - The result divides both inputs
  - The result is the greatest such divisor

**5.** State (and if possible prove) a Hoare triple for a binary search function over
a sorted array.

## Section 4: Real-World Applications (★)

**6.** Research one of the following formally verified systems and write a 1-2 paragraph
summary of what was verified, what tools were used, and what the significance was:
  - CompCert (verified C compiler)
  - seL4 (verified OS microkernel)
  - CryptoVerif (verified cryptographic protocols)
  - Fiat Cryptography (verified elliptic curve implementations used in Chrome/Firefox)
