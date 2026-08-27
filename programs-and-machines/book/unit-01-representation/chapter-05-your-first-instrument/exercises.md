# Exercises

Exercises marked **[carries forward]** introduce something a later chapter
assumes.

Every exercise here involves running something. That is the point of the chapter.

## Getting it running

**5.1.** Type `Hello.java` from Section 5.1.3 and run it. Then break it four
ways, one at a time, and record the exact error message: remove a semicolon,
remove a closing brace, rename the file to `hello.java`, misspell `main` as
`Main`. For each, say whether the error came from `javac` or from `java`.

**5.2.** Change the program to print your name on one line and today's date on
the next, using two `println` calls. Then do it with one call.

**5.3.** Predict what each of these prints, then check:
```java
System.out.println("total: " + 3 + 4);
System.out.println("total: " + (3 + 4));
System.out.println(3 + 4 + " is the total");
```
Explain the third one in terms of left-to-right evaluation.

## Using the instrument

**5.4. [carries forward]** Type `Instrument.java` from Section 5.2.3 and run it.
Confirm that the output matches the book.

**5.5.** Add lines to show the bit patterns of 1, 2, 4, 8, and 16. What is the
visual pattern, and what does it tell you about multiplication by 2?

**5.6.** Show the patterns for −1, −2, and −128. Then apply flip-and-add-one to
the pattern for −128 by hand and explain the result using Section 2.2.2.

**5.7. [carries forward]** Extend the instrument with a method that shows a
`long` in the same format, and use it to display `Long.MAX_VALUE` and
`Long.MAX_VALUE + 1`.

**5.8.** Display the bits of `0.5`, `0.25`, and `0.75` as doubles. All three are
exactly representable. What do their fraction fields look like, and why?

**5.9.** Display the bits of `1.0`, `2.0`, and `4.0`. Only the exponent changes.
Read off each exponent field and confirm it is 1023, 1024, 1025.

## Predicting

For each of these, **write down your prediction first**, then run it. The
prediction is the exercise; running it is only the grading.

**5.10.**
```java
System.out.println(7 / 2);
System.out.println(7 / 2.0);
System.out.println(7 % 2);
System.out.println(-7 / 2);
System.out.println(-7 % 2);
```

**5.11.**
```java
byte b = 127;
b++;
System.out.println(b);
```

**5.12.**
```java
System.out.println((int) 3.99);
System.out.println((int) -3.99);
System.out.println((byte) 300);
System.out.println((char) 66);
```

**5.13.**
```java
System.out.println(1 / 0);
System.out.println(1.0 / 0);
```
One of these fails. Which, and why does the other not?

## Going further

**5.14.** Write a program that prints, for each of `byte`, `short`, `int`, and
`long`, its minimum and maximum using the constants `Byte.MIN_VALUE` and so on.
Check every number against the range you would derive from Chapter 2.

**5.15.** `2147483648` will not compile as an `int` literal. Find the exact error
message, explain it in terms of Chapter 2, and then make it compile.

**5.16. [carries forward]** Write a program that prints the code point and the
UTF-8 byte count of a character you supply. Test it with `A`, `é`, and the emoji `"\uD83D\uDE00"`.
Explain any result that surprises you using Chapter 4. (You will need
`String.codePointAt` and `getBytes("UTF-8")`; looking up unfamiliar methods is a
skill this book expects you to start building now.)

**5.17.** The JVM compiles bytecode to machine code only after a method has run
many times. Design an experiment that would let you observe this — you do not
need to run it, but say what you would measure and what pattern would confirm
the effect.

**5.18.** Section 5.1.3 deferred four things: `public`, `static`, `void`, and
`String[]`. Write down, in one sentence each, what you currently believe each
one does. Keep the page. You are asked to revisit it at the end of Chapter 19,
and comparing the two answers is the most useful thing this exercise does.
