#!/usr/bin/env python3
"""build_pdf.py — Compile the Metaphysics project into a PDF textbook.

Usage:
    python3 build_pdf.py [output.pdf]

Requires: pandoc (>=3.0), xelatex, DejaVu Serif font.
The script walks the numbered directory tree in sorted order, shifts headings to
match the directory depth, and calls pandoc with xelatex to produce the PDF.
"""

import os
import re
import sys
import subprocess
import tempfile
from pathlib import Path

PROJECT_ROOT = Path(__file__).parent.resolve()
OUTPUT_PDF = (
    Path(sys.argv[1]).resolve() if len(sys.argv) > 1
    else PROJECT_ROOT / "Metaphysics_Textbook.pdf"
)

# Support files appear at the END of each section, after all content and subdirectories.
# Order matters: review questions first, then glossary/figures, then bibliography.
SUPPORT_FILES = {
    "review_questions.md",
    "exercises.md",
    "important_terms_and_ideas.md",
    "important_figures.md",
    "references_and_primary_sources.md",
    "further_reading.md",
}

SUPPORT_FILE_ORDER = [
    "review_questions.md",
    "exercises.md",
    "important_terms_and_ideas.md",
    "important_figures.md",
    "references_and_primary_sources.md",
    "further_reading.md",
]

CONTENT_PAT = re.compile(r'^\d{2}_.*\.md$')
HEADING_PAT = re.compile(r'^(#{1,6})(\s.*)?$')
FENCE_PAT   = re.compile(r'^(`{3,}|~{3,})')

# --top-level-division=part maps: # → \part, ## → \chapter, ### → \section,
# #### → \subsection, ##### → \subsubsection, ###### → \paragraph
YAML_HEADER = """\
---
title: "Metaphysics: A Systematic Introduction"
subtitle: "From Ontology to Metametaphysics"
date: "2026"
documentclass: book
classoption:
  - 12pt
  - openany
mainfont: "DejaVu Serif"
monofont: "DejaVu Sans Mono"
geometry:
  - paperwidth=7in
  - paperheight=10in
  - top=1in
  - bottom=1in
  - inner=1.25in
  - outer=0.75in
  - includeheadfoot
toc: true
toc-depth: 3
numbersections: true
colorlinks: true
---
"""


def clean_heading(raw: str) -> str:
    """Strip leading NN_ prefix and convert underscores to spaces."""
    s = re.sub(r'^\d+_', '', raw)
    return s.replace('_', ' ')


def shift_headings(text: str, shift: int) -> str:
    """Deepen every markdown heading in text by shift levels (max depth 6).

    Lines inside fenced code blocks are left untouched.
    """
    if shift == 0:
        return text
    lines = text.split('\n')
    out = []
    in_fence = False
    for line in lines:
        # Track opening/closing of code fences before deciding to transform.
        if FENCE_PAT.match(line.rstrip()):
            in_fence = not in_fence
        if not in_fence:
            m = HEADING_PAT.match(line)
            if m:
                new_depth = min(len(m.group(1)) + shift, 6)
                tail = m.group(2) if m.group(2) is not None else ''
                line = '#' * new_depth + tail
        out.append(line)
    return '\n'.join(out)


def strip_intro_heading(raw: str) -> str:
    """Return the body of a 00_introduction.md file with its first heading removed.

    The intro heading is redundant with the directory heading that precedes it,
    so we emit only the body text as an unnumbered preamble.
    """
    lines = raw.strip().split('\n')
    start = 0
    if lines and HEADING_PAT.match(lines[0]):
        start = 1
        while start < len(lines) and not lines[start].strip():
            start += 1
    return '\n'.join(lines[start:]).strip()


def collect(dirpath: Path, depth: int, out: list) -> None:
    """Recursively append shifted markdown content from dirpath into out.

    depth: directory levels below project root (root=0, modules=1, ...).
    The directory itself is emitted as a heading at level `depth`.
    00_introduction.md is emitted as unnumbered preamble body text immediately
    after the directory heading. All other content files are emitted with
    headings shifted by `depth` so that each file's top-level '#' lands one
    level below its parent directory.
    """
    try:
        entries = sorted(dirpath.iterdir(), key=lambda p: p.name)
    except PermissionError as e:
        print(f"  Warning: {e}", file=sys.stderr)
        return

    file_map     = {e.name: e for e in entries if e.is_file()}
    content_files = [
        e for e in entries
        if e.is_file() and CONTENT_PAT.match(e.name) and e.name != '00_introduction.md'
    ]
    support_files = [
        file_map[name] for name in SUPPORT_FILE_ORDER
        if name in file_map
    ]
    subdirs = [e for e in entries if e.is_dir() and not e.name.startswith('.')]

    # Inject the directory heading (project root has no heading — it's the book title).
    if depth > 0:
        hashes = '#' * min(depth, 6)
        out.append(f"{hashes} {clean_heading(dirpath.name)}")
        out.append('')

    # 00_introduction.md is emitted as unnumbered preamble body text.
    intro_path = dirpath / '00_introduction.md'
    if intro_path.exists():
        try:
            raw = intro_path.read_text(encoding='utf-8')
        except OSError as e:
            print(f"  Warning: cannot read {intro_path}: {e}", file=sys.stderr)
            raw = ''
        body = strip_intro_heading(raw)
        if body:
            out.append(body)
            out.append('')

    # Files shift by `depth`: their '#' becomes '#'*(depth+1), one level below the dir.
    shift = depth

    for fpath in content_files:
        try:
            raw = fpath.read_text(encoding='utf-8')
        except OSError as e:
            print(f"  Warning: cannot read {fpath}: {e}", file=sys.stderr)
            continue
        out.append(shift_headings(raw.rstrip(), shift))
        out.append('')

    # Subdirectories before support files so that all section content comes first.
    for subdir in subdirs:
        collect(subdir, depth + 1, out)

    # Support files at the very end of the section (review questions, glossary, refs).
    for fpath in support_files:
        try:
            raw = fpath.read_text(encoding='utf-8')
        except OSError:
            continue
        out.append(shift_headings(raw.rstrip(), shift))
        out.append('')


def main() -> None:
    module_dirs = sorted(
        d for d in PROJECT_ROOT.iterdir()
        if d.is_dir() and re.match(r'^\d{2}_', d.name)
    )

    if not module_dirs:
        sys.exit(f"No numbered module directories found in {PROJECT_ROOT}")

    print(f"Project root  : {PROJECT_ROOT}")
    print(f"Modules found : {len(module_dirs)}")
    print(f"Output        : {OUTPUT_PDF}")
    print()

    chunks = [YAML_HEADER]

    for mod in module_dirs:
        print(f"  Collecting {mod.name} …")
        collect(mod, depth=1, out=chunks)
        chunks.append('')

    combined = '\n'.join(chunks)

    with tempfile.NamedTemporaryFile(
        mode='w', suffix='.md', encoding='utf-8',
        delete=False, prefix='metaphysics_'
    ) as tmp:
        tmp.write(combined)
        tmp_path = tmp.name

    char_count = len(combined)
    print(f"\nCombined source : {tmp_path}")
    print(f"Total characters: {char_count:,}")
    print("Running pandoc  …")

    cmd = [
        'pandoc', tmp_path,
        '--pdf-engine=xelatex',
        '--top-level-division=part',
        '--output', str(OUTPUT_PDF),
    ]

    try:
        proc = subprocess.run(cmd, capture_output=True, text=True, timeout=900)
    except subprocess.TimeoutExpired:
        os.unlink(tmp_path)
        sys.exit("pandoc timed out after 15 minutes")

    if proc.returncode != 0:
        print("\npandoc FAILED. Last 80 lines of stderr:\n", file=sys.stderr)
        for line in proc.stderr.splitlines()[-80:]:
            print(' ', line, file=sys.stderr)
        print(f"\nCombined source left at: {tmp_path}", file=sys.stderr)
        sys.exit(1)

    os.unlink(tmp_path)

    size_mb = OUTPUT_PDF.stat().st_size / 1_048_576
    print(f"\nDone! {OUTPUT_PDF}  ({size_mb:.1f} MB)")


if __name__ == '__main__':
    main()
