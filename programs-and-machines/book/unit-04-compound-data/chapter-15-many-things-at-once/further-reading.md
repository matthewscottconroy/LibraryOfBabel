# Further Reading

## On memory and caches

Drepper, U. (2007). "What Every Programmer Should Know About Memory." LWN.net.

A long, free, and unusually good article on how memory actually works: cache
lines, associativity, prefetching, and why access patterns matter. Section 3 is
the part relevant here. More detail than you need now and the right place to come
back to when Section 15.2.3's measurement makes you curious.

Wilkes, M. V. (1965). "Slave Memories and Dynamic Storage Allocation."
*IEEE Transactions on Electronic Computers*, EC-14(2), 270–271.

Two pages, and the origin of the cache.

Patterson, D. A., & Hennessy, J. L. *Computer Organization and Design*. Morgan
Kaufmann. Chapter 5, "Large and Fast: Exploiting Memory Hierarchy".

The textbook treatment, with the arithmetic of hit rates and miss penalties.

## On bounds checking and safety

Hoare, C. A. R. (1981). "The Emperor's Old Clothes." *Communications of the ACM*,
24(2), 75–83.

His Turing Award lecture, and one of the best pieces of writing in the field. The
passage on array bounds checking is short and worth the whole piece; so is his
account of a language design project that failed.

The Microsoft Security Response Center's analyses of memory-safety defect rates,
and the Chromium project's published security statistics.

Both have repeatedly reported that around two thirds of serious vulnerabilities in
their large C and C++ codebases are memory-safety issues. Worth looking at the
current numbers rather than taking Section 15.1.3's figure on trust.

## On arrays and algorithms

Knuth, D. E. *The Art of Computer Programming, Volume 1: Fundamental Algorithms*
(3rd ed., 1997). Addison-Wesley. Section 2.2.

Arrays and sequential allocation treated exhaustively, including the address
arithmetic for multi-dimensional arrays stored contiguously — which is what Java
does not do, and the comparison is instructive.

Bentley, J. (2000). *Programming Pearls* (2nd ed.). Addison-Wesley.

Short essays on making programs fast and small, several of them about array layout
and traversal order. Column 8's account of finding the maximum-sum subarray is a
small classic.

## Java specifics

*The Java Language Specification*, Java SE 17 edition. Oracle. Chapter 10,
"Arrays".

Covers the array store check, the covariance rule, and the exact semantics of
`length`. Section 10.10 explains why storing the wrong type into an array of
objects throws at run time — a wrinkle this chapter did not raise and which
Chapter 17 will.

The `java.util.Arrays` documentation.

Worth reading through once. Several methods there are ones people reimplement by
hand because they did not know they existed.
