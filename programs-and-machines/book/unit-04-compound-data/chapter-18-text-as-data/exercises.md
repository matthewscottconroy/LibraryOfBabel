# Exercises

Exercises marked **[carries forward]** introduce something a later chapter
assumes.

## Immutability

**18.1.** Predict and explain:
```java
String s = "hello";
s.toUpperCase();
System.out.println(s);
```

**18.2.** Name the four benefits of `String` immutability. For each, give a
concrete situation where it saves you something.

**18.3. [carries forward]** Predict and explain all three:
```java
String a = "hi", b = "hi", c = new String("hi");
System.out.println(a == b);
System.out.println(a == c);
System.out.println(a.equals(c));
```
Then say why this bug survives testing.

**18.4.** Why can a `String` be a safe `HashMap` key when a mutable object cannot?

## Building

**18.5.** Explain why `result += "a"` in a loop is quadratic. Give the arithmetic
for 40,000 iterations.

**18.6. [carries forward]** Rewrite using `StringBuilder`, and say what the cost
becomes:
```java
String csv = "";
for (String field : fields) csv += field + ",";
```

**18.7.** The compiler turns `"a" + b + "c"` into a `StringBuilder` automatically.
Why can it not do the same for a `+=` inside a loop?

**18.8.** When is `StringBuilder` *not* worth using? Give two cases.

**18.9.** Rewrite 18.6 again using `String.join`, and say which of the three
versions you prefer.

## Comparing

**18.10.** For each pair, say what `equals`, `equalsIgnoreCase`, and `compareTo`
give: `("abc","abc")`, `("ABC","abc")`, `("apple","banana")`, `("Zebra","apple")`.

**18.11.** Explain why `"Zebra".compareTo("apple")` is negative, and why that
surprises users.

**18.12. [carries forward]** Two strings display as `café` and are not equal.
Explain, and give the fix.

**18.13.** Explain the Turkish-I problem. What is the defensive habit, and when
should you *not* apply it?

**18.14.** When would you use `Collator` rather than `compareTo`?

## Parsing

**18.15.** Predict each:
```java
"a.b.c".split(".").length
"a,b,,".split(",").length
"a,b,,".split(",", -1).length
"a  b\tc".split("\\s+").length
```

**18.16.** Write code that parses `Ada,Lovelace,1815` into a name and an `int`
year, handling a malformed year without crashing.

**18.17.** Why does this print an empty line, and how do you fix it?
```java
int n = in.nextInt();
String name = in.nextLine();
```

**18.18.** `Ada,"Lovelace, Countess",1815` breaks a comma split. Describe what a
correct CSV parser must do, and say what you would actually use.

## Patterns and formatting

**18.19.** Write patterns for: a UK postcode-like shape of letters and digits; a
time as `HH:MM`; a word of at least three letters.

**18.20. [carries forward]** Explain, using Chapter 6, why no regular expression
can match balanced brackets.

**18.21.** Rewrite with `printf`, aligning the columns:
```java
System.out.println(name + " scored " + score + " (" + pct + "%)");
```

**18.22.** `String.format("%,.2f", 1234.5)` gives different results on different
machines. Explain, and say when each is correct.

**18.23.** Why is `2024-01-15` the right format for a data file and `15/01/2024`
the wrong one?

## Going further

**18.24.** Write a method that reads lines of `key=value`, tolerating spaces,
blank lines, and lines beginning with `#`. State its contract, then test it with at
least six boundary cases.

**18.25.** Section 18.2.2 says a correct email pattern is over four hundred
characters. Find out why, and say what you would do instead in a real program.

**18.26.** This chapter claims `String` is a worked example of the whole unit.
Write a paragraph justifying that: name the array, the abstract data type, the
representation invariant, and the growth-by-doubling, and say where each appears.
