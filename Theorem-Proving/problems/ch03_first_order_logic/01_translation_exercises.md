# First-Order Logic: Translation Exercises

## Section 1: English to FOL (★)

Use the vocabulary:
- `Person(x)`: x is a person
- `Student(x)`: x is a student
- `Prof(x)`: x is a professor
- `Loves(x,y)`: x loves y
- `Teaches(x,y)`: x teaches y
- `Smart(x)`: x is smart
- Domain: people

**1.** Translate to FOL:
  a. Every professor teaches some student.
  b. Some student loves every professor.
  c. No professor loves themselves.
  d. If you love someone, they love you back.
  e. There is exactly one person who is both a student and a professor.
  f. Everyone either is a student or teaches a student.

## Section 2: FOL to English (★)

**2.** Translate each FOL formula to natural English:
  a. `∀x(Student(x) → ∃y(Prof(y) ∧ Teaches(y,x)))`
  b. `∃x(Prof(x) ∧ ∀y(Student(y) → ¬Loves(x,y)))`
  c. `∀x∀y((Loves(x,y) ∧ Smart(y)) → Smart(x))`
  d. `¬∃x(Person(x) ∧ Loves(x,x))`

## Section 3: Tarski's World (★★)

In a world with blocks a, b, c, d (use: Cube, Tet, Dodec, Small, Medium, Large, LeftOf):

**3.** Write FOL sentences that are true in exactly the following cases:
  a. There is a small cube to the left of a large tetrahedron
  b. Every cube is smaller than every tetrahedron
  c. No two blocks have the same size

**4.** Build a Tarski's World (on paper or in software) where:
  - `∀x∀y(Cube(x) ∧ Cube(y) → SameSize(x,y))` is true
  - `∃x∃y(Cube(x) ∧ Tet(y) ∧ LeftOf(x,y))` is true
  - `∀x(Large(x) → Cube(x))` is false

## Section 4: Common Pitfalls (★★)

**5.** What is wrong with each translation?
  a. "Some student studies every subject." →  `∀x(Subject(x) → ∃y(Student(y) ∧ Studies(y,x)))`
  b. "Everyone has a unique best friend." → `∃x∀y BestFriend(y,x)`

**6.** Translate each sentence in two ways, making the scope ambiguity explicit:
  a. "A guard checked every visitor."
  b. "Every student wants to marry a professor."
