# Exercises

**27.1** Take a class of your own with at least three fields and two methods.
Print its name, its superclass, every declared field with its type and modifiers,
and every declared method with its return type. Then run the same code with
`getFields` and `getMethods` instead of the `Declared` versions, and explain the
difference in the output.

**27.2** Use reflection to read and then modify a `private` field of an object from
outside its class. Then do the same to a `private final` field. Write two
sentences on what this means for the guarantees of Chapters 19 and 20.

**27.3** Remove the `setAccessible(true)` from Exercise 27.2 and read the
exception. Say what the checks are and when they run.

**27.4** Load `java.util.ArrayList` by name with `Class.forName`, construct one,
and add an element — without the word `ArrayList` appearing as a type anywhere.
Then misspell the method name and describe when and how you find out.

**27.5** *Measurement.* Benchmark a direct method call against `Method.invoke`,
ten million iterations each, after warm-up. Report both times and the ratio. Then
benchmark `getMethod` alone and say where the cost actually is.

**27.6** Confirm that `new ArrayList<String>().getClass() == new
ArrayList<Integer>().getClass()`. Then explain in one sentence what a
`List<String>` is at run time.

**27.7** Reproduce heap pollution: assign a `List<String>` to a raw `List`, add an
`Integer`, and then read element zero as a `String`. Report where the exception
occurs and why it is not where you added the element.

**27.8** For each of these, say which erasure restriction it violates and why the
restriction follows: `x instanceof List<String>`, `new T[10]`,
`void f(List<String>)` alongside `void f(List<Integer>)`, `List<int>`.

**27.9** Use `getGenericParameterTypes()` to recover the full generic type of a
method parameter. Then explain the rule about what survives erasure and what does
not, using your result as the example.

**27.10** Declare an annotation with `RUNTIME` retention and a `value` member.
Put it on three fields of a class and write a loop that prints each annotated
field with its annotation's value. Then change the retention to `CLASS`, run
again, and explain the output.

**27.11** Write the fifteen-line test runner from Section 27.2.2. Give it a suite
with three passing tests, one failing test, and one unannotated method. Confirm
the counts. Then make it also honor an `@Ignore` annotation.

**27.12** Look up the declaration of `java.lang.Override` in the JDK source. It has
no members and `SOURCE` retention. Explain, in a paragraph, how something that
survives only to the end of compilation and contains no information prevents the
`equals(Point)` bug from Chapter 20.

**27.13** *Longer.* [carries forward] Write a small object-to-text serializer: given any object,
produce a line per field using reflection, honoring an optional
`@Name("column_name")` annotation to override the field name and a `@Skip`
annotation to omit a field. Test it on three unrelated classes it has never seen.
Then say what it cannot handle, and check your list against Section 27.2.1.

**27.14** *Design, no code.* You are asked to add a feature that dispatches to one
of six handlers based on a string from a configuration file. One colleague
proposes `Class.forName` on a fully qualified name from the file. Another proposes
an enum with per-constant behavior and a lookup by name. Argue for one, using
Section 27.1.2's four costs.
