# The JVM as a Machine

The compiler produces instructions for a machine that does not exist. This
deserves an explanation.

## The problem it solves

Processors differ. An Intel chip and an ARM chip have different instruction sets
— different opcodes, different registers, different rules — so machine code
compiled for one is meaningless to the other. Chapter 1's endianness discussion
was one small instance of a much larger incompatibility.

Traditionally this means compiling separately for each target: a Windows build, a
Mac build, a Linux build, and now separate builds for Intel and ARM versions of
each. Every combination is a separate artifact to produce and test.

Java's answer is to add a layer. Compile once, to instructions for a *specified*
machine that is defined by a document rather than by silicon. Then write, for
each real processor, one program that executes those instructions.

That program is the **Java Virtual Machine**, and the arrangement is why
`Hello.class` compiled on your laptop runs unmodified on a server you have never
seen.

## What the JVM is

The JVM is a specification of a machine: what instructions exist, what they do,
how memory is organized, what happens on error. It has an instruction set of
around 200 opcodes, and unlike a physical processor it is a **stack machine** — it
has no general-purpose registers, and instructions take their operands from and
leave their results on a stack.

Adding two numbers looks like this:

```
iload_1      push the value of local variable 1
iload_2      push the value of local variable 2
iadd         pop two, add, push the result
istore_3     pop, store into local variable 3
```

The `i` prefix means integer. There are parallel families for `long`, `float`,
and `double` — `ladd`, `fadd`, `dadd` — because the JVM's instructions are typed,
which is Chapter 1's point about operations belonging to the agreement, showing up
directly in an instruction set.

A stack machine was chosen partly because it makes bytecode compact and easy to
verify, and partly because it does not bake in assumptions about how many
registers the real processor has.

## Interpretation and compilation, again

The first JVMs read bytecode one instruction at a time and did what each said —
**interpretation**. That is straightforwardly slower than running native machine
code, and it gave Java an early reputation for being slow that outlasted the
condition by many years.

Modern JVMs interpret at first, while watching. When a method has run enough
times to be worth the effort, a **just-in-time compiler** translates that method's
bytecode into real machine code for the actual processor, and subsequent calls run
natively.

The result is sometimes faster than an ahead-of-time compiled language, which
sounds impossible until you see why: the JIT knows things a static compiler
cannot. It knows which branches actually get taken, which types actually turn up
at a call site, what the input data looks like today. It can compile
optimistically for the common case and keep a slower path in reserve.

You will see the cost of this in Unit VIII when we measure performance, because it
means a Java program is slow for its first few thousand iterations and then
abruptly faster. Benchmarks that do not account for warm-up are worthless, and a
great many published benchmarks do not.

## What you should take from this

Three practical consequences.

**Portability is real but not free.** The same `.class` file runs anywhere there
is a JVM — but a JVM has to exist for that platform, and it is a substantial
piece of software.

**"Compiled" and "interpreted" are not a clean division.** Java is compiled to
bytecode, which is interpreted, and then compiled again to machine code while
running. Languages you will meet do all sorts of combinations of these, and the
categories are less useful than they look.

**Errors arrive at two different times.** Compile-time errors come from `javac`
before anything runs. Run-time errors come from the JVM during execution. Knowing
which kind you are looking at tells you which tool to blame and where to look, and
new programmers lose a lot of time to not making the distinction.

Now let us write something.
