# Further Reading

## Getting set up

The official JDK installation instructions for your platform, from Oracle or
from an OpenJDK distribution such as Adoptium, Amazon Corretto, or your system's
package manager.

Any JDK of version 17 or later will run everything in this book. Note that you
need a **JDK** and not merely a **JRE**: a JRE can run compiled programs but has
no compiler, so `javac` will be missing and `java Hello.java` will fail. If you
have hit exactly that error, this is why.

## Reference

*The Java Language Specification*, Java SE 17 edition. Gosling, J., Joy, B.,
Steele, G., Bracha, G., Buckley, A., Smith, D., Bierman, G. Oracle.

Chapter 4 covers types and values; Section 4.2 states the ranges and the
two's-complement guarantee. Not a tutorial. Worth opening once now, so that it
stops being intimidating and becomes a thing you know how to look something up
in.

*The Java Virtual Machine Specification*, Java SE 17 edition. Lindholm, T.,
Yellin, F., Bracha, G., Buckley, A., Smith, D. Oracle.

Chapter 2 describes the abstract machine sketched in Section 5.1.2, including the
stack discipline and the typed instruction families. Read the first ten pages
when you are curious; the rest is for implementers.

The Java API documentation, at the `java.base` module.

You will use this constantly for the rest of the book. Get into the habit now of
looking up a method rather than guessing at it — `Integer.toBinaryString`,
`Double.toHexString`, and `String.repeat` all appeared in this chapter's
instrument, and all are documented there.

## On compilers, if you are curious

Aho, A. V., Lam, M. S., Sethi, R., & Ullman, J. D. (2006). *Compilers:
Principles, Techniques, and Tools* (2nd ed.). Pearson.

The standard text, known as the dragon book. Far beyond this chapter, and the
place to go if the question "how does `javac` actually work" will not leave you
alone. Chapter 24 of this book builds a small parser, which is the same subject
at one thousandth the scale.

## Talks

Steele, G. L. (1998). "Growing a Language." OOPSLA keynote.

Referenced in the profiles, and worth watching rather than reading. The
constraint he places on himself becomes apparent within two minutes and changes
how you hear the rest.

## On learning to program

Ericsson, K. A., Krampe, R. T., & Tesch-Römer, C. (1993). "The Role of Deliberate
Practice in the Acquisition of Expert Performance." *Psychological Review*,
100(3), 363–406.

Not about programming. It is the research behind the claim that improvement comes
from practice at the edge of your ability with immediate feedback — which is why
this chapter's exercises ask you to predict before running. The prediction
supplies the feedback. Reading this once may change how you use the rest of the
book.
