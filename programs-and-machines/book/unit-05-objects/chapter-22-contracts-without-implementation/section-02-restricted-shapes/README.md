# Restricted Shapes

Everything so far in this chapter has gained power by promising *less*. An
interface says what a thing can do and stays silent about everything else, and that
silence is what keeps you free.

The two constructs in this section do the opposite. They promise *more* — this set
has exactly three members; this type is nothing but its two fields — and get
something back for it. Watching that trade run in reverse is the point of putting
them here.

Two lessons, and one idea shared between them.

Interfaces get their power by saying less — they name a capability and leave
everything else open. Enums and records get theirs by saying *more*, and by
saying it in a form the compiler can use. An enum declares that a set is closed.
A record declares that a type is nothing but its components.

Both hand the compiler a guarantee and get checking back for it. Both replace
code you would otherwise write by hand and get wrong.
