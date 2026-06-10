#!/usr/bin/env python3
"""
generate_textbook.py — assemble the Philosophy of Time textbook into a PDF.

Directory hierarchy:
  unit-XX-name/
    intro.md                              → heading shift 0  (Part level)
    chapter-XX-name/
      intro.md                            → heading shift +1 (Chapter level)
      section-XX-name/
        intro.md                          → heading shift +2 (Section level)
        1-name.md, 2-name.md, ...         → heading shift +3 (Subsection level)
      exercises.md                        → heading shift +2
      further-reading.md                  → heading shift +2
      important-concepts.md               → heading shift +2
      important-researchers.md            → heading shift +2

Heading shifts prepend '#' characters to every ATX heading line so that
pandoc's --top-level-division=part sees the correct hierarchy:
  shift 0  → # = Part,   ## = Chapter, ...
  shift +1 → ## = Chapter, ### = Section, ...
  etc.
"""

import os
import re
import sys
import shutil
import subprocess
import textwrap
from pathlib import Path

# ---------------------------------------------------------------------------
# Configuration
# ---------------------------------------------------------------------------

BASE_DIR = Path(__file__).parent.resolve()
OUTPUT_DIR = BASE_DIR / "output"
COMBINED_MD = OUTPUT_DIR / "philosophy_of_time_combined.md"
OUTPUT_PDF = OUTPUT_DIR / "philosophy_of_time.pdf"

YAML_FRONT_MATTER = textwrap.dedent("""\
    ---
    title: "The Philosophy of Time: A Comprehensive Textbook"
    author: "Philosophy of Time Project"
    date: "2026"
    documentclass: book
    classoption:
      - openany
    geometry:
      - top=1in
      - bottom=1in
      - left=1.25in
      - right=1.25in
    fontsize: 11pt
    linestretch: 1.25
    toc: true
    toc-depth: 3
    number-sections: true
    numbersections: true
    colorlinks: true
    linkcolor: black
    urlcolor: NavyBlue
    citecolor: black
    mainfont: "Noto Serif"
    monofont: "DejaVu Sans Mono"
    header-includes:
      - \\usepackage{fontspec}
      - \\usepackage{fancyhdr}
      - \\pagestyle{fancy}
      - \\fancyhf{}
      - \\fancyhead[LE,RO]{\\thepage}
      - \\fancyhead[RE]{\\leftmark}
      - \\fancyhead[LO]{\\rightmark}
      - \\renewcommand{\\headrulewidth}{0.4pt}
      - \\usepackage{microtype}
      - \\usepackage{setspace}
    ---

""")

# Chapter-end files, in display order
CHAPTER_END_FILES = [
    "exercises.md",
    "further-reading.md",
    "important-concepts.md",
    "important-researchers.md",
]

# Regex matching ATX headings: optional leading whitespace, 1-6 '#', space, text
ATX_HEADING_RE = re.compile(r"^(#{1,6})(\s+.*)$", re.MULTILINE)


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def check_dependencies():
    """Verify pandoc and a LaTeX engine are available."""
    errors = []

    pandoc = shutil.which("pandoc")
    if not pandoc:
        errors.append("pandoc not found — install pandoc >= 2.0")
    else:
        result = subprocess.run(
            ["pandoc", "--version"], capture_output=True, text=True
        )
        version_line = result.stdout.splitlines()[0] if result.stdout else "unknown"
        print(f"  pandoc:  {version_line}")

    latex_engine = None
    for engine in ("xelatex", "lualatex", "pdflatex"):
        path = shutil.which(engine)
        if path:
            latex_engine = engine
            print(f"  LaTeX:   {engine} ({path})")
            break

    if not latex_engine:
        errors.append(
            "No LaTeX engine found — install TeX Live or MiKTeX "
            "(xelatex, lualatex, or pdflatex)"
        )

    if errors:
        for e in errors:
            print(f"ERROR: {e}", file=sys.stderr)
        sys.exit(1)

    return latex_engine


def shift_headings(text: str, shift: int) -> str:
    """Add `shift` '#' characters to every ATX heading in text."""
    if shift == 0:
        return text
    prefix = "#" * shift

    def replacer(m):
        return prefix + m.group(1) + m.group(2)

    return ATX_HEADING_RE.sub(replacer, text)


def read_and_shift(path: Path, shift: int) -> str:
    """Read a markdown file, shift its headings, and return the result."""
    try:
        text = path.read_text(encoding="utf-8")
    except OSError as e:
        print(f"  WARNING: could not read {path}: {e}", file=sys.stderr)
        return ""

    # Strip YAML front matter if present (--- ... ---) so it doesn't appear
    # mid-document.
    if text.startswith("---"):
        end = text.find("\n---", 3)
        if end != -1:
            text = text[end + 4:]

    text = shift_headings(text.strip(), shift)
    return text


def section_sort_key(name: str) -> tuple:
    """
    Sort key for section/subsection directory or file names.

    Handles patterns:
      section-01-name    → (1, 'name')
      chapter-03-name    → (3, 'name')
      1-name.md          → (1, 'name')
      intro.md           → (-1, '')     ← sorts before numbered items
      exercises.md etc.  → (999, ...)   ← sorts after content
    """
    # intro always first
    if name in ("intro.md", "intro"):
        return (-1, "")

    # chapter-end files — fixed order
    if name in CHAPTER_END_FILES:
        return (999, str(CHAPTER_END_FILES.index(name)))

    # section-NN-name  or  chapter-NN-name
    m = re.match(r"(?:section|chapter)-(\d+)", name)
    if m:
        return (int(m.group(1)), name)

    # NN-name.md  (subsection files)
    m = re.match(r"(\d+)-", name)
    if m:
        return (int(m.group(1)), name)

    return (500, name)


def collect_files(base: Path) -> list[tuple[Path, int]]:
    """
    Walk the directory tree and return an ordered list of (path, heading_shift)
    tuples representing every markdown file to include.

    Hierarchy and shifts:
      unit intro.md         shift 0
      chapter intro.md      shift 1
      section intro.md      shift 2
      subsection *.md       shift 3
      chapter-end files     shift 2
    """
    entries: list[tuple[Path, int]] = []

    unit_dirs = sorted(
        [d for d in base.iterdir() if d.is_dir() and d.name.startswith("unit-")],
        key=lambda d: section_sort_key(d.name),
    )

    for unit_dir in unit_dirs:
        unit_intro = unit_dir / "intro.md"
        if unit_intro.exists():
            entries.append((unit_intro, 0))

        chapter_dirs = sorted(
            [
                d
                for d in unit_dir.iterdir()
                if d.is_dir() and d.name.startswith("chapter-")
            ],
            key=lambda d: section_sort_key(d.name),
        )

        for chapter_dir in chapter_dirs:
            chapter_intro = chapter_dir / "intro.md"
            if chapter_intro.exists():
                entries.append((chapter_intro, 1))

            section_dirs = sorted(
                [
                    d
                    for d in chapter_dir.iterdir()
                    if d.is_dir() and d.name.startswith("section-")
                ],
                key=lambda d: section_sort_key(d.name),
            )

            for section_dir in section_dirs:
                section_intro = section_dir / "intro.md"
                if section_intro.exists():
                    entries.append((section_intro, 2))

                subsection_files = sorted(
                    [
                        f
                        for f in section_dir.iterdir()
                        if f.is_file()
                        and f.suffix == ".md"
                        and f.name != "intro.md"
                    ],
                    key=lambda f: section_sort_key(f.name),
                )
                for sf in subsection_files:
                    entries.append((sf, 3))

            # Chapter-end files (in fixed order)
            for cef in CHAPTER_END_FILES:
                cef_path = chapter_dir / cef
                if cef_path.exists():
                    entries.append((cef_path, 2))

    return entries


def build_combined_markdown(entries: list[tuple[Path, int]]) -> None:
    """Write all shifted markdown content into a single combined file."""
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

    with COMBINED_MD.open("w", encoding="utf-8") as out:
        out.write(YAML_FRONT_MATTER)

        for i, (path, shift) in enumerate(entries):
            content = read_and_shift(path, shift)
            if not content:
                continue
            # Ensure each section is separated by a blank line
            out.write(content)
            out.write("\n\n")

            if (i + 1) % 50 == 0:
                print(f"  assembled {i + 1}/{len(entries)} files...")

    size_kb = COMBINED_MD.stat().st_size / 1024
    print(f"  combined markdown: {COMBINED_MD} ({size_kb:.0f} KB)")


def run_pandoc(latex_engine: str) -> None:
    """Invoke pandoc to convert the combined markdown to PDF."""
    cmd = [
        "pandoc",
        str(COMBINED_MD),
        "--pdf-engine", latex_engine,
        "--top-level-division=part",
        "--toc",
        "--toc-depth=3",
        "--number-sections",
        "--highlight-style=tango",
        "--variable", "geometry:margin=1in",
        "-o", str(OUTPUT_PDF),
    ]

    print(f"\n  running: {' '.join(cmd[:4])} ... -o {OUTPUT_PDF}")
    print("  (this may take several minutes for a large document)\n")

    result = subprocess.run(cmd, capture_output=True, text=True)

    if result.returncode != 0:
        print("pandoc FAILED:", file=sys.stderr)
        print(result.stderr, file=sys.stderr)
        sys.exit(1)

    if result.stderr:
        # pandoc often emits warnings on stderr even on success
        for line in result.stderr.splitlines():
            print(f"  pandoc: {line}")

    size_mb = OUTPUT_PDF.stat().st_size / (1024 * 1024)
    print(f"\n  PDF written: {OUTPUT_PDF} ({size_mb:.1f} MB)")


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    print("=" * 60)
    print("Philosophy of Time — PDF Generator")
    print("=" * 60)

    print("\n[1/4] Checking dependencies...")
    latex_engine = check_dependencies()

    print("\n[2/4] Collecting files...")
    entries = collect_files(BASE_DIR)
    print(f"  {len(entries)} markdown files found")

    if not entries:
        print("ERROR: no markdown files found under", BASE_DIR, file=sys.stderr)
        sys.exit(1)

    print("\n[3/4] Assembling combined markdown...")
    build_combined_markdown(entries)

    print("\n[4/4] Rendering PDF with pandoc + " + latex_engine + "...")
    run_pandoc(latex_engine)

    print("\nDone.")
    print(f"  Output: {OUTPUT_PDF}")


if __name__ == "__main__":
    main()
