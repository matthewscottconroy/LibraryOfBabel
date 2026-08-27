# Further Reading

## On the mechanism

*The Java Virtual Machine Specification*, Java SE 17 edition. Oracle. Sections
2.5 and 2.6.

Section 2.5 describes the runtime data areas — the stacks, the heap, the method
area. Section 2.6 describes frames, including the local variable array and the
operand stack. Ten pages, and it is the authoritative version of Section 12.1.1.

Patterson, D. A., & Hennessy, J. L. *Computer Organization and Design*. Morgan
Kaufmann. Section 2.8, "Supporting Procedures in Computer Hardware".

The same material at the machine level: the stack pointer, the calling
convention, which registers a callee must preserve. Shows what the JVM's
abstraction is an abstraction *of*.

## The historical papers

Samelson, K., & Bauer, F. L. (1960). "Sequential Formula Translation."
*Communications of the ACM*, 3(2), 76–83.

The stack introduced for nested structure. Worth reading for how the idea is
motivated by parenthesized expressions before being applied to procedures.

Dijkstra, E. W. (1960). "Recursive Programming." *Numerische Mathematik*, 2,
312–318.

How to implement recursion with activation records, by the person who first did
it. Short.

Wilkes, M. V., Wheeler, D. J., & Gill, S. (1951). *The Preparation of Programs
for an Electronic Digital Computer*. Addison-Wesley.

For the Wheeler jump, and for what subroutine calling looked like before the
stack.

## On pass-by-value

*The Java Language Specification*, Java SE 17 edition. Oracle. Section 8.4.1.

"When the method or constructor is invoked, the values of the actual argument
expressions initialize newly created parameter variables." That sentence settles
the question, and it is worth having read it in the specification rather than in
an argument.

Bloch, J. (2018). *Effective Java* (3rd ed.). Addison-Wesley. Items 50 and 52.

Item 50, "Make defensive copies when needed", is the practical consequence of a
method being able to modify what it is passed. Item 52, "Use overloading
judiciously", is Section 12.2.3 with more examples of it going wrong — including
the `List.remove` trap.

## Diagnosing

Oracle's documentation for `-Xss` (stack size) and `jstack` (dumping the stacks of
a running JVM).

`jstack` is worth knowing exists. When a program appears hung, it prints what
every thread is doing, which is frequently the entire diagnosis. It becomes far
more useful after Chapter 31.
