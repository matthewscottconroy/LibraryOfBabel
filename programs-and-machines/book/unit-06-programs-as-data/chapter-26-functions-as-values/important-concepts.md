# Important Concepts

**Behavior as a value** — a piece of code that can be passed, returned, or stored,
exactly as an `int` can. Chapter 25's `Procedure` record made this concrete;
Java has had it since 2014.

**Functional interface** — an interface with exactly one abstract method.
`@FunctionalInterface` asserts it and makes adding a second one a compile error.

**A lambda is an implementation of a functional interface** — not a new kind of
value and not a function type. `x -> x * x` assigned to an `IntOp` *is* an
`IntOp`, and calling it is an ordinary `invokeinterface`.

**Why Java has no function types** — compatibility. `Comparator`, written in 1998,
accepts lambdas without a line changing. The cost is that a lambda has no type of
its own, so `var f = x -> x * x;` does not compile.

**The six standard interfaces** — `Function`, `Predicate`, `Consumer`, `Supplier`,
`UnaryOperator`, `BinaryOperator`. The other thirty-odd in
`java.util.function` are primitive specializations that exist to avoid boxing.

**Lambda syntax** — parentheses optional for one parameter, types inferred, a
single expression is the result, a braced block needs `return`.

**Capture** — a lambda may use effectively final locals of the enclosing method,
fields via `this`, and statics. This is Chapter 25's closure, in Java.

**Effectively final** — never assigned after initialization. Required because a
local lives in a stack frame that may be gone before the lambda runs, so the value
is copied; allowing assignment would make the copy silently disagree.

**The array escape hatch** — `int[] counter = {0}` sidesteps the restriction and
removes the nudge that motivated it. Verified failing under parallelism: three
runs counting a million elements gave 97282, 78637 and 906250.

**Lambdas are not anonymous classes** — `this` means the enclosing object rather
than the anonymous instance, and no separate class file is generated.

**Method references** — `String::length`, `Integer::sum`, `System.out::println`,
`ArrayList::new`. The instance-method form supplies the receiver as the first
argument, which is why `String::length` satisfies `Function<String,Integer>`.

**Higher-order method** — one that takes a function, returns one, or both.
`Collections.sort` has taken a `Comparator` since 1998; only the notation is new.

**Functional parameter versus template method** — the same structure with no
class, no inheritance, and a run-time choice. The template method still wins when
several holes must be filled coherently or when there is shared state.

**Returning a function** — `adder(5)` produces an object holding 5 and code that
uses it. A closure and a one-field, one-method object are the same thing under two
notations.

**Composition** — `and`, `or`, `negate`, `andThen`, `compose` are default methods
on the standard interfaces, which turns a set of small predicates into a
vocabulary.

**Pure function** — reads only its arguments, does nothing but return a value.
Movable, cacheable, testable in isolation, safe on several threads.

**Referential transparency** — a call can be replaced by its result without
changing the program. What the parallel counter lacked and the stream assumed.

**Map, filter, reduce** — transform each element, keep the ones that pass a test,
combine everything into one value. Most loops are one of these three.

**Reduce and the identity** — an empty input returns the identity, so it must be
genuinely neutral. This is Chapter 13's accumulator passing, now with the
accumulator as the only state, which is why it parallelizes.

**A stream is not a collection** — it holds no elements; it describes a computation
over a source. Everything else follows from that.

**Source, intermediate, terminal** — nothing happens until the terminal operation
runs. A pipeline with no terminal operation does nothing, silently.

**Laziness** — elements are pulled through the whole pipeline one at a time.
`peek` verified that `findFirst` examined four of ten elements.

**Stage fusion** — no intermediate collections are built, which is why streams are
competitive with loops at all.

**Short-circuiting** — `findFirst`, `anyMatch`, `allMatch`, `limit` stop as soon as
the answer is known, which is what makes infinite streams usable.

**Collectors** — `groupingBy(Person::city, counting())` in one expression against
six lines of `computeIfAbsent`. The strongest case for the style.

**Optional** — a container of zero or one values, making absence visible in the
type. Use as a return type, not a field or parameter, and avoid bare `get()`.

**Primitive streams** — `IntStream` and friends exist because `Stream<Integer>`
boxes. Measured: 3 ms against 29 ms over ten million elements.

**parallelStream is one word and a trap** — it requires purity and associativity,
and gives wrong answers rather than errors when they are absent. Chapter 31.

**When a loop is clearer** — no index in streams, no `zip`, awkward early exits on
accumulated state, mutation, and long bodies.

**Debugging cost** — laziness makes stepping counter-intuitive and traces are full
of synthetic frames. A pipeline that is hard to debug is usually too long.

**The discipline outlives the syntax** — pure functions, effects at the edges,
immutable data. All available since Chapter 11; lambdas make the pure parts small
enough to pass around.
