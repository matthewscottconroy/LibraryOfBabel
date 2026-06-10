# Pigeonhole Principle: Proof and Applications

## Theorem (Basic Form)
If n+1 or more pigeons are placed in n pigeonholes, at least one pigeonhole contains
at least 2 pigeons.

## Proof (by contradiction)
Suppose each of the n pigeonholes contains at most 1 pigeon.
Then the total number of pigeons is at most n. But we assumed n+1 pigeons — contradiction. □

## Generalized Pigeonhole Principle
If m objects are placed into n boxes, at least one box contains ⌈m/n⌉ objects.

**Proof**: If every box had at most ⌈m/n⌉ - 1 objects, the total would be at most
n(⌈m/n⌉ - 1) < n · (m/n) = m — contradiction. □

## Applications

**1. Birthday paradox (≥23 people → ≥50% chance shared birthday)**:
With 367 people, two must share a birthday (366 days + Feb 29).

**2. Five of 13 cards share a suit**:
Divide a 13-card hand by 4 suits: ⌈13/4⌉ = 4 cards in one suit minimum.

**3. Among any 6 integers, two have the same remainder mod 5**:
5 possible remainders (0,1,2,3,4); 6 integers → by pigeonhole, two share a remainder.

**4. Ramsey theory**: Any 2-coloring of the edges of K₆ contains a monochromatic triangle.
(This is Ramsey number R(3,3) = 6, proved via pigeonhole.)

**5. Data compression**: No lossless compression algorithm can compress all inputs.
(|{0,1}ⁿ| > |{0,1}^{n-1}|, so some pair maps to the same shorter string.)
