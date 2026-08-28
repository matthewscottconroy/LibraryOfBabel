# Assigning Responsibility

There is no compiler for design.

A badly designed program runs exactly as fast as a well designed one, passes the
same tests, and produces the same output. Nothing flags it. The difference appears
weeks later, when a requirement changes and one version takes an afternoon and the
other takes a fortnight — and by then whoever made the decisions is not available
to explain them.

So this section is about a judgment that nothing will check for you, which is why
it is worth having explicit rules of thumb for it.

Three lessons.

First the basic question of design: which class should know a given fact, and
what goes wrong when the answer is *the wrong one*. Then the chapter's main
argument — composition against inheritance — with a demonstration of a class that
is broken by what it inherited. Then coupling and cohesion, the two measures that
give the whole discussion a vocabulary.
