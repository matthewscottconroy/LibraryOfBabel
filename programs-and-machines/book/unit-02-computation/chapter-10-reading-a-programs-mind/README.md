# Reading a Program's Mind

Everything in this unit so far has been about what a program *is*. This chapter
is about the situation you will actually spend your time in: a program exists, it
is not doing what you expected, and you need to find out why.

I want to be blunt about the proportions. You will spend far more of your
programming life reading and correcting code than writing it fresh — most
estimates put reading at several times writing, and debugging is a large share of
that. It is the central activity, and it is usually taught by omission: you are
shown how to write programs, the programs fail, and you are left to develop a
method by yourself.

Most people develop a bad one. The bad method is to change something plausible
and rerun, repeatedly, until the symptom disappears. It sometimes works, it
teaches you nothing, and it frequently leaves the actual defect in place with the
symptom masked.

The good method has a name in other fields: it is the experimental method. You
have a belief about what the program does. The program disagrees. Find the
specific place where your belief and the program's behavior first diverge, by
making observations that distinguish between hypotheses.

That is all debugging is. The rest is technique.

**Tracing by Hand** is the fundamental skill: following a program's state through
its execution on paper. It is slow, and doing it a dozen times builds an accurate
mental model of the machine that nothing else supplies. Chapter 6 said a
computation is a sequence of states; a trace is that sequence, written down.

**When It Goes Wrong** is the practical half. How to read a Java error message
and stack trace — which contains far more information than beginners extract from
it. How to find a bug by bisection rather than by guessing. And what a debugger
gives you that printing does not.

One thing before we start, and it is not technical.

When a program does something you did not expect, the machine is not being
capricious. Chapter 6 established that a machine is deterministic: same state,
same behavior. If it did something surprising, then one of your beliefs about the
state is wrong, and it is a specific belief that can be found.

That reframing is the difference between "this is broken and I do not know why"
— a hopeless state — and "one of my assumptions is false; which one?" — a
question with a method attached.

You will be wrong about your own code constantly. Everybody is. The skill is not
avoiding that; it is finding the false belief quickly and without ego.
