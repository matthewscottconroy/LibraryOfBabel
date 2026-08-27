# Your First Program

Create a file called `Hello.java`. The name matters, and I will explain why in a
moment.

```java
public class Hello {
    public static void main(String[] args) {
        System.out.println("Hello, world.");
    }
}
```

Run it:

```
$ java Hello.java
Hello, world.
```

Or, the two-step way:

```
$ javac Hello.java
$ java Hello
Hello, world.
```

That is a working program. Now let us take it apart, honestly.

## Line by line

**`public class Hello {`**

Java organizes all code into **classes**. A class is a named container for code
and data; Unit V is about what they are really for, and until then you can read
`class Hello` as "here begins the thing called Hello".

The filename must match: a public class named `Hello` must live in `Hello.java`.
This is unusual — most languages do not care — and it exists so that the compiler
and the JVM can find a class by name without searching every file. It is a
convention with teeth: get it wrong and `javac` refuses.

`public` means "visible from anywhere". *Deferred to Chapter 19*, where access
control is the point rather than an obstacle.

**`public static void main(String[] args) {`**

This is the line I promised not to fully explain, so here is the honest
accounting.

`main` is the name the JVM looks for when you run a class. It is a convention,
fixed by the specification: run a class and the JVM calls its `main` method. If
it is absent or spelled differently, you get an error at run time rather than
compile time, because the compiler had no reason to think you meant this class to
be startable.

`void` means this method returns no value. *Chapter 11.*

`static` means the method belongs to the class itself rather than to an object
made from it. This is necessary here because when the JVM starts, no objects
exist yet — something has to be callable before anything has been created.
*Chapter 19.*

`String[] args` is the command-line arguments, as an array of strings. *Arrays
are Chapter 15.* You can pass them:

```
$ java Hello.java Alice Bob
```

and `args` would hold `"Alice"` and `"Bob"`. Our program ignores them.

**`System.out.println("Hello, world.");`**

`System.out` is the standard output stream — the console, unless redirected.
`println` prints its argument and then a newline; `print` omits the newline.

The text in double quotes is a **string literal**. Chapter 4 tells you what is
actually in it: a sequence of characters, stored by the JVM as UTF-16 code units.

The semicolon ends the statement. Java uses them to mark where statements end
rather than relying on line breaks, which is why you can split a long statement
across lines freely.

**The braces**

`{` and `}` group statements into blocks. Every block opened must be closed. Most
of the confusing compiler errors you meet in your first weeks are unbalanced
braces, and the reported line number is often nowhere near the actual mistake —
because the compiler only notices something is wrong when it reaches the end of
the file and finds a brace missing.

Indent consistently and this problem mostly disappears. The indentation means
nothing to the compiler and everything to you.

## The debts

Four things were deferred: `public`, `static`, `void`, and `String[]`. Each is
recorded above with the chapter that pays it off.

I want to name why I am doing it this way. There is a real temptation, when
teaching, to explain everything at the first opportunity, because leaving
something unexplained feels like a failure of nerve. But an explanation you lack
the context to absorb is not an explanation — it is a sequence of words you will
have to learn again later, and the second time you will believe you already know
it, which is worse.

So the debts are real and they will be paid. In the meantime, `public static void
main(String[] args)` is the incantation that makes a class runnable, and it is
completely fine to type it without full understanding for the next ten chapters.

## When it goes wrong

Three errors you will meet, and what they mean.

```
error: class Hello is public, should be declared in a file named Hello.java
```

The filename does not match the class name.

```
error: ';' expected
```

A missing semicolon. The reported line is usually the line *after* the one that
needs it, since the compiler reads on until something does not fit.

```
Error: Could not find or load main class Hello
```

This one is a run-time error, from `java` rather than `javac`. Either you have
not compiled yet, or you are in the wrong directory, or you typed the file name
with `.java` on the end when running the two-step way.

Notice that the first two are compile-time and the third is run-time. You now
know why that distinction exists, and it will keep being useful.

Next: making the program show us something worth seeing.
