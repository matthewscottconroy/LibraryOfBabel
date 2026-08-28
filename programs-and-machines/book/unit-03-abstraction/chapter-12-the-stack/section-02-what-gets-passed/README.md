# What Gets Passed

"Java passes objects by reference" is the single most repeated wrong sentence about
this language.

It is wrong in a way that matters, because it predicts the wrong answer for a
specific and common case, and people who believe it write a method to modify their
caller's variable and cannot work out why it does nothing.

Three lessons, and the first two are the chapter's reason for existing.

Values and references first: what a variable of object type actually holds, which
is the fact everything else depends on. Then the sentence people get wrong — Java
passes by value, always — and a careful demonstration of why the two behaviors
that seem to contradict it do not.

Then overloading: several methods sharing a name, distinguished by their
parameters, and the rules the compiler uses to pick.
