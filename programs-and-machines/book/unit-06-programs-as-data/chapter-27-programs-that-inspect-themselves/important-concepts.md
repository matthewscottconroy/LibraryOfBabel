# Important Concepts

**Reflection** — a running program examining and manipulating its own structure:
classes, fields, methods, constructors, modifiers and annotations, all by name and
all decided at run time.

**Class as an object** — `Account.class` and `a.getClass()` return the same object,
because the JVM creates exactly one `Class` per loaded class.

**getDeclared versus not** — `getDeclaredFields` and `getDeclaredMethods` return
everything the class declares including private members; `getFields` and
`getMethods` return only public ones including inherited. Choosing wrong is the
commonest reflection bug.

**Reading and writing by name** — `getDeclaredField("cents").setLong(obj, v)`
modifies a private field from outside the class. Verified, including on a `private
final` field.

**setAccessible** — the call that suppresses the access checks. A deliberate
design decision, because frameworks legitimately need to reach into classes they
do not own. Since Java 9 a module may refuse it.

**Encapsulation is enforced against ordinary code and negotiable against
reflection** — Chapter 19's boundary holds for every reader playing by the rules.

**Construction by name** — `Class.forName` plus `getDeclaredConstructor().
newInstance()` lets a program use a class that did not exist when it was compiled.
This is what every plugin system and framework is built on.

**Type safety is suspended** — a method name is a string, so a typo becomes
`NoSuchMethodException` at run time rather than a compile error.

**InvocationTargetException** — wraps whatever the invoked method threw. The real
exception is in `getCause()`, and forgetting to unwrap produces useless traces.

**The speed cost** — measured at roughly forty to sixty times a direct call: no
inlining, arguments boxed into an `Object[]`, return value boxed, access
re-checked. Lookup is cheap by comparison, at about twenty nanoseconds.

**The tooling cost** — find-usages misses reflective calls, dead-code elimination
and obfuscation break, and ahead-of-time compilation cannot see reflective
references. This is why frameworks have been moving work to compile time.

**When reflection is right** — code that must work with classes it has never seen:
frameworks, tools, and loading by configuration. If you know the class at compile
time, call the method.

**Erasure** — type arguments are checked during compilation and removed from the
bytecode. `List<String>` and `List<Integer>` are the same class at run time, both
holding `Object` references.

**The compile-time guarantee** — if your code compiled without unchecked warnings,
the compiler-inserted casts will never fail. Unchecked warnings mark exactly where
the guarantee has a hole.

**Heap pollution** — an `Integer` inside a `List<String>`, admitted through a raw
type. The failure appears at the *read*, in a cast that does not appear in the
source.

**Why erasure** — migration compatibility. Code compiled in 1999 had to keep
working with generic libraries, and `List<String>` and `List` compile to the same
bytecode. C# chose reified generics because its ecosystem was young enough to
break.

**What erasure forbids** — `instanceof` with a type argument, `new T[]` and
`new T()`, overloading on type arguments, generic exceptions, and primitives as
type arguments. Each follows mechanically from there being nothing at run time.

**No `List<int>`** — the largest consequence. It is why autoboxing exists, why
`Stream<Integer>` was ten times slower than `IntStream`, and why
`java.util.function` needs forty interfaces.

**What survives** — generic information is retained where it is *declared* and
erased where it is *used*. A method's signature is recoverable with
`getGenericParameterTypes`; an object's type argument is not, because the object
never had one.

**Super type token** — `new TypeReference<List<Person>>() {}`. The empty braces
create a subclass, which makes the type argument part of a class declaration and
therefore retained.

**Annotation** — information attached to code for another program to read. It
changes nothing by itself.

**@Target** — where an annotation may be written. Setting it turns a misplaced
annotation into a compile error.

**@Retention** — `SOURCE`, `CLASS`, or `RUNTIME`. An annotation read reflectively
must be `RUNTIME`; forgetting produces the commonest annotation bug, where
everything compiles and nothing is found.

**The `value` member** — an annotation with a single member named `value` may omit
the name at the use site, which is why so many annotations have one.

**Annotations are declarative** — an annotation cannot execute, only be found.
Something else must do the work.

**Four kinds of reader** — nobody, the compiler, an annotation processor, and the
running program. `@Override` is the second, `@Test` is the fourth.

**@Override explained** — no members, `SOURCE` retention, and a rule in the
compiler that reports an error when the marked method overrides nothing. It does
not change dispatch; it is an assertion, checked.

**Annotation processing** — generating source at compile time from annotated code.
The direction modern frameworks are moving, for exactly the reasons reflection is
costly.

**The unit's through-line** — the boundary between code and data is a choice, not
a fact, and it is drawn differently by a compiler, an interpreter, a serializer,
and a test runner.
