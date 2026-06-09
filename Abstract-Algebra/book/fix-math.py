#!/usr/bin/env python3
"""Fix inline math formatting for pandoc tex_math_dollars compatibility.

Two problems to fix:
1. $X{}$ - previous script incorrectly inserted {} before closing $ instead of
   removing the trailing space. Remove the {}.
2. $X $ - trailing space before closing $ causes pandoc to not recognise as math.
   Remove the trailing space.

pandoc tex_math_dollars rule: no whitespace immediately before closing $.
"""

import re
from pathlib import Path

BOOK = Path(__file__).parent

# Pattern 1: wrongly-added {} just before closing $ (from previous fix attempt)
WRONG_BRACE = re.compile(r'\$([^$\n]+?)\{\}\$')

# Pattern 2: trailing whitespace before closing $ in inline math
TRAILING_SPACE = re.compile(r'\$([^$\n]+?) +\$')

total_files = 0
wrong_brace_fixes = 0
trailing_space_fixes = 0

for md in sorted(BOOK.rglob("*.md")):
    text = md.read_text(encoding="utf-8")
    orig = text

    # Fix 1: remove wrongly-appended {}
    new, n1 = WRONG_BRACE.subn(r'$\1$', text)
    wrong_brace_fixes += n1
    text = new

    # Fix 2: remove trailing spaces inside math
    new, n2 = TRAILING_SPACE.subn(r'$\1$', text)
    trailing_space_fixes += n2
    text = new

    if text != orig:
        md.write_text(text, encoding="utf-8")
        total_files += 1

print("Modified", total_files, "files")
print("  Removed spurious {} insertions:", wrong_brace_fixes)
print("  Removed trailing spaces in math:", trailing_space_fixes)
