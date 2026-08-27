#!/usr/bin/env python3
r"""Lint the book for the failure modes that have actually bitten this project.

Each check exists because the corresponding mistake was made at least once and
cost a rebuild:

  1. Unicode superscripts in prose      -> the serif font has no glyph for the
                                           letter forms, and numeric ones render
                                           inconsistently.  Use $x^{n}$.
  2. Bare backslash commands in prose   -> pandoc passes raw TeX through, so
                                           `\uD83D` in running text is executed
                                           as a control sequence and aborts.
  3. Glyphs no installed font carries   -> emoji, CJK, and the U+2717 cross.
  4. Un-closable inline math            -> a `$` the author meant as a closing
                                           delimiter but that pandoc rejects.
  5. Banned words                       -> voice-spec hard rule.
  6. Zero-byte files.
"""
import re, sys, pathlib, collections

ROOT = pathlib.Path(__file__).resolve().parent
FENCE = re.compile(r'^\s{0,3}(`{3,}|~{3,})')
SUPS  = '⁰¹²³⁴⁵⁶⁷⁸⁹⁻ⁿᵂᵏⁱ'
EXOTIC = re.compile(r'[\U0001F000-\U0001FAFF✀-➿一-鿿぀-ヿ]')
BANNED = re.compile(r'\b(simply|obviously)\b', re.I)

def blocks(text):
    """(is_prose, chunk) pairs; fenced code is not prose."""
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

def unclosed_math(chunk):
    """A '$' the author meant to close with, which pandoc will not accept."""
    hits, i, n = [], 0, len(chunk)
    while i < n:
        c = chunk[i]
        if c == '\\': i += 2; continue
        if c == '`':
            m = re.match(r'`+', chunk[i:]); t = m.group(0)
            j = chunk.find(t, i+len(t)); i = n if j == -1 else j+len(t); continue
        if chunk.startswith('$$', i):
            j = chunk.find('$$', i+2); i = n if j == -1 else j+2; continue
        if c == '$' and i+1 < n and not chunk[i+1].isspace():
            j, rejected = i+1, None
            while j < n:
                if chunk[j] == '\\': j += 2; continue
                if chunk[j] == '$':
                    if not chunk[j-1].isspace(): break
                    if rejected is None: rejected = j
                j += 1
            if rejected is not None: hits.append(chunk[i:rejected+1][:60])
            i = j+1 if j < n else n
            continue
        i += 1
    return hits

problems = collections.Counter()
detail = []
for p in sorted((ROOT/'book').rglob('*.md')):
    raw = p.read_text(encoding='utf-8')
    rel = p.relative_to(ROOT)
    if not raw.strip():
        problems['empty'] += 1; detail.append(f"EMPTY  {rel}")
    for is_prose, chunk in blocks(raw):
        if not is_prose: continue
        for ch in set(chunk) & set(SUPS):
            problems['superscript'] += 1
            detail.append(f"SUPER  {rel}: {ch!r} -- use $x^{{n}}$"); break
        stripped = re.sub(r'`[^`]*`', '', chunk)
        stripped = re.sub(r'\$\$.*?\$\$', '', stripped, flags=re.S)
        stripped = re.sub(r'\$[^$\n]*\$', '', stripped)
        for m in re.finditer(r'\\[a-zA-Z]+', stripped):
            problems['raw_tex'] += 1
            detail.append(f"TEX    {rel}: {m.group(0)!r} in prose -- wrap in backticks")
        for m in EXOTIC.finditer(chunk):
            problems['glyph'] += 1
            detail.append(f"GLYPH  {rel}: {m.group(0)!r} has no font")
        for h in unclosed_math(chunk):
            problems['math'] += 1
            detail.append(f"MATH   {rel}: {h!r}")
        # block quotes are other people's words; do not police them
        unquoted = '\n'.join(l for l in chunk.split('\n') if not l.lstrip().startswith('>'))
        for m in BANNED.finditer(unquoted):
            problems['banned'] += 1
            detail.append(f"WORD   {rel}: {m.group(0)!r}")

for line in detail[:40]: print(line)
if len(detail) > 40: print(f"... and {len(detail)-40} more")
print()
print("SUMMARY:", dict(problems) if problems else "clean")
sys.exit(1 if problems else 0)
