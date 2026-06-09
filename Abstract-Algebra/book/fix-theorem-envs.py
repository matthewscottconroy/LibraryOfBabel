#!/usr/bin/env python3
"""Fix blank lines inside LaTeX theorem environments in markdown files.

pandoc raw_tex mode terminates a raw block at a blank line, which splits
\begin{ENV} from \end{ENV} — leaving unclosed environments in the LaTeX output.

Fix: inside any \begin{THEOREM_ENV}...\end{THEOREM_ENV} block, replace every
blank line with \par so pandoc keeps the whole block as one raw chunk.

Theorem environments recognised: definition, theorem, proposition, lemma,
corollary, example, remark.
"""

import re
from pathlib import Path

BOOK = Path(__file__).parent

ENVS = {"definition", "theorem", "proposition", "lemma",
        "corollary", "example", "remark"}

ENV_OPEN  = re.compile(r'^\\begin\{(' + '|'.join(ENVS) + r')\}', re.MULTILINE)
ENV_CLOSE = re.compile(r'^\\end\{(' + '|'.join(ENVS) + r')\}', re.MULTILINE)

total_files = 0
total_fixes = 0


def fix_text(text: str) -> tuple[str, int]:
    """Return (fixed_text, number_of_blank_lines_replaced)."""
    lines = text.split('\n')
    result = []
    inside_env = 0   # nesting depth (usually 0 or 1)
    fixes = 0

    for line in lines:
        if ENV_OPEN.match(line):
            inside_env += 1
            result.append(line)
        elif ENV_CLOSE.match(line):
            inside_env = max(0, inside_env - 1)
            result.append(line)
        elif inside_env > 0 and line.strip() == '':
            # blank line inside a theorem env — replace with \par
            result.append(r'\par')
            fixes += 1
        else:
            result.append(line)

    return '\n'.join(result), fixes


for md in sorted(BOOK.rglob("*.md")):
    text = md.read_text(encoding="utf-8")
    # Quick check: skip files that have no theorem environments at all
    if not any(f'\\begin{{{e}}}' in text for e in ENVS):
        continue

    fixed, n = fix_text(text)
    if n > 0:
        md.write_text(fixed, encoding="utf-8")
        total_files += 1
        total_fixes += n

print(f"Modified {total_files} files")
print(f"  Replaced {total_fixes} blank lines inside theorem environments with \\par")
