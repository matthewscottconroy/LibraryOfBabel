# Key Concepts

**Compiler.** A program that translates source text into another form. It does
not run your program; running is a separate step. `javac Hello.java` produces
`Hello.class`.

**Filename and class name must match.** A public class `Hello` must live in
`Hello.java`, so that a class can be located by name without searching.

**Static typing.** Types are checked at compile time, before the program runs.
Java also checks definite assignment and reachability, on the principle that
failing at compile time is cheaper than failing in production.

**Bytecode.** The instruction set the compiler targets — instructions for a
specified machine rather than for any physical processor.

**The Java Virtual Machine.** A program implementing that specified machine. One
compiled `.class` file runs anywhere a JVM exists, which is what "write once, run
anywhere" means. The JVM is a stack machine with typed instructions: `iadd` for
`int`, `dadd` for `double`.

**JIT compilation.** Modern JVMs interpret bytecode at first, then compile
frequently executed methods to native machine code while running. This is why
Java programs are slow for their first few thousand iterations and then abruptly
faster — and why benchmarks that ignore warm-up are worthless.

**Compile-time versus run-time errors.** `javac` reports the first kind before
anything executes; the JVM reports the second during execution. Knowing which you
are looking at tells you where to look.

**`main`.** The method the JVM calls to start a class. Its signature
`public static void main(String[] args)` is fixed by specification.

**Single-file source execution.** Since Java 11, `java Hello.java` compiles in
memory and runs, leaving no `.class` file. Convenient for experiments; the
two-step model still applies underneath.

**The eight primitive types.** `byte`, `short`, `int`, `long` are two's
complement integers of 8, 16, 32, 64 bits. `float` and `double` are IEEE 754
single and double precision. `char` is a 16-bit *unsigned* UTF-16 code unit.
`boolean` has no specified size.

**Literals carry types.** `3.14` is a `double`; `3.14f` is a `float`.
`2147483648` will not compile as an `int`; `2147483648L` is a `long`. Single
quotes make a `char`, double quotes make a `String`.

**Integer division truncates.** `7 / 2` is 3, not 3.5, because both operands are
`int`. Truncation is toward zero, so `-7 / 2` is −3. `%` returns what division
discarded.

**The type decides the operation.** `/` is a family of operations, not one. `1 / 0`
throws because integers have no infinity; `1.0 / 0` yields `Infinity` because
floating point does. Same operator, different agreement.

**Widening is silent, narrowing needs a cast.** Conversions that cannot lose
information happen automatically. Ones that can require `(int)`, which is you
accepting responsibility. `(int) 3.99` truncates to 3; `(byte) 300` keeps the low
eight bits and gives 44.

**Printing.** `print`, `println`, and `printf`. Format specifiers `%d`, `%s`,
`%x`, `%n`, with optional width and `-` for left alignment. Prefer `%n` to `\n`.

**Concatenation evaluates left to right.** `"sum: " + 1 + 2` gives `sum: 12`;
parenthesize the arithmetic to get `sum: 3`.

**Observe rather than guess.** When a program is inexplicable, print the value.
Better: predict what will print, then check. The prediction is where the learning
is.
