# Exercises

Exercises marked **[carries forward]** introduce something a later chapter
assumes.

## References and aliasing

**20.1.** Predict and explain:
```java
int[] p = {1,2,3};
int[] q = p;
q[0] = 99;
System.out.println(p[0]);
```

**20.2.** Name the four ways an alias is created. For each, give a line of code.

**20.3. [carries forward]** A method returns the internal `List<Item>` of an
`Order`. Write the caller code that empties the order without appearing to touch
it. Then fix the method three different ways.

**20.4.** When is aliasing exactly what you want? Give a concrete example and say
what would break if each reference were a separate copy.

**20.5.** A constructor stores the `List<Item>` it was given. What can the caller
still do? Write the defensive version.

## Copying

**20.6.** Explain the difference between a reference copy, a shallow copy and a
deep copy, using `List<List<Integer>>`.

**20.7.** Predict:
```java
List<List<Integer>> outer = new ArrayList<>();
outer.add(new ArrayList<>(List.of(1,2)));
List<List<Integer>> shallow = new ArrayList<>(outer);
shallow.get(0).add(3);
System.out.println(outer.get(0));
```

**20.8.** Why is every copying facility in Java shallow? Give two reasons deep
copying is not well defined in general.

**20.9. [carries forward]** Write a copy constructor for a class holding a
`String`, an `int`, and a `List<String>`. Which of the three needs copying, and
why not the others?

**20.10.** Give two reasons to prefer a copy constructor over `clone()`.

## Equality

**20.11.** Why is `a.equals(b)` false for two `Point` objects with the same
coordinates, when `Point` defines no `equals`?

**20.12.** Write `equals` for a `Money` class with `cents` and `currency`. Include
the identity check, the type check, and the field comparison.

**20.13.** Why does `public boolean equals(Point o)` fail to override anything?
What would `@Override` have done?

**20.14. [carries forward]** For each, say whether you would define value equality
and which fields you would use: `Point`; `Account`; `Thread`; `LocalDate`;
`Order`.

**20.15.** Explain why `"admin".equals(name)` is safer than `name.equals("admin")`,
and give a cleaner alternative.

## hashCode

**20.16.** A class defines `equals` and not `hashCode`. What is the size of a
`HashSet` after adding two equal objects, and why?

**20.17.** State the two rules relating `equals` and `hashCode`. Which direction
of implication holds, and which does not?

**20.18.** `return 0;` is a legal `hashCode`. Why is it a bad one?

**20.19. [carries forward]** Predict and explain all three lines:
```java
List<Integer> key = new ArrayList<>(List.of(1,2));
Map<List<Integer>,String> m = new HashMap<>();
m.put(key, "v");
System.out.println(m.get(key));
key.add(3);
System.out.println(m.get(key));
System.out.println(m.size());
```
Can the entry ever be retrieved or removed afterwards?

## Immutability

**20.20.** List the five rules for an immutable class. Which one do people most
often miss, and what goes wrong?

**20.21.** Explain why `private final List<String> items` does not make a class
immutable.

**20.22.** Name four things immutability buys. Which will not be visible to you
until Chapter 31?

**20.23.** `String` is immutable and `StringBuilder` exists. Explain the pattern,
and name another pair in the library that follows it.

**20.24.** Bloch says classes should be immutable unless there is a very good
reason otherwise. Argue for that position, then against it. Where does an
`Account` fall, and why?

## Going further

**20.25.** Take the `Account` from Chapter 19 and write an immutable version where
`deposit` returns a new `Account`. What becomes easier? What becomes awkward? Which
would you ship?

**20.26.** Section 20.2.3 distinguishes entities from values. Classify ten types
from a program you have written, and say whether each should be immutable.

**20.27.** Construct a bug caused by unintended aliasing where the stack trace at
the point of damage names no code responsible for it. Then say which of Section
20.1.2's defenses would have prevented it.
