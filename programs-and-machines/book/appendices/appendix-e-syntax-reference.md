# Java Syntax Reference

Everything this book uses, in one place, with the chapter that explains it.

This is for looking things up, not for reading through. If a construct here is
unfamiliar, the chapter reference is where it is taught.

## Types

```java
byte    b = 100;              //  8 bits, -128 .. 127                  (Ch 5)
short   s = 1000;             // 16 bits, -32,768 .. 32,767
int     i = 100000;           // 32 bits, ±2.1 billion
long    l = 100000L;          // 64 bits; note the L
float   f = 3.14f;            // 32-bit IEEE 754; note the f
double  d = 3.14;             // 64-bit IEEE 754
char    c = 'A';              // 16-bit unsigned UTF-16 code unit      (Ch 4)
boolean t = true;

var     x = 42;               // inferred as int, still static         (Ch 7)
final int MAX = 100;          // cannot be reassigned
```

Integer arithmetic wraps; floating point rounds (Ch 2, Ch 3).
Integer division truncates: `7 / 2` is `3`. `%` gives the remainder.
`1 / 0` throws; `1.0 / 0` is `Infinity`.

## Conversions

```java
long  wide   = anInt;         // widening: automatic
int   narrow = (int) aLong;   // narrowing: explicit cast, may lose data
int   trunc  = (int) 3.99;    // 3 — truncates toward zero
byte  low    = (byte) 300;    // 44 — keeps the low 8 bits
```

## Operators

```
arithmetic     +  -  *  /  %                                    (Ch 5)
comparison     ==  !=  <  >  <=  >=                             (Ch 8)
logical        &&  ||  !          short-circuit
               &   |   ^          full evaluation, and bitwise
assignment     =  +=  -=  *=  /=  %=
increment      ++  --
conditional    condition ? ifTrue : ifFalse
```

`==` on objects compares references, not contents. Use `equals` (Ch 20, Ch 18).

## Control flow

```java
if (condition) { ... } else if (other) { ... } else { ... }     // Ch 8

switch (value) {                                                 // Ch 8
    case 1, 2 -> doSomething();
    default   -> doOther();
}

String s = switch (value) {          // as an expression
    case 1 -> "one";
    default -> "many";
};

while (condition) { ... }                                        // Ch 9
do { ... } while (condition);
for (int i = 0; i < n; i++) { ... }
for (String item : collection) { ... }                           // enhanced

break;        // leave the innermost loop or switch
continue;     // next iteration
return value; // leave the method
```

## Methods

```java
static int square(int n) { return n * n; }                       // Ch 11
static void print(String s) { ... }                              // no return
static int sum(int... values) { ... }                            // varargs
```

Parameters are passed **by value**, always. For an object the value copied is a
reference, so the method can modify the object and cannot replace it (Ch 12).

## Arrays

```java
int[] a = new int[5];              // five zeros                   (Ch 15)
int[] b = {3, 1, 4};
String[] s = new String[3];        // three nulls

a[0] = 7;
a.length                           // a field, not a method
int[][] grid = new int[3][4];      // arrays of arrays
grid[row].length                   // never grid[0].length

Arrays.toString(a)                 // for printing
Arrays.equals(a, b)                // contents, not identity
Arrays.sort(a)
Arrays.copyOf(a, n)
```

Indices run 0 to `length - 1`. Every access is bounds-checked.

## Collections

```java
List<String> list = new ArrayList<>();                            // Ch 17
list.add("x"); list.get(0); list.size(); list.contains("x");
list.remove(0);                    // by index
list.remove("x");                  // by value

Set<String> set = new HashSet<>();
set.add("x"); set.contains("x");

Map<String, Integer> map = new HashMap<>();
map.put("k", 1); map.get("k"); map.getOrDefault("k", 0);
map.merge("k", 1, Integer::sum);

for (Map.Entry<String, Integer> e : map.entrySet()) {
    e.getKey(); e.getValue();
}

List.of("a", "b")                  // immutable
new ArrayList<>(List.of("a"))      // mutable copy
```

Declare the variable as the interface. Never modify a collection while iterating
it with an enhanced `for`; use `removeIf`.

## Strings

```java
String s = "hello";                                               // Ch 18
s.length(); s.charAt(0); s.substring(1, 3);
s.equals(t);                       // never ==
s.equalsIgnoreCase(t); s.compareTo(t);
s.contains("ell"); s.indexOf("l"); s.startsWith("he");
s.isEmpty(); s.isBlank(); s.strip();
s.toUpperCase(Locale.ROOT);        // pass a locale for keys
s.split(",");                      // takes a regular expression
String.join("-", list);
String.format("%-10s %5d %8.2f", name, n, d);

StringBuilder sb = new StringBuilder();
sb.append(x); sb.toString();       // in loops, always
```

Strings are immutable. Every method returns a new one.

## Escapes

```
\n   newline          \t   tab            \\   backslash
\"   double quote     \'   single quote
\u0041   the character with code point U+0041, i.e. A
```

```java
String block = """
    multi-line
    text""";
```

## Errors

```java
try {                                                             // Ch 28
    risky();
} catch (SomeException e) {
    System.err.println(e.getMessage());
} finally {
    cleanup();
}

throw new IllegalArgumentException("must not be empty");
Objects.requireNonNull(value, "value must not be null");
```

## Printing and reading

```java
System.out.print(x);                                              // Ch 5
System.out.println(x);
System.out.printf("%s = %d%n", name, value);
System.err.println("to standard error");

Scanner in = new Scanner(System.in);
in.nextInt(); in.next(); in.nextLine();
```

`%s` string, `%d` integer, `%f` float, `%x` hex, `%n` newline, `%%` literal
percent. Width and alignment: `%-10s`, `%5d`, `%08.2f`, `%,d`.

## Useful library calls

```java
Math.abs, Math.max, Math.min, Math.pow, Math.sqrt, Math.round
Math.addExact, Math.multiplyExact         // throw on overflow    (Ch 2)

Integer.parseInt(s); Integer.MAX_VALUE; Integer.toBinaryString(n)
Double.parseDouble(s); Double.isNaN(d); Double.toHexString(d)
Character.isDigit(c); Character.isLetter(c)

new BigDecimal("0.10")                    // from a string, never a double (Ch 3)
```

## The shape of a program

```java
public class Name {                        // file must be Name.java
    public static void main(String[] args) {
        ...
    }
}
```

```
$ javac Name.java && java Name       # two steps
$ java Name.java                     # single file, Java 11+
```

## Things that catch people

| looks right | is wrong because |
|---|---|
| `if (s == "hello")` | compares references; use `equals` |
| `if (x = 5)` | assignment, not comparison |
| `7 / 2` giving 3.5 | integer division truncates |
| `0.1 + 0.2 == 0.3` | floating point; compare with a tolerance |
| `a[a.length]` | last index is `length - 1` |
| `for (int s : a) s = 0;` | assigns to a copy |
| `list.remove(1)` | removes index 1, not the value 1 |
| `s.toUpperCase()` alone | returns a new string; assign it |
| `result += x` in a loop | quadratic; use `StringBuilder` |
| `x == Double.NaN` | always false; use `Double.isNaN` |
| `int n = map.get(k)` | throws if absent, via unboxing null |
| `grid[0].length` | assumes rectangular; use `grid[row].length` |
| a brace-less `if` with two lines | only the first is conditional |
