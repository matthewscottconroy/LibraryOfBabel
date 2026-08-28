# The Array

Ten variables named `score1` through `score10` are tolerable. A hundred are not,
and a number of scores you will not know until the program runs is impossible.

What you want is one name and a way to say *which one* — and the way that gets
implemented turns out to explain almost everything an array does and does not let
you do.

Three lessons.

The first is the argument for why indices are arithmetic and why that forces
every other property arrays have. The second is the practical business of
declaring, filling, and reading them. The third is bounds checking — what Java
does on every access, why it costs something, and what it buys.
