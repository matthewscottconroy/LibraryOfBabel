#!/usr/bin/env python3
"""Auto-fix the mechanical issues tools-lint.py reports.

Currently: convert Unicode superscripts in prose to LaTeX math, leaving code
fences alone.  Run this after drafting, then run tools-lint.py to confirm.
"""
import re, pathlib, sys

ROOT = pathlib.Path(__file__).resolve().parent
FENCE = re.compile(r'^\s{0,3}(`{3,}|~{3,})')
SUP = {'⁰':'0','¹':'1','²':'2','³':'3','⁴':'4','⁵':'5','⁶':'6','⁷':'7','⁸':'8',
       '⁹':'9','⁻':'-','ⁿ':'n','ᵂ':'W','ᵏ':'k','ⁱ':'i'}
PAT = re.compile(rf'([0-9]+|[A-Za-z])([{"".join(SUP)}]+)')

def blocks(text):
    out, buf, fence = [], [], None
    for line in text.split('\n'):
        m = FENCE.match(line)
        if m:
            ch = m.group(1)[0]
            if fence is None:
                if buf: out.append((True, '\n'.join(buf))); buf = []
                fence = ch
            elif ch == fence:
                fence = None
            out.append((False, line)); continue
        if fence is not None: out.append((False, line)); continue
        buf.append(line)
    if buf: out.append((True, '\n'.join(buf)))
    return out

total = files = 0
for p in sorted((ROOT/'book').rglob('*.md')):
    s = p.read_text(encoding='utf-8')
    out, n = [], 0
    for is_prose, chunk in blocks(s):
        if is_prose:
            new, c = PAT.subn(
                lambda m: f"${m.group(1)}^{{{''.join(SUP[c] for c in m.group(2))}}}$",
                chunk)
            n += c; out.append(new)
        else:
            out.append(chunk)
    joined = '\n'.join(out)
    if joined != s:
        p.write_text(joined, encoding='utf-8'); files += 1; total += n
print(f"converted {total} exponents in {files} files")
