#!/usr/bin/env python3
r"""Lint the book against its own outline, and against the failure modes that
have actually cost a rebuild.

The outline (programs_and_machines_outline.md) is the structural contract. This
script parses it and checks the tree matches, so that the contract is enforced
rather than merely described. Everything else here exists because the
corresponding mistake was made at least once:

  STRUCTURE  the tree disagrees with the outline (missing or stray files)
  HEADING    a unit intro or chapter overview carries a `##` heading. These
             render at the same level as section titles, so the table of
             contents numbers "What is here" as a peer of "The Array". Fixed
             once by hand, then reintroduced in fourteen files -- hence this
             check.
  BACKMATTER a written chapter is missing one of its four back-matter files
  CARRIES    an exercise set marks nothing [carries forward]
  CHAPTERREF a "Chapter N" reference points outside 1..35
  SUPER      Unicode superscripts in prose; the serif font has no glyph for the
             letter forms. Use $x^{n}$.
  TEX        a bare backslash command in prose; pandoc passes raw TeX through,
             so `\uD83D` in running text is executed and aborts the build
  GLYPH      emoji, CJK, or U+2717 -- no installed font carries them
  MATH       a `$` the author meant as a closing delimiter but that pandoc
             rejects, silently swallowing the prose that follows
  WORD       voice-spec banned words (block quotes are exempt: other people's
             words are not ours to police)

Usage:  python3 tools-lint.py [--quiet]
"""
import re, sys, pathlib, collections

ROOT     = pathlib.Path(__file__).resolve().parent
BOOK     = ROOT / 'book'
OUTLINE  = ROOT / 'programs_and_machines_outline.md'
FENCE    = re.compile(r'^\s{0,3}(`{3,}|~{3,})')
SUPS     = '⁰¹²³⁴⁵⁶⁷⁸⁹⁻ⁿᵂᵏⁱ'
EXOTIC   = re.compile(r'[\U0001F000-\U0001FAFF✀-➿一-鿿぀-ヿ]')
BANNED   = re.compile(r'\b(simply|obviously)\b', re.I)
BACKMATTER = ('exercises.md', 'further-reading.md',
              'important-concepts.md', 'important-researchers.md')


# --------------------------------------------------------------------------- #
# The outline is the contract
# --------------------------------------------------------------------------- #

def parse_outline():
    units, unit, chap, sec = [], None, None, None
    for line in OUTLINE.read_text(encoding='utf-8').split('\n'):
        if m := re.match(r'^## Unit ([IVX]+) — (.+)$', line):
            unit = {'roman': m.group(1), 'slug': None, 'chapters': []}
            units.append(unit); chap = None
        elif (m := re.match(r'^`book/([^/]+)/intro\.md`$', line)) and unit:
            unit['slug'] = m.group(1)
        elif (m := re.match(r'^### Chapter (\d+)\. (.+)$', line)) and unit:
            chap = {'num': int(m.group(1)), 'slug': None, 'sections': []}
            unit['chapters'].append(chap); sec = None
        elif (m := re.match(r'^`([^`]+)/README\.md` — chapter overview$', line)) and chap:
            chap['slug'] = m.group(1)
        elif (m := re.match(r'^- \*\*(.+?)\*\* — `([^`]+)/README\.md`$', line)) and chap:
            sec = {'slug': m.group(2), 'files': []}
            chap['sections'].append(sec)
        elif (m := re.match(r'^  - `([^`]+\.md)` — ', line)) and sec:
            sec['files'].append(m.group(1))
    return units


def expected_paths(units):
    """Every file the outline promises, and the chapters written so far."""
    want, written = set(), []
    for u in units:
        # a unit with no written chapters is not yet started; do not demand its intro
        if any((BOOK / u['slug'] / c['slug'] / 'README.md').exists() for c in u['chapters']):
            want.add(BOOK / u['slug'] / 'intro.md')
        for c in u['chapters']:
            base = BOOK / u['slug'] / c['slug']
            if not (base / 'README.md').exists():
                continue                       # chapter not written yet
            written.append((c['num'], base))
            want.add(base / 'README.md')
            for bm in BACKMATTER:
                want.add(base / bm)
            for s in c['sections']:
                want.add(base / s['slug'] / 'README.md')
                for f in s['files']:
                    want.add(base / s['slug'] / f)
    # appendices, listed in the outline's own Appendices section
    apx = re.findall(r'^- `(appendices/[^`]+\.md)`', OUTLINE.read_text(encoding='utf-8'), re.M)
    for rel in apx:
        want.add(BOOK / rel)
    return want, written


# --------------------------------------------------------------------------- #
# Prose scanning
# --------------------------------------------------------------------------- #

def blocks(text):
    """(is_prose, chunk) pairs; fenced code is not prose."""
    out, buf, fence = [], [], None
    for line in text.split('\n'):
        if m := FENCE.match(line):
            ch = m.group(1)[0]
            if fence is None:
                if buf: out.append((True, '\n'.join(buf))); buf = []
                fence = ch
            elif ch == fence:
                fence = None
            out.append((False, line)); continue
        if fence is not None:
            out.append((False, line)); continue
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
            t = re.match(r'`+', chunk[i:]).group(0)
            j = chunk.find(t, i + len(t))
            i = n if j == -1 else j + len(t); continue
        if chunk.startswith('$$', i):
            j = chunk.find('$$', i + 2)
            i = n if j == -1 else j + 2; continue
        if c == '$' and i + 1 < n and not chunk[i + 1].isspace():
            j, rejected = i + 1, None
            while j < n:
                if chunk[j] == '\\': j += 2; continue
                if chunk[j] == '$':
                    if not chunk[j - 1].isspace(): break
                    if rejected is None: rejected = j
                j += 1
            if rejected is not None: hits.append(chunk[i:rejected + 1][:60])
            i = j + 1 if j < n else n
            continue
        i += 1
    return hits


# --------------------------------------------------------------------------- #

def main():
    quiet = '--quiet' in sys.argv
    units = parse_outline()
    if not units:
        print("FATAL: could not parse the outline"); return 2

    problems, detail = collections.Counter(), []
    def note(kind, msg):
        problems[kind] += 1; detail.append(f"{kind:<10} {msg}")

    # --- structure against the outline --------------------------------------
    want, written = expected_paths(units)
    have = {p for p in BOOK.rglob('*.md')}
    for p in sorted(want - have):
        note('STRUCTURE', f"{p.relative_to(ROOT)} promised by the outline, missing")
    written_bases = {b for _, b in written}
    for p in sorted(have - want):
        # a stray file inside a written chapter is a real finding
        if any(str(p).startswith(str(b)) for b in written_bases) \
           or p.parent == BOOK / 'appendices':
            note('STRUCTURE', f"{p.relative_to(ROOT)} present but not in the outline")

    # --- back matter --------------------------------------------------------
    for num, base in written:
        for bm in BACKMATTER:
            if not (base / bm).exists():
                note('BACKMATTER', f"chapter {num} has no {bm}")
        ex = base / 'exercises.md'
        if ex.exists() and 'carries forward' not in ex.read_text(encoding='utf-8'):
            note('CARRIES', f"chapter {num} marks no exercise [carries forward]")

    # --- the heading convention --------------------------------------------
    overviews = [BOOK / u['slug'] / 'intro.md' for u in units]
    overviews += [b / 'README.md' for _, b in written]
    for p in overviews:
        if not p.exists(): continue
        for line in p.read_text(encoding='utf-8').split('\n'):
            if line.startswith('## '):
                note('HEADING', f"{p.relative_to(ROOT)}: '{line[3:][:40]}' "
                                f"-- overview files carry no ## headings")

    # --- prose --------------------------------------------------------------
    maxch = max(c['num'] for u in units for c in u['chapters'])
    for p in sorted(have):
        raw = p.read_text(encoding='utf-8')
        rel = p.relative_to(ROOT)
        if not raw.strip():
            note('STRUCTURE', f"{rel} is empty")
        for n in (int(x) for x in re.findall(r'Chapter (\d+)', raw)):
            if not 1 <= n <= maxch:
                note('CHAPTERREF', f"{rel}: Chapter {n} is outside 1..{maxch}")
        for is_prose, chunk in blocks(raw):
            if not is_prose: continue
            if set(chunk) & set(SUPS):
                note('SUPER', f"{rel}: Unicode superscript -- use $x^{{n}}$")
            stripped = re.sub(r'`[^`]*`', '', chunk)
            stripped = re.sub(r'\$\$.*?\$\$', '', stripped, flags=re.S)
            stripped = re.sub(r'\$[^$\n]*\$', '', stripped)
            for m in re.finditer(r'\\[a-zA-Z]+', stripped):
                note('TEX', f"{rel}: {m.group(0)!r} in prose -- wrap in backticks")
            for m in EXOTIC.finditer(chunk):
                note('GLYPH', f"{rel}: {m.group(0)!r} has no font")
            for h in unclosed_math(chunk):
                note('MATH', f"{rel}: {h!r}")
            unquoted = '\n'.join(l for l in chunk.split('\n')
                                 if not l.lstrip().startswith('>'))
            for m in BANNED.finditer(unquoted):
                note('WORD', f"{rel}: {m.group(0)!r}")

    # --- question bank ------------------------------------------------------
    import json
    qroot = ROOT / 'questions'
    if qroot.exists():
        for num, _ in written:
            d = qroot / f'ch{num - 1:02d}'          # subject.toml index is 0-based
            files = sorted(d.glob('*.json')) if d.exists() else []
            if not files:
                note('QUESTIONS', f"chapter {num} has no questions ({d.name}/ is empty)")
                continue
            idxs = []
            for f in files:
                try:
                    q = json.loads(f.read_text(encoding='utf-8'))
                except Exception as exc:
                    note('QUESTIONS', f"{f.name}: invalid JSON ({exc})"); continue
                if q.get('chapter') != num - 1:
                    note('QUESTIONS', f"{d.name}/{f.name}: chapter is {q.get('chapter')}, expected {num - 1}")
                kind = q.get('kind')
                if kind == 'blank':
                    if q.get('answer') != q.get('choices', [None])[0]:
                        note('QUESTIONS', f"{d.name}/{f.name}: blank answer must equal choices[0]")
                    if '___' not in q.get('text', ''):
                        note('QUESTIONS', f"{d.name}/{f.name}: blank question has no ___ in its text")
                elif kind in ('mc', 'tf'):
                    a = q.get('answer')
                    if not isinstance(a, int) or not 0 <= a < len(q.get('choices', [])):
                        note('QUESTIONS', f"{d.name}/{f.name}: answer {a!r} is not a valid index")
                    else:
                        idxs.append(a)
                else:
                    note('QUESTIONS', f"{d.name}/{f.name}: bad kind {kind!r}")
            if idxs and len(set(idxs)) == 1:
                note('QUESTIONS', f"{d.name}: every correct answer is at index {idxs[0]}")

    if not quiet:
        for line in detail[:50]: print(line)
        if len(detail) > 50: print(f"... and {len(detail) - 50} more")
        print()
    ch = len(written)
    print(f"checked {len(have)} files, {ch}/{sum(len(u['chapters']) for u in units)} chapters written")
    print("SUMMARY:", dict(problems) if problems else "clean")
    return 1 if problems else 0


if __name__ == '__main__':
    sys.exit(main())
