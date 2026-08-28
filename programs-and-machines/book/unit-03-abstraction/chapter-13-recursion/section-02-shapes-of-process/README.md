# Shapes of Process

The last section argued that recursion is a good way to think. This one is about
when it is an expensive way to compute.

Two methods can look almost identical, differ by a single extra call, and differ in
running time by a factor that grows without bound. Being able to see that
difference on the page — before running anything — is the point of what follows.

Three lessons about what recursion costs.

The first distinguishes *linear* recursion, where each call makes one further
call, from *tree* recursion, where each makes several — a difference that turns
out to be the difference between a fast method and an unusable one.

The second draws a distinction that is easy to miss and worth having: a method
can be written recursively while the *process* it generates is iterative, and the
two are not the same question.

The third is the honest chapter: when recursion is the wrong choice.
