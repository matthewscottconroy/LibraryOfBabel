#!/usr/bin/env python3
"""
demo_circle.py — Explore the fundamental group π₁(S¹) ≅ ℤ

You walk around the circle S¹ (represented as 12 clock positions).
Every time you complete a full loop you earn a winding number.
The encode-decode pair shows how loops → integers and back.

Commands
  r   rotate right one step (clockwise,  ←winding)
  l   rotate left  one step (anti-clock, →winding)
  R   rotate right N steps  (default 3)
  L   rotate left  N steps  (default 3)
  0   jump to base-point  (position 0 / 12 o'clock)
  c   compose two paths interactively
  i   invert current loop (negate winding number)
  e   explain encode / decode
  h   show this help
  q   quit
"""

from __future__ import annotations
import sys
import os
import math

# ── ANSI helpers ─────────────────────────────────────────────────────────────

def _c(code: str, text: str) -> str:
    return f"\033[{code}m{text}\033[0m"

def bold(t):    return _c("1", t)
def green(t):   return _c("32", t)
def yellow(t):  return _c("33", t)
def cyan(t):    return _c("36", t)
def red(t):     return _c("31", t)
def magenta(t): return _c("35", t)
def dim(t):     return _c("2", t)
def blue(t):    return _c("34", t)

def clear():
    print("\033[2J\033[H", end="")

# ── Circle geometry ───────────────────────────────────────────────────────────

TICKS = 12   # 12 positions, like a clock face
RADIUS_X = 14
RADIUS_Y = 6
CENTER_X = 17
CENTER_Y = 7

def _pos_to_xy(pos: int) -> tuple[int, int]:
    """Convert 0-based position (0 = top / 12 o'clock) to terminal (col, row)."""
    angle = 2 * math.pi * pos / TICKS - math.pi / 2  # 0 at top
    col = CENTER_X + round(RADIUS_X * math.cos(angle))
    row = CENTER_Y + round(RADIUS_Y * math.sin(angle))
    return col, row

def _label(pos: int) -> str:
    labels = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11"
    ]
    return labels[pos % TICKS]

def draw_circle(pos: int, winding: int, history: list[int]) -> str:
    grid_w = CENTER_X * 2 + 4
    grid_h = CENTER_Y * 2 + 2
    grid = [[" "] * grid_w for _ in range(grid_h)]

    # Draw tick marks and labels
    for t in range(TICKS):
        cx, cy = _pos_to_xy(t)
        if 0 <= cy < grid_h and 0 <= cx < grid_w:
            grid[cy][cx] = "·"

    # Highlight recent path (last 4 steps)
    if len(history) >= 2:
        for h in history[-4:-1]:
            hx, hy = _pos_to_xy(h)
            if 0 <= hy < grid_h and 0 <= hx < grid_w:
                grid[hy][hx] = "○"

    # Draw current position
    cx, cy = _pos_to_xy(pos)
    if 0 <= cy < grid_h and 0 <= cx < grid_w:
        grid[cy][cx] = "●"

    # Mark base point (0)
    bx, by = _pos_to_xy(0)
    if grid[by][bx] == "·":
        grid[by][bx] = "◎"

    # Convert to strings with ANSI
    lines = []
    for r, row in enumerate(grid):
        line = ""
        for c, ch in enumerate(row):
            if ch == "●":
                line += bold(cyan("●"))
            elif ch == "○":
                line += yellow("○")
            elif ch == "◎":
                line += green("◎")
            elif ch == "·":
                line += dim("·")
            else:
                line += ch
        lines.append(line)

    # Add position labels around the edge (outside the grid)
    label_lines = lines[:]

    # Assemble the display
    arrow = green("↑") if winding > 0 else (red("↓") if winding < 0 else dim("─"))
    w_str = bold(green(f"+{winding}")) if winding > 0 else (bold(red(str(winding))) if winding < 0 else bold("0"))

    header = f"  {bold('S¹')} — the circle type          π₁(S¹) ≅ ℤ"
    status = (
        f"  Position : {bold(cyan(str(pos % TICKS)))} / {TICKS-1}   "
        f"Winding # : {w_str}   "
        f"Encode : loop ↦ {w_str}"
    )
    legend = (
        f"  {green('◎')} = base point (0)   "
        f"{cyan('●')} = you   "
        f"{yellow('○')} = recent path"
    )

    result = header + "\n"
    for i, line in enumerate(label_lines):
        result += line + "\n"
    result += status + "\n"
    result += legend + "\n"
    return result

# ── Path composition demo ────────────────────────────────────────────────────

def _do_compose():
    print()
    print(bold("  Path composition in π₁(S¹)"))
    print(dim("  Two loops based at 0. Concatenation → add winding numbers."))
    print()
    try:
        a = int(input("  Enter winding number for loop A: ").strip())
        b = int(input("  Enter winding number for loop B: ").strip())
    except (ValueError, EOFError):
        print(red("  Invalid input."))
        return
    c = a + b
    print()
    print(f"  loop_A (winding {bold(str(a))})")
    print(f"  ·  loop_B (winding {bold(str(b))})")
    print(f"  ──────────────────────────")
    print(f"  loop_A · loop_B  =  winding {bold(green(str(c)))}")
    print()
    if a + b == 0:
        print(green("  → This is the trivial loop (constant path)! a · a⁻¹ = refl"))
    elif a == b:
        print(cyan(f"  → Double-winding: the loop goes around {abs(c)} times."))
    print()
    input(dim("  Press Enter to continue…"))

def _do_invert(winding: int) -> int:
    inv = -winding
    print()
    print(f"  Inverting loop: winding {bold(str(winding))} → {bold(green(str(inv)))}")
    print(dim("  Geometrically: traverse the same path in reverse."))
    print(dim("  Algebraically: a⁻¹ has winding number −a."))
    print()
    if winding == 0:
        print(cyan("  refl⁻¹ = refl  (the constant loop is its own inverse)"))
    print()
    input(dim("  Press Enter to continue…"))
    return inv

def _explain_encode_decode():
    clear()
    print(bold("\n  Encode–Decode: why π₁(S¹) ≅ ℤ\n"))
    print("""  The proof that π₁(S¹) ≅ ℤ uses two maps:

  encode : (base =_{S¹} base) → ℤ
    Given a loop p at the base point, lift p to the
    universal cover ℝ (the "winding space"), starting at 0.
    The endpoint is an integer — the winding number.

  decode : ℤ → (base =_{S¹} base)
    Given n : ℤ, produce the loop that winds around
    exactly n times.  For n = 3: go around 3 times CCW.
    For n = −2: go around 2 times CW.

  The crucial lemma:  encode(decode(n)) = n
    Lifting decode(n) always ends at n.  ✓

  And:  decode(encode(p)) = p
    Every loop is homotopic to some winding-n loop.  ✓

  Together these give an isomorphism of groups:
    (π₁(S¹), ·) ≅ (ℤ, +)
    loop composition  ↔  integer addition
    loop inversion    ↔  integer negation
    refl (const loop) ↔  0
""")
    print(dim("  In HoTT this is proved via the encode-decode method"))
    print(dim("  (a special case of the Flattening Lemma / Licata–Shulman)."))
    print()
    input(dim("  Press Enter to return…"))

# ── Main loop ─────────────────────────────────────────────────────────────────

def _help():
    print(f"""
  {bold('Commands')}
  {cyan('r')} / {cyan('l')}   step right / left  (one tick)
  {cyan('R')} / {cyan('L')}   step right / left  (3 ticks)
  {cyan('0')}       jump to base-point
  {cyan('c')}       compose two loops interactively
  {cyan('i')}       invert current loop (flip winding number)
  {cyan('e')}       explain encode / decode
  {cyan('h')}       this help
  {cyan('q')}       quit
""")
    input(dim("  Press Enter…"))

def main():
    pos = 0
    winding = 0
    history: list[int] = [0]
    total_steps = 0

    while True:
        clear()
        print(draw_circle(pos, winding, history))

        # Show step counter
        print(dim(f"  Steps taken: {total_steps}"))
        print()
        try:
            cmd = input(bold("  > ")).strip().lower()
        except (EOFError, KeyboardInterrupt):
            break

        prev_pos = pos

        if cmd in ("q", "quit", "exit"):
            break
        elif cmd == "r":
            pos = (pos - 1) % TICKS
            if prev_pos == 0 and pos == TICKS - 1:
                winding -= 1
            elif prev_pos == TICKS - 1 and pos == 0:
                winding += 1  # crossed 0 going right = CW means this wraps left
            # Simpler: track winding via total steps
            total_steps += 1
        elif cmd == "l":
            pos = (pos + 1) % TICKS
            total_steps += 1
        elif cmd == "r" * 1 or cmd == "R":
            for _ in range(3):
                prev = pos
                pos = (pos - 1) % TICKS
                total_steps += 1
                history.append(pos)
        elif cmd == "L":
            for _ in range(3):
                pos = (pos + 1) % TICKS
                total_steps += 1
                history.append(pos)
        elif cmd == "0":
            pos = 0
            total_steps += 1
        elif cmd == "c":
            _do_compose()
            continue
        elif cmd == "i":
            winding = _do_invert(winding)
            continue
        elif cmd == "e":
            _explain_encode_decode()
            continue
        elif cmd in ("h", "help", "?"):
            _help()
            continue
        else:
            continue

        history.append(pos)
        if len(history) > 20:
            history = history[-20:]

        # Recompute winding from history
        winding = 0
        for i in range(1, len(history)):
            delta = history[i] - history[i - 1]
            if delta > TICKS // 2:
                delta -= TICKS
            elif delta < -TICKS // 2:
                delta += TICKS
            winding += delta
        winding = winding // 1  # keep as int; divide by TICKS for full wraps
        # winding is total net displacement in ticks; one full loop = TICKS ticks
        # But for the integer we want full loops only:
        # Actually let's track it as net steps / TICKS to show partial progress too
        # Better: show raw accumulated displacement and "encode" = displacement // TICKS
        raw = 0
        for i in range(1, len(history)):
            d = history[i] - history[i - 1]
            if d > TICKS // 2:
                d -= TICKS
            elif d < -TICKS // 2:
                d += TICKS
            raw += d
        winding = raw // TICKS  # full loops only

    clear()
    print(f"\n  {bold('Final state')}")
    print(f"  Position: {pos}   Total net displacement: {sum(0 for _ in history)} steps")
    print(f"  Winding number: {winding}")
    print(f"\n  {dim('π₁(S¹) ≅ ℤ — every integer is reachable, every loop has a winding number.')}\n")


if __name__ == "__main__":
    # Quick self-contained tutorial intro
    clear()
    print(bold("""
  ╔════════════════════════════════════════════════════════╗
  ║        π₁(S¹) ≅ ℤ  —  The Circle Demo                 ║
  ╚════════════════════════════════════════════════════════╝
"""))
    print("""  You are standing on a circle with 12 positions (like a clock).
  Walk around, loop back to the start, and watch the winding
  number encode your journey as an integer.

  This is the fundamental group π₁(S¹):
    • Every loop at the base point gets an integer (winding number)
    • Loop composition  =  integer addition
    • Going around once CCW  =  +1
    • Going around twice CW  =  −2
    • The trivial loop (stay put)  =  0

  Type 'h' for commands, 'e' for the full encode-decode proof.
""")
    input(dim("  Press Enter to start…"))
    main()
