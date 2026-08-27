# Further Reading

## The practical books

Kernighan, B. W., & Pike, R. (1999). *The Practice of Programming*.
Addison-Wesley. Chapter 5, "Debugging".

Thirty pages, and the best concentrated treatment of the subject. Their advice
overlaps this chapter's substantially, which I take as confirmation rather than
coincidence. The whole book is short and worth reading once you are comfortable
with Unit III.

Agans, D. J. (2002). *Debugging: The Nine Indispensable Rules for Finding Even the
Most Elusive Software and Hardware Problems*. AMACOM.

Written by a hardware engineer, which makes the examples unfamiliar and the
principles clearer. Each rule gets a chapter of war stories.

Zeller, A. (2009). *Why Programs Fail: A Guide to Systematic Debugging* (2nd ed.).
Morgan Kaufmann.

The academic treatment, and the source of *delta debugging* — an algorithm that
automates the minimization of Section 10.2.2. If the idea that input reduction
could be mechanical interests you, this is where it is worked out.

## On tracing and understanding

Wilkes, M. V. (1985). *Memoirs of a Computer Pioneer*. MIT Press.

For the passage quoted in the profiles, and for a first-hand account of what it
was like when none of this was known.

Petzold, C. (1999). *Code*. Microsoft Press.

Recommended in Chapters 1 and 8, and relevant again: a mental model accurate
enough to trace against is what the whole book supplies.

## Tools

The debugger documentation for your development environment — IntelliJ IDEA,
Eclipse, VS Code, or `jdb` at the command line.

Spend twenty minutes with it deliberately: set a breakpoint, inspect a variable,
step into a method, click up the call stack, and set a conditional breakpoint.
Most people never learn the last two, and they are the ones that matter.

The `jdb` documentation, if you want to see what a debugger is underneath.
Clumsy to use and instructive to have used once.

## On the size of the problem

Britton, T., Jeng, L., Carver, G., Cheak, P., & Katzenellenbogen, T. (2013).
*Reversible Debugging Software: Quantify the Time and Cost Saved Using Reversible
Debuggers*. Cambridge Judge Business School.

Frequently cited for the estimate that developers spend a substantial fraction of
their time debugging. Treat the precise figure with caution — such studies are
hard to do well — but the order of magnitude is not controversial, and it is the
justification for this chapter existing at all.
