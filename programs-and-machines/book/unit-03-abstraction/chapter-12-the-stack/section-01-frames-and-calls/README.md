# Frames and Calls

Three lessons on the mechanism.

The call frame is the packet of state belonging to one execution of one method:
its parameters, its locals, and the address to return to. The stack discipline is
the rule for organizing frames, and it turns out to be forced — given that calls
nest, no other arrangement works. And stack overflow is what happens when the
frames run out of room, which is a genuinely useful error to understand rather
than merely to recognize.
