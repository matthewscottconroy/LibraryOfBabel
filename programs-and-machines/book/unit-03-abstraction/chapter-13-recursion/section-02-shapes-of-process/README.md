# Shapes of Process

Three lessons about what recursion costs.

The first distinguishes *linear* recursion, where each call makes one further
call, from *tree* recursion, where each makes several — a difference that turns
out to be the difference between a fast method and an unusable one.

The second draws a distinction that is easy to miss and worth having: a method
can be written recursively while the *process* it generates is iterative, and the
two are not the same question.

The third is the honest chapter: when recursion is the wrong choice.
