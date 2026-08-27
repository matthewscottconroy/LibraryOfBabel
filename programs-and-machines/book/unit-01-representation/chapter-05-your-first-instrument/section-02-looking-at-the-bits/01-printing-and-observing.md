# Printing and Observing

A program that computes without reporting is not much use to a learner. Before we
can look at bits, we need to be able to look at anything.

## The three printing methods

```java
System.out.print("no newline");
System.out.println("with a newline");
System.out.printf("formatted %d and %s%n", 42, "text");
```

`print` and `println` differ only in the trailing newline. `printf` is the
interesting one.

## Formatted output

`printf` takes a **format string** containing placeholders, followed by the values
to substitute:

```java
System.out.printf("%-22s %12d%n", "Integer.MAX_VALUE", 2147483647);
```

produces

```
Integer.MAX_VALUE         2147483647
```

Reading the placeholders:

| Placeholder | Meaning |
|---|---|
| `%d` | a whole number |
| `%s` | a string, or anything convertible to one |
| `%f` | a floating-point number |
| `%x` | hexadecimal, lowercase; `%X` uppercase |
| `%n` | a newline |
| `%%` | a literal percent sign |

Between the `%` and the letter you can put a width, and a `-` to left-align:

```
%12d    right-align in 12 columns
%-22s   left-align in 22 columns
%08X    hex, 8 digits, zero-padded
```

That last one will be doing a lot of work shortly, because zero-padded
eight-digit hex is exactly the shape of a 32-bit value.

Use `%n` rather than `\n` for newlines in `printf`. `\n` is always the single
character 10; `%n` is whatever the current platform considers a line separator,
which on Windows is two characters. It rarely matters and costs nothing to get
right.

## String concatenation

The `+` operator joins strings:

```java
System.out.println("value is " + 214);
```

When one operand is a string, Java converts the other to a string and joins them.
This is convenient and has one trap worth meeting now:

```java
System.out.println("sum: " + 1 + 2);      // prints  sum: 12
System.out.println("sum: " + (1 + 2));    // prints  sum: 3
```

`+` evaluates left to right. In the first line, `"sum: " + 1` produces the string
`"sum: 1"`, and then `+ 2` appends `"2"`. The parentheses in the second line force
the addition to happen first.

If you have ever printed a total and got two numbers stuck together, this is why.

## Escape sequences

Some characters cannot be typed directly inside a string literal:

| Escape | Character |
|---|---|
| `\n` | newline (10) |
| `\t` | tab (9) |
| `\"` | double quote |
| `\\` | backslash |
| `\u0041` | the character with code point U+0041 (`A`) |

That last one is Chapter 4 showing through: `"\u00e9"` is `é`, and you can
write any character by its code point when your keyboard or your file encoding
will not cooperate.

## Reading input

Occasionally you will want the program to ask:

```java
import java.util.Scanner;

public class Ask {
    public static void main(String[] args) {
        Scanner in = new Scanner(System.in);
        System.out.print("Enter a number: ");
        int n = in.nextInt();
        System.out.println("You typed " + n);
    }
}
```

Three things there are unexplained — `import`, `new`, and the fact that `in` is an
object with methods. All three are Unit V, and `Scanner` will be treated properly
in Chapter 29 when we deal with input generally. For now it is a recipe you can
copy when you want interactive programs.

I would encourage you not to use it much yet. Programs that hard-code their
inputs are easier to re-run, easier to compare against, and easier to reason
about while you are still learning what the machine does. Interactive input is a
convenience that mostly slows down experimentation.

## Observing is the point

One habit, and it matters more than any syntax in this lesson.

When you do not understand what a program is doing, **print something**. Print
the value you think is wrong. Print it before and after the line you suspect.
Print the thing you are certain about, to check that you are certain correctly.

Later chapters will introduce a debugger, which is better in most ways. But
printing never stops being useful, it works in situations where a debugger cannot
reach, and — most importantly for now — the discipline of *predicting what will
print and then checking* is the single fastest way to build an accurate model of
what the machine is doing.

That is what the instrument in two lessons' time is for.
