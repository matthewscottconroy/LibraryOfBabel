# Further Reading

## The framework

Bloch, J. (2018). *Effective Java* (3rd ed.). Addison-Wesley. Items 26–33 and
64.

Item 26 is "Don't use raw types". Item 28, "Prefer lists to arrays", explains a
subtlety this chapter avoided — that arrays are covariant and generics are not, so
`Object[] a = new String[1]` compiles and fails at run time while the generic
equivalent is rejected at compile time. Item 64 is the declare-by-interface rule.

The Java Collections Framework documentation and tutorial, from Oracle.

The overview pages are genuinely good, and the table of implementations with their
costs is worth printing.

## On generics

Naftalin, M., & Wadler, P. (2006). *Java Generics and Collections*. O'Reilly.

The full treatment, by one of the people who designed the type system. The chapters
on erasure and on wildcards — the `? extends T` notation this chapter avoided —
are the clearest available.

Gafter, N. Various posts on his blog about erasure and reification.

For why Java made the choice it did, from the person who implemented it.

## On amortized analysis

Cormen, T. H., Leiserson, C. E., Rivest, R. L., & Stein, C. (2009). *Introduction
to Algorithms* (3rd ed.). MIT Press. Chapter 17.

Amortized analysis proper, with the dynamic-table example that is exactly Section
17.1.2 done rigorously. Three methods — aggregate, accounting, potential — for
proving the same result.

Tarjan, R. E. (1985). "Amortized Computational Complexity." *SIAM Journal on
Algebraic and Discrete Methods*, 6(2), 306–318.

Where the term was introduced.

## On data structures generally

Sedgewick, R., & Wayne, K. (2011). *Algorithms* (4th ed.). Addison-Wesley.

In Java, and the treatment of the underlying structures — resizing arrays, linked
lists, hash tables, balanced trees — is the natural companion to this chapter.
Chapter 1.3 covers exactly what `ArrayList` does internally.

## On cache effects

Drepper, U. (2007). "What Every Programmer Should Know About Memory." LWN.net.

Recommended in Chapter 15, and the explanation for why `LinkedList` loses cases
that theory says it should win. Chapter 32 revisits the gap between predicted and
measured cost.
