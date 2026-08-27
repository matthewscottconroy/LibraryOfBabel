# An Instrument for the Rest of the Book

Time to stop taking my word for things.

Everything Unit I claimed about what is in memory, we can now display. This
lesson builds a small program that does it, and then we point it at the four
chapters behind us.

## The program

Create `Instrument.java`:

```java
public class Instrument {

    public static void main(String[] args) {
        show("214", 214);
        show("-5", -5);
        show("Integer.MAX_VALUE", Integer.MAX_VALUE);
        show("Integer.MAX_VALUE + 1", Integer.MAX_VALUE + 1);

        System.out.println();
        System.out.println("0.1 as bits : " + bits64(Double.doubleToLongBits(0.1)));
        System.out.println("0.1 as hex  : " + Double.toHexString(0.1));
        System.out.println("'A' as bits : " + bits32('A'));
    }

    static void show(String label, int value) {
        System.out.printf("%-22s %12d  %s  0x%08X%n",
                          label, value, bits32(value), value);
    }

    static String bits32(int v) {
        String s = Integer.toBinaryString(v);
        s = "0".repeat(32 - s.length()) + s;
        return group(s);
    }

    static String bits64(long v) {
        String s = Long.toBinaryString(v);
        s = "0".repeat(64 - s.length()) + s;
        return group(s);
    }

    static String group(String s) {
        StringBuilder out = new StringBuilder();
        for (int i = 0; i < s.length(); i++) {
            if (i > 0 && i % 8 == 0) out.append(' ');
            out.append(s.charAt(i));
        }
        return out.toString();
    }
}
```

There are things in there you have not been taught — `static` methods, a loop, a
`StringBuilder`, an `if`. Do not worry about them. Type it, run it, and treat the
helper methods as a black box for now; Chapters 9, 11, and 18 will explain each
one, and you will be able to come back and read this program completely.

Run it:

```
$ java Instrument.java
214                             214  00000000 00000000 00000000 11010110  0x000000D6
-5                               -5  11111111 11111111 11111111 11111011  0xFFFFFFFB
Integer.MAX_VALUE        2147483647  01111111 11111111 11111111 11111111  0x7FFFFFFF
Integer.MAX_VALUE + 1   -2147483648  10000000 00000000 00000000 00000000  0x80000000

0.1 as bits : 00111111 10111001 10011001 10011001 10011001 10011001 10011001 10011010
0.1 as hex  : 0x1.999999999999ap-4
'A' as bits : 00000000 00000000 00000000 01000001
```

Now look at what it just told you.

## Reading the output

**214 is `11010110`.** The exact pattern we converted by hand in Chapter 2, with
24 leading zeros because an `int` is 32 bits wide. And `0xD6` in hexadecimal —
`D` is `1101`, `6` is `0110` — which is the regrouping from Section 2.1.3, done
by the machine.

**−5 is `11111111 11111111 11111111 11111011`.** Chapter 2 said the 8-bit pattern
for −5 was `11111011`, and here are those same eight bits at the bottom with 1s
filling everything above. Read as unsigned it would be 4,294,967,291, which is
$2^{32}$ − 5 — the wrap-around position on the circle, exactly as promised. And the
hexadecimal `0xFFFFFFFB` shows the same thing more compactly.

Try the flip-and-add-one recipe on it by hand and you will get back
`00000000 00000000 00000000 00000101`, which is 5.

**`Integer.MAX_VALUE` is a 0 followed by thirty-one 1s.** Add one and every column
carries, the leading 0 becomes 1, and you get `10000000 …` — the most negative
`int`. There it is on the fourth line, with no exception raised and no warning
printed. Chapter 2's central example, confirmed.

**0.1's bits.** Split them the way Chapter 3 described: one sign bit `0`, eleven
exponent bits `01111111011` which is 1019, and 52 fraction bits. 1019 − 1023 is
−4. And look at the fraction: `1001 1001 1001 …` repeating, then ending `1010`
instead of `1001` — the repeating binary expansion of one tenth, rounded up at
the last bit. The hexadecimal form `0x1.999999999999ap-4` says the same thing
even more clearly: a mantissa of nines, a final `a` where rounding pushed it up,
times $2^{-4}$.

**`'A'` is 65.** Chapter 4's ASCII value, and the pattern `01000001` — which is
the very first pattern in this book, from Section 1.2.1, where it was five
different things at once. Here it is being one of them, because we asked for one
particular reading.

## What you have

You now have a way to answer, for yourself, the question this unit has been
asking: *what is actually in there?*

That matters more than it may seem. For the rest of this book — and for the rest
of your time programming — you will meet situations where a value is not what you
expected. The instinct to guess is strong and mostly unproductive. The habit
worth building instead is to look.

Add to this program as you go. Give it a `long`. Give it a `float` and compare
the layout to a `double`. Feed it a character above 127 and watch what `char`
does. Feed it the result of a calculation you do not trust.

## Closing Unit I

Five chapters ago we started with a wire holding 3.2 volts, and the claim that
the wire does not hold a number.

Since then we have built, in order: the bit as an agreement about voltage;
positional notation as an agreement about bits; two's complement as an agreement
that makes subtraction free and overflow inevitable; floating point as a bargain
buying range with precision; Unicode as an agreement about characters layered on
an agreement about integers; and finally a language whose types are exactly the
names of these agreements.

Every one of them is a convention. Every one has edges. Every one is silent at
those edges.

You can now read the bottom of the stack, which almost no beginning programmer
can do, and which will keep paying off for years. When something is inexplicable
— a total off by a penny, a name that displays as `Ã©`, a positive number that
went negative — you have somewhere to stand.

Unit II asks the same question about process. We have settled what a machine can
*hold*. Now: what does it mean for a machine to take a step?
