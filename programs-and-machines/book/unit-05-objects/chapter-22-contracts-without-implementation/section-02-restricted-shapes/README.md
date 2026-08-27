# Restricted Shapes

Two lessons, and one idea shared between them.

Interfaces get their power by saying less — they name a capability and leave
everything else open. Enums and records get theirs by saying *more*, and by
saying it in a form the compiler can use. An enum declares that a set is closed.
A record declares that a type is nothing but its components.

Both hand the compiler a guarantee and get checking back for it. Both replace
code you would otherwise write by hand and get wrong.
