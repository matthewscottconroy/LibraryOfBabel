"""Verify the sum formula computationally."""

def sum_direct(n): return sum(range(n + 1))
def sum_formula(n): return n * (n + 1) // 2

for n in range(20):
    d = sum_direct(n)
    f = sum_formula(n)
    status = 'OK' if d == f else 'FAIL'
    print(f"n={n:2d}: direct={d:3d}, formula={f:3d} [{status}]")
