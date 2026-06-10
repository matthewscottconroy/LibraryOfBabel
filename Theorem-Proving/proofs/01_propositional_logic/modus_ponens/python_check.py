"""Verify Modus Ponens by truth table."""

def implies(p, q):
    return (not p) or q

print("Modus Ponens: P, P→Q ⊢ Q")
print("P | Q | P→Q | Premises | Q (conclusion)")
for p in [True, False]:
    for q in [True, False]:
        pq = implies(p, q)
        premises = p and pq
        valid = (not premises) or q   # if premises hold, does conclusion?
        print(f"{int(p)} | {int(q)} |  {int(pq)}  |    {int(premises)}     | {int(q)}  {'✓' if valid else '✗'}")

print()
print("All rows valid:", all(
    (not (p and implies(p,q))) or q
    for p in [True,False] for q in [True,False]
))
