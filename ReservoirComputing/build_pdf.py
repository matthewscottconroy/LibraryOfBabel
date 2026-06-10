#!/usr/bin/env python3
"""
build_pdf.py — Convert the Reservoir Computing textbook to PDF (or HTML).

Requires:
  - pandoc >= 2.14      https://pandoc.org
  - TeX Live / MiKTeX   for PDF output (xelatex preferred, pdflatex fallback)

Usage:
    python build_pdf.py                       # Full textbook → PDF
    python build_pdf.py --output PATH.pdf     # Custom output path
    python build_pdf.py --html                # HTML (no LaTeX needed)
    python build_pdf.py --markdown            # Dump combined markdown only
    python build_pdf.py --chapters 1-5        # Only chapters 1-5 (fast testing)
    python build_pdf.py --check               # Check deps + list file order
    python build_pdf.py --verbose             # Show full pandoc/LaTeX output

The script assembles 400+ individual section files into a single document with:
  - Parts  (Units I–X, Appendices)       → LaTeX \\part
  - Chapters                              → LaTeX \\chapter
  - Sections (section directories)       → LaTeX \\section
  - Subsections (headings within files)  → LaTeX \\subsection / \\subsubsection
"""

import argparse
import os
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

# ---------------------------------------------------------------------------
# Paths
# ---------------------------------------------------------------------------

SCRIPT_DIR = Path(__file__).parent
BOOK_DIR = SCRIPT_DIR / "book"
DEFAULT_OUTPUT = SCRIPT_DIR / "reservoir_computing_textbook.pdf"

# ---------------------------------------------------------------------------
# LaTeX / Pandoc metadata
# ---------------------------------------------------------------------------

YAML_METADATA = r"""---
title: "Reservoir Computing"
subtitle: "From First Principles to the Frontier"
date: "2026"
documentclass: book
classoption:
  - 12pt
  - oneside
  - openany
geometry:
  - top=1.0in
  - bottom=1.1in
  - left=1.25in
  - right=1.0in
fontsize: 12pt
colorlinks: true
linkcolor: NavyBlue
urlcolor: NavyBlue
citecolor: NavyBlue
toc: true
toc-depth: 2
numbersections: true
secnumdepth: 2
---

"""

# LaTeX preamble written to a temp file and passed with -H.
# Using a separate file avoids pandoc mis-parsing backslashes in YAML.
LATEX_HEADER = r"""
% Unicode and font setup (works for both pdflatex and xelatex)
\usepackage{iftex}
\ifPDFTeX
  \usepackage[utf8]{inputenc}
  \usepackage[T1]{fontenc}
  \usepackage{lmodern}
\else
  % xelatex / lualatex: use fontspec with Latin Modern
  \usepackage{fontspec}
  \setmainfont{Latin Modern Roman}
  \setmonofont{Latin Modern Mono}
\fi

% Mathematics
\usepackage{amsmath,amssymb,amsthm}
\usepackage{bm}

% Tables and figures
\usepackage{booktabs}
\usepackage{longtable}
\usepackage{array}
\usepackage{float}

% Typography
\usepackage{microtype}
\usepackage[dvipsnames]{xcolor}
\setlength{\headheight}{14pt}

% Headers and footers
\usepackage{fancyhdr}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[LE]{\small\nouppercase{\leftmark}}
\fancyhead[RO]{\small\nouppercase{\rightmark}}
\fancyfoot[C]{\thepage}
\renewcommand{\headrulewidth}{0.4pt}
\fancypagestyle{plain}{%
  \fancyhf{}%
  \fancyfoot[C]{\thepage}%
  \renewcommand{\headrulewidth}{0pt}%
}

% Chapter title formatting
\usepackage[explicit]{titlesec}
\titleformat{\chapter}[display]
  {\normalfont\huge\bfseries}
  {\chaptertitlename\ \thechapter}{16pt}{\Huge #1}
\titlespacing*{\chapter}{0pt}{30pt}{20pt}

% Paragraph spacing
\setlength{\parskip}{0.5em}
\setlength{\parindent}{0pt}

% Theorem environments
\theoremstyle{plain}
\newtheorem{theorem}{Theorem}[chapter]
\newtheorem{lemma}[theorem]{Lemma}
\newtheorem{corollary}[theorem]{Corollary}
\newtheorem{proposition}[theorem]{Proposition}
\theoremstyle{definition}
\newtheorem{definition}[theorem]{Definition}
\newtheorem{example}[theorem]{Example}
\theoremstyle{remark}
\newtheorem{remark}[theorem]{Remark}
"""

# ---------------------------------------------------------------------------
# File ordering
# ---------------------------------------------------------------------------

# Files at the chapter level are included after all sections, in this order.
CHAPTER_BACKMATTER_ORDER = [
    "exercises.md",
    "key_concepts.md",
    "key_researchers.md",
    "further_reading.md",
]


def collect_files(book_dir: Path, chapter_range=None):
    """
    Walk the book directory and return an ordered list of
    (path, heading_shift) pairs.

    heading_shift is the number of '#' characters to prepend to every heading
    in the file so that the whole document, compiled with pandoc's
    --top-level-division=part, maps onto the LaTeX hierarchy:

        H1 (#)     → \\part
        H2 (##)    → \\chapter
        H3 (###)   → \\section
        H4 (####)  → \\subsection
        H5 (#####) → \\subsubsection

    Shift assignments:
        preface.md                        +1  (H1 → \\chapter)
        unit_XX/introduction.md            0  (H1 → \\part)
        unit_XX/chapter_YY/intro.md       +1  (H1 → \\chapter)
        unit_XX/chapter_YY/section_ZZ/*  +2  (H1 → \\section, H2 → \\subsection)
        chapter-level back-matter         +2  (H1 "# Exercises" → \\section)
        appendices/appendix_X/README.md  +1  (H1 → \\chapter)
    """
    files = []
    chapter_idx = 0  # 1-based chapter counter for range filtering

    def in_range(n):
        if chapter_range is None:
            return True
        lo, hi = chapter_range
        return lo <= n <= hi

    # --- Preface ---
    preface = book_dir / "preface.md"
    if preface.exists():
        files.append((preface, 1))

    # --- Units ---
    for unit_dir in sorted(
        d for d in book_dir.iterdir()
        if d.is_dir() and re.match(r"unit_\d+", d.name)
    ):
        unit_intro = unit_dir / "introduction.md"
        if unit_intro.exists():
            files.append((unit_intro, 0))

        for chapter_dir in sorted(
            d for d in unit_dir.iterdir()
            if d.is_dir() and re.match(r"chapter_\d+", d.name)
        ):
            chapter_idx += 1
            if not in_range(chapter_idx):
                continue

            ch_intro = chapter_dir / "introduction.md"
            if ch_intro.exists():
                files.append((ch_intro, 1))

            for section_dir in sorted(
                d for d in chapter_dir.iterdir()
                if d.is_dir() and re.match(r"section_\d+", d.name)
            ):
                sec_intro = section_dir / "introduction.md"
                if sec_intro.exists():
                    files.append((sec_intro, 2))

                for f in sorted(
                    f for f in section_dir.iterdir()
                    if f.is_file() and f.suffix == ".md" and f.name != "introduction.md"
                ):
                    files.append((f, 2))

            for fname in CHAPTER_BACKMATTER_ORDER:
                f = chapter_dir / fname
                if f.exists():
                    files.append((f, 2))

    # --- Appendices ---
    app_dir = book_dir / "appendices"
    if app_dir.exists() and any(True for _ in app_dir.iterdir()):
        files.append(None)  # sentinel: inject \part{Appendices}
        for sub in sorted(
            d for d in app_dir.iterdir()
            if d.is_dir() and re.match(r"appendix_", d.name)
        ):
            readme = sub / "README.md"
            if readme.exists():
                files.append((readme, 1))

    return files


# ---------------------------------------------------------------------------
# Markdown transformation
# ---------------------------------------------------------------------------

HEADING_RE = re.compile(r"^(#{1,6})([ \t])", re.MULTILINE)


def shift_headings(text: str, shift: int) -> str:
    """Add `shift` extra '#' to every ATX heading, capping at H6."""
    if shift == 0:
        return text

    def _replace(m):
        new_level = min(len(m.group(1)) + shift, 6)
        return "#" * new_level + m.group(2)

    return HEADING_RE.sub(_replace, text)


# Unicode → LaTeX replacement map for characters that pdflatex/T1 can't handle
# Replacements that are safe both inside and outside math mode use text commands;
# symbols only valid in math are wrapped in $...$.
_UNICODE_REPLACEMENTS = [
    ("✓", r"$\checkmark$"),   # ✓ CHECK MARK
    ("✔", r"$\checkmark$"),   # ✔ HEAVY CHECK MARK
    ("✗", r"$\times$"),       # ✗ BALLOT X
    ("✘", r"$\times$"),       # ✘ HEAVY BALLOT X
    ("★", r"$\bigstar$"),     # ★ BLACK STAR
    ("☆", r"$\star$"),        # ☆ WHITE STAR
    ("×", r"$\times$"),       # × MULTIPLICATION SIGN
    ("±", r"$\pm$"),          # ± PLUS-MINUS
    ("²", r"$^{2}$"),         # ² SUPERSCRIPT TWO
    ("³", r"$^{3}$"),         # ³ SUPERSCRIPT THREE
    ("°", r"$^{\circ}$"),     # ° DEGREE SIGN
    ("≈", r"$\approx$"),      # ≈ ALMOST EQUAL TO
    ("≠", r"$\neq$"),         # ≠ NOT EQUAL TO
    ("≤", r"$\leq$"),         # ≤ LESS-THAN OR EQUAL
    ("≥", r"$\geq$"),         # ≥ GREATER-THAN OR EQUAL
    ("∞", r"$\infty$"),       # ∞ INFINITY
    ("→", r"$\to$"),          # → RIGHTWARDS ARROW
    ("←", r"$\leftarrow$"),   # ← LEFTWARDS ARROW
    ("↔", r"$\leftrightarrow$"),  # ↔ LEFT RIGHT ARROW
    ("⇒", r"$\Rightarrow$"),  # ⇒
    ("⇐", r"$\Leftarrow$"),   # ⇐
    ("⇔", r"$\Leftrightarrow$"),  # ⇔
    ("∈", r"$\in$"),          # ∈ ELEMENT OF
    ("∉", r"$\notin$"),       # ∉
    ("∑", r"$\sum$"),         # ∑ N-ARY SUMMATION
    ("∏", r"$\prod$"),        # ∏ N-ARY PRODUCT
    ("∂", r"$\partial$"),     # ∂ PARTIAL DIFFERENTIAL
    ("∇", r"$\nabla$"),       # ∇ NABLA
    ("·", r"$\cdot$"),        # · MIDDLE DOT
    ("…", r"\\ldots"),        # … HORIZONTAL ELLIPSIS
]


def clean_content(text: str) -> str:
    """Normalise content for clean pandoc parsing."""
    # Ensure blank lines before and after fenced code blocks
    text = re.sub(r"(?<!\n)\n(```)", r"\n\n\1", text)
    text = re.sub(r"(```)\n(?!\n)", r"\1\n\n", text)
    # Pandoc requires a blank line before a table; add one if missing
    text = re.sub(r"([^\n])\n(\|)", r"\1\n\n\2", text)
    # Replace Unicode symbols that pdflatex/T1 cannot handle
    for char, replacement in _UNICODE_REPLACEMENTS:
        text = text.replace(char, replacement)
    return text


# ---------------------------------------------------------------------------
# Document assembly
# ---------------------------------------------------------------------------

APPENDICES_PART_HEADER = "\n\n# Appendices\n\n"


def assemble(files, book_dir: Path) -> str:
    parts = [YAML_METADATA]

    for entry in files:
        # Sentinel: inject the Appendices part heading
        if entry is None:
            parts.append(APPENDICES_PART_HEADER)
            continue

        path, shift = entry

        try:
            text = path.read_text(encoding="utf-8")
        except Exception as exc:
            print(f"  WARNING: cannot read {path}: {exc}", file=sys.stderr)
            continue

        text = clean_content(text)
        text = shift_headings(text, shift)

        rel = path.relative_to(book_dir)
        parts.append(f"\n\n<!-- === {rel} === -->\n\n")
        parts.append(text)
        parts.append("\n\n")

    return "".join(parts)


# ---------------------------------------------------------------------------
# Dependency checks
# ---------------------------------------------------------------------------

def check_pandoc() -> bool:
    if not shutil.which("pandoc"):
        print("  ERROR: pandoc not found — install from https://pandoc.org",
              file=sys.stderr)
        return False
    v = subprocess.run(["pandoc", "--version"],
                       capture_output=True, text=True).stdout.splitlines()[0]
    print(f"  pandoc : {v}")
    return True


def pick_latex_engine() -> str | None:
    # pdflatex is preferred: more robust with the Unicode characters in this
    # textbook (em-dashes, accented letters in researcher names) via inputenc+T1.
    # xelatex is available as a fallback via --engine=xelatex.
    for engine in ("pdflatex", "xelatex"):
        if shutil.which(engine):
            print(f"  LaTeX  : {engine}")
            return engine
    print("  WARNING: no LaTeX engine found (pdflatex / xelatex). "
          "Use --html for HTML output.", file=sys.stderr)
    return None


# ---------------------------------------------------------------------------
# Build targets
# ---------------------------------------------------------------------------

def build_pdf(combined: str, output: Path, engine: str, verbose: bool) -> bool:
    tmp_md = tmp_tex = None
    try:
        with tempfile.NamedTemporaryFile(
            suffix=".md", mode="w", encoding="utf-8", delete=False
        ) as fh:
            fh.write(combined)
            tmp_md = fh.name

        with tempfile.NamedTemporaryFile(
            suffix=".tex", mode="w", encoding="utf-8", delete=False
        ) as fh:
            fh.write(LATEX_HEADER)
            tmp_tex = fh.name

        cmd = [
            "pandoc", tmp_md,
            "--from",  "markdown+tex_math_dollars+raw_tex+pipe_tables"
                       "+fenced_code_blocks+smart",
            "--to",    "pdf",
            f"--pdf-engine={engine}",
            "--top-level-division=part",
            "--toc",
            "--toc-depth=2",
            "--number-sections",
            "--highlight-style=tango",
            f"-H", tmp_tex,
            "--output", str(output),
        ]

        # xelatex needs mainfont set; pdflatex uses lmodern via inputenc
        if engine in ("xelatex", "lualatex"):
            cmd += [
                "--variable=mainfont:Latin Modern Roman",
                "--variable=monofont:Latin Modern Mono",
            ]

        if verbose:
            print("$", " ".join(cmd))

        result = subprocess.run(cmd, capture_output=not verbose, text=True)
        if result.returncode != 0:
            print("\n--- pandoc/LaTeX stderr (last 60 lines) ---", file=sys.stderr)
            for line in (result.stderr or "").splitlines()[-60:]:
                print(f"  {line}", file=sys.stderr)
            return False

    finally:
        for p in (tmp_md, tmp_tex):
            if p and os.path.exists(p):
                os.unlink(p)

    return True


def build_html(combined: str, output: Path, verbose: bool) -> bool:
    html_out = output.with_suffix(".html")

    with tempfile.NamedTemporaryFile(
        suffix=".md", mode="w", encoding="utf-8", delete=False
    ) as fh:
        fh.write(combined)
        tmp = fh.name

    cmd = [
        "pandoc", tmp,
        "--from",  "markdown+tex_math_dollars+pipe_tables+fenced_code_blocks+smart",
        "--to",    "html5",
        "--standalone",
        "--mathjax",
        "--toc",
        "--toc-depth=2",
        "--number-sections",
        "--highlight-style=tango",
        "--output", str(html_out),
        "--metadata", "title=Reservoir Computing: From First Principles to the Frontier",
    ]

    if verbose:
        print("$", " ".join(cmd))

    try:
        result = subprocess.run(cmd, capture_output=not verbose, text=True)
        if result.returncode != 0:
            print(result.stderr[-3000:], file=sys.stderr)
            return False
    finally:
        os.unlink(tmp)

    return True


def build_markdown(combined: str, output: Path) -> bool:
    md_out = output.with_suffix(".md")
    md_out.write_text(combined, encoding="utf-8")
    size_kb = md_out.stat().st_size // 1024
    print(f"  Written: {md_out}  ({size_kb:,} KB)")
    return True


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------

def parse_range(s: str):
    if "-" in s:
        lo, hi = s.split("-", 1)
        return int(lo), int(hi)
    n = int(s)
    return n, n


def main():
    parser = argparse.ArgumentParser(
        description="Build the Reservoir Computing textbook.",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__,
    )
    parser.add_argument("-o", "--output", default=str(DEFAULT_OUTPUT),
                        help="Output path (default: %(default)s)")
    parser.add_argument("--html", action="store_true",
                        help="Produce HTML instead of PDF (no LaTeX required)")
    parser.add_argument("--markdown", action="store_true",
                        help="Dump combined markdown file without compiling")
    parser.add_argument("--check", action="store_true",
                        help="Check dependencies and list files, then exit")
    parser.add_argument("--chapters", metavar="N[-M]",
                        help="Include only chapters N through M")
    parser.add_argument("--book-dir", default=str(BOOK_DIR),
                        help="Path to the book/ directory (default: %(default)s)")
    parser.add_argument("--engine", metavar="ENGINE",
                        help="LaTeX engine override: pdflatex, xelatex, lualatex")
    parser.add_argument("-v", "--verbose", action="store_true",
                        help="Pass pandoc/LaTeX output through to stdout")
    args = parser.parse_args()

    book_dir = Path(args.book_dir)
    output   = Path(args.output)

    if not book_dir.exists():
        sys.exit(f"ERROR: book directory not found: {book_dir}")

    # ---- Dependency check ----
    print("Checking dependencies...")
    ok = check_pandoc()
    if not ok:
        sys.exit(1)

    latex_engine = None
    if not args.html and not args.markdown:
        latex_engine = args.engine if args.engine else pick_latex_engine()
        if latex_engine is None:
            sys.exit(
                "No LaTeX engine found.\n"
                "  Install TeX Live: https://tug.org/texlive/\n"
                "  Or use --html for an HTML build."
            )

    # ---- File collection ----
    chapter_range = parse_range(args.chapters) if args.chapters else None
    files = collect_files(book_dir, chapter_range)

    real_files = [f for f in files if f is not None]
    print(f"\nCollected {len(real_files)} files "
          f"({'all chapters' if not chapter_range else f'chapters {chapter_range[0]}–{chapter_range[1]}'})")

    if args.check:
        for entry in files:
            if entry is None:
                print("  [sentinel: \\part{Appendices}]")
            else:
                path, shift = entry
                print(f"  [{shift:+d}]  {path.relative_to(book_dir)}")
        return

    # ---- Assemble ----
    print("Assembling document...")
    combined = assemble(files, book_dir)
    chars = len(combined)
    words = len(combined.split())
    print(f"  {chars:,} characters  ≈ {words:,} words")

    # ---- Build ----
    if args.markdown:
        print(f"\nWriting combined markdown...")
        build_markdown(combined, output)
        return

    if args.html:
        html_path = output.with_suffix(".html")
        print(f"\nBuilding HTML → {html_path}")
        success = build_html(combined, output, args.verbose)
    else:
        print(f"\nBuilding PDF → {output}")
        print("  This can take 5–15 minutes for the full textbook.")
        print("  Use --chapters 1-3 for a quick test build.")
        success = build_pdf(combined, output, latex_engine, args.verbose)

    if not success:
        print("\nBuild FAILED. Re-run with --verbose for details.", file=sys.stderr)
        sys.exit(1)

    final_path = output if not args.html else output.with_suffix(".html")
    size_mb = final_path.stat().st_size / 1_000_000
    print(f"\nDone: {final_path}  ({size_mb:.1f} MB)")


if __name__ == "__main__":
    main()
