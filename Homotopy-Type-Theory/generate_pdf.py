#!/usr/bin/env python3
"""
generate_pdf.py — Compile the HoTT textbook into a PDF.

Requirements:
  pandoc >= 2.11  (sudo apt install pandoc  OR  brew install pandoc)
  One of:
    xelatex  — best Unicode support (sudo apt install texlive-xetex texlive-fonts-extra texlive-latex-extra)
    lualatex — good Unicode support (sudo apt install texlive-luatex texlive-fonts-extra texlive-latex-extra)
    pdflatex — basic support      (sudo apt install texlive-full)

Usage:
  python3 generate_pdf.py                     # full book → hott-textbook.pdf
  python3 generate_pdf.py -o my-book.pdf      # custom output path
  python3 generate_pdf.py --unit 6            # only unit 06 (core HoTT)
  python3 generate_pdf.py --chapter 18        # only chapter 18 (univalence)
  python3 generate_pdf.py --engine pdflatex   # override LaTeX engine
  python3 generate_pdf.py --no-toc            # omit table of contents
  python3 generate_pdf.py --list-units        # show available units
  python3 generate_pdf.py --dry-run           # assemble markdown, skip pandoc
"""

from __future__ import annotations

import argparse
import pathlib
import re
import shutil
import subprocess
import sys
import tempfile
from typing import Optional

ROOT = pathlib.Path(__file__).parent
BOOK_DIR = ROOT / "book"

# ─── ANSI helpers ─────────────────────────────────────────────────────────────

def _bold(s: str) -> str:
    return f"\033[1m{s}\033[0m"

def _dim(s: str) -> str:
    return f"\033[2m{s}\033[0m"

def _green(s: str) -> str:
    return f"\033[32m{s}\033[0m"

def _red(s: str) -> str:
    return f"\033[31m{s}\033[0m"

def _cyan(s: str) -> str:
    return f"\033[36m{s}\033[0m"


# ─── YAML front matter ────────────────────────────────────────────────────────

_FRONTMATTER_XETEX = """\
---
title: "Homotopy Type Theory"
subtitle: "A Complete Curriculum — From Mathematical Foundations to Research Frontiers"
date: "2026"
documentclass: book
classoption:
  - 11pt
  - openany
geometry:
  - top=1.25in
  - bottom=1.25in
  - left=1.35in
  - right=1.0in
header-includes:
  - \\usepackage{fontspec}
  - \\usepackage{unicode-math}
  - \\usepackage{microtype}
  - \\usepackage{xcolor}
  - \\definecolor{linkblue}{RGB}{25,85,155}
  - \\definecolor{chaptercolor}{RGB}{40,40,80}
  - \\usepackage{fancyhdr}
  - \\pagestyle{fancy}
  - \\fancyhf{}
  - \\fancyhead[LE]{\\small\\nouppercase{\\leftmark}}
  - \\fancyhead[RO]{\\small\\nouppercase{\\rightmark}}
  - \\fancyfoot[C]{\\small\\thepage}
  - \\renewcommand{\\headrulewidth}{0.3pt}
  - \\renewcommand{\\footrulewidth}{0pt}
  - \\usepackage{titlesec}
  - \\titleformat{\\chapter}[display]{\\normalfont\\huge\\bfseries\\color{chaptercolor}}{\\chaptertitlename\\ \\thechapter}{16pt}{\\Huge}
  - \\titlespacing*{\\chapter}{0pt}{0pt}{24pt}
  - \\usepackage{titling}
  - \\usepackage{enumitem}
  - \\setlist{topsep=3pt,itemsep=2pt}
  - \\usepackage{booktabs}
  - \\usepackage{longtable}
  - \\usepackage{array}
  - \\providecommand{\\llbracket}{\\mathopen{[\\![}}
  - \\providecommand{\\rrbracket}{\\mathclose{]\\!]}}
  - \\newcommand{\\shapeop}{\\mathord{\\intop}}
  - \\usepackage{parskip}
  - \\setlength{\\parskip}{6pt}
  - \\setlength{\\parindent}{0pt}
hyperrefoptions:
  - linktoc=all
  - bookmarksnumbered=true
colorlinks: true
linkcolor: linkblue
urlcolor: linkblue
toccolor: black
toc: true
toc-depth: 2
numbersections: true
---
"""

_FRONTMATTER_PDFLATEX = """\
---
title: "Homotopy Type Theory"
subtitle: "A Complete Curriculum — From Mathematical Foundations to Research Frontiers"
date: "2026"
documentclass: book
classoption:
  - 11pt
  - openany
geometry:
  - top=1.25in
  - bottom=1.25in
  - left=1.35in
  - right=1.0in
header-includes:
  - \\usepackage[utf8]{inputenc}
  - \\usepackage[T1]{fontenc}
  - \\usepackage{lmodern}
  - \\usepackage{amssymb}
  - \\usepackage{amsmath}
  - \\usepackage{amsthm}
  - \\usepackage{microtype}
  - \\usepackage{xcolor}
  - \\definecolor{linkblue}{RGB}{25,85,155}
  - \\definecolor{chaptercolor}{RGB}{40,40,80}
  - \\usepackage{fancyhdr}
  - \\pagestyle{fancy}
  - \\fancyhf{}
  - \\fancyhead[LE]{\\small\\nouppercase{\\leftmark}}
  - \\fancyhead[RO]{\\small\\nouppercase{\\rightmark}}
  - \\fancyfoot[C]{\\small\\thepage}
  - \\renewcommand{\\headrulewidth}{0.3pt}
  - \\usepackage{titlesec}
  - \\titleformat{\\chapter}[display]{\\normalfont\\huge\\bfseries\\color{chaptercolor}}{\\chaptertitlename\\ \\thechapter}{16pt}{\\Huge}
  - \\titlespacing*{\\chapter}{0pt}{0pt}{24pt}
  - \\usepackage{enumitem}
  - \\setlist{topsep=3pt,itemsep=2pt}
  - \\usepackage{booktabs}
  - \\usepackage{longtable}
  - \\usepackage{array}
  - \\providecommand{\\llbracket}{\\mathopen{[\\![}}
  - \\providecommand{\\rrbracket}{\\mathclose{]\\!]}}
  - \\newcommand{\\shapeop}{\\mathord{\\intop}}
  - \\usepackage{parskip}
  - \\setlength{\\parskip}{6pt}
  - \\setlength{\\parindent}{0pt}
colorlinks: true
linkcolor: linkblue
urlcolor: linkblue
toc: true
toc-depth: 2
numbersections: true
---
"""


# ─── LaTeX math preprocessing ────────────────────────────────────────────────

def fix_latex_math(text: str) -> str:
    r"""
    Fix common LaTeX math patterns that cause compilation errors.

    Specifically: subscripts/superscripts of the form  _\cmd{X}  or  ^\cmd{X}
    need outer braces in LaTeX: they must be  _{\cmd{X}}  and  ^{\cmd{X}}.

    Examples fixed:
      $\vdash_\text{IPC} A$        ->  $\vdash_{\text{IPC}} A$
      $\mathsf{rec}_\mathbb{N}$    ->  $\mathsf{rec}_{\mathbb{N}}$
      $0_\mathbf{2}$               ->  $0_{\mathbf{2}}$
      $X_\mathcal{C}$              ->  $X_{\mathcal{C}}$
    """
    # \int as a superscript/subscript label (modal HoTT shape modality notation)
    # causes a "Limit controls must follow a math operator" error because \int is
    # defined as \mathop{\intop}\nolimits and the trailing \nolimits is illegal
    # inside a superscript group.  Replace ^\int with ^\shapeop (\shapeop is
    # defined in the front matter as \mathord{\intop}).
    text = re.sub(r'\^\\int\b', r'^{\\shapeop}', text)
    text = re.sub(r'_\\int\b', r'_{\\shapeop}', text)

    # One-level: _\cmd{X} → _{\cmd{X}}   (argument has no nested braces)
    text = re.sub(
        r'([_^])\\([A-Za-z]+)\{([^{}]*)\}',
        r'\1{\\\2{\3}}',
        text,
    )
    # Two-level: _\cmd{X{Y}Z} → _{\cmd{X{Y}Z}}   (single level of nesting)
    # Run a second pass to catch cases the first pass may have revealed
    text = re.sub(
        r'([_^])\\([A-Za-z]+)\{([^{}]*\{[^{}]*\}[^{}]*)\}',
        r'\1{\\\2{\3}}',
        text,
    )
    return text


# ─── File discovery ───────────────────────────────────────────────────────────

def _sorted_dirs(parent: pathlib.Path, prefix: str = "") -> list[pathlib.Path]:
    return sorted(d for d in parent.iterdir() if d.is_dir() and d.name.startswith(prefix))


def unit_dirs(unit_filter: Optional[int] = None) -> list[pathlib.Path]:
    dirs = _sorted_dirs(BOOK_DIR, "unit-")
    if unit_filter is not None:
        dirs = [d for d in dirs if d.name.startswith(f"unit-{unit_filter:02d}-")]
    return dirs


def chapter_dirs(unit_dir: pathlib.Path, chapter_filter: Optional[int] = None) -> list[pathlib.Path]:
    dirs = _sorted_dirs(unit_dir, "ch")
    if chapter_filter is not None:
        dirs = [d for d in dirs if d.name.startswith(f"ch{chapter_filter:02d}-")]
    return dirs


def section_dirs(ch_dir: pathlib.Path) -> list[pathlib.Path]:
    """Return all section subdirectories in sorted order."""
    return sorted(d for d in ch_dir.iterdir() if d.is_dir())


# ─── Markdown processing ──────────────────────────────────────────────────────

def read_md(path: pathlib.Path) -> str:
    """Read a markdown file, stripping YAML front matter and fixing LaTeX math."""
    text = path.read_text(encoding="utf-8", errors="replace")
    if text.startswith("---"):
        end = text.find("\n---", 3)
        if end != -1:
            text = text[end + 4:]
    text = fix_latex_math(text)
    return text.strip()


def extract_h1(text: str) -> Optional[str]:
    """Return the text of the first # heading, or None."""
    for line in text.split("\n"):
        if re.match(r"^#\s+\S", line):
            return line.lstrip("#").strip()
    return None


def strip_h1(text: str) -> str:
    """Remove the first # heading line from text."""
    lines = text.split("\n")
    for i, line in enumerate(lines):
        if re.match(r"^#\s+\S", line):
            remaining = "\n".join(lines[:i] + lines[i + 1:]).strip()
            return remaining
    return text


def shift_headers(text: str, by: int) -> str:
    """
    Shift all Markdown header levels by `by`, respecting fenced code blocks.
    Headers inside ``` or ~~~ fences are left untouched.
    """
    if by == 0:
        return text
    lines = text.split("\n")
    result: list[str] = []
    fence: Optional[str] = None

    for line in lines:
        # Detect start/end of fenced code block
        m = re.match(r"^(`{3,}|~{3,})", line)
        if m:
            delim = m.group(1)
            if fence is None:
                fence = delim
            elif line.rstrip().startswith(fence):
                fence = None
            result.append(line)
            continue

        if fence is None:
            hm = re.match(r"^(#{1,6})(\s+.*|$)", line)
            if hm:
                level = len(hm.group(1))
                new_level = min(level + by, 6)
                result.append("#" * new_level + hm.group(2))
                continue

        result.append(line)

    return "\n".join(result)


def raw_latex(content: str) -> str:
    """Wrap a string in a pandoc raw LaTeX block."""
    return f"```{{=latex}}\n{content}\n```"


# ─── Dependency checks ────────────────────────────────────────────────────────

def detect_engine() -> Optional[str]:
    for engine in ("xelatex", "lualatex", "pdflatex"):
        if shutil.which(engine):
            return engine
    return None


def check_deps(engine: str) -> None:
    if not shutil.which("pandoc"):
        sys.exit(
            _red("Error:") + " pandoc not found.\n"
            "  Install: sudo apt install pandoc\n"
            "       or: brew install pandoc"
        )
    if not shutil.which(engine):
        sys.exit(
            _red("Error:") + f" {engine} not found.\n"
            "  Install: sudo apt install texlive-xetex texlive-fonts-extra texlive-latex-extra\n"
            "       or: sudo apt install texlive-full"
        )


# ─── Document assembly ────────────────────────────────────────────────────────

def _unit_title(unit_dir: pathlib.Path) -> str:
    readme = unit_dir / "README.md"
    if readme.exists():
        title = extract_h1(read_md(readme))
        if title:
            return title
    # Fallback: derive from directory name  e.g. "unit-01-mathematical-foundations"
    parts = unit_dir.name.split("-", 2)
    return parts[2].replace("-", " ").title() if len(parts) >= 3 else unit_dir.name


def _chapter_title(ch_dir: pathlib.Path) -> str:
    readme = ch_dir / "README.md"
    if readme.exists():
        title = extract_h1(read_md(readme))
        if title:
            return title
    parts = ch_dir.name.split("-", 1)
    return parts[1].replace("-", " ").title() if len(parts) >= 2 else ch_dir.name


def build_combined_markdown(
    unit_filter: Optional[int],
    chapter_filter: Optional[int],
    engine: str,
    verbose: bool = False,
) -> str:
    chunks: list[str] = []

    # Choose front matter based on engine
    fm = _FRONTMATTER_XETEX if engine in ("xelatex", "lualatex") else _FRONTMATTER_PDFLATEX
    chunks.append(fm.strip())
    chunks.append("")

    # Book-level README → "Introduction" chapter (only in full-book builds)
    if unit_filter is None and chapter_filter is None:
        book_readme = BOOK_DIR / "README.md"
        if book_readme.exists():
            if verbose:
                print(f"  {_dim('intro')}  {book_readme.relative_to(ROOT)}")
            chunks.append(read_md(book_readme))
            chunks.append("")
            chunks.append(raw_latex(r"\clearpage"))
            chunks.append("")

    all_units = unit_dirs(unit_filter)
    for ui, unit_dir in enumerate(all_units):
        title = _unit_title(unit_dir)
        if verbose:
            print(f"  {_cyan('unit')}   {unit_dir.name}  ({title})")

        # Inject \part{} for full builds; skip for single-unit/chapter builds
        if unit_filter is None:
            chunks.append(raw_latex(f"\\part{{{title}}}"))
            chunks.append("")
            # Unit README → part overview chapter (full builds only)
            unit_readme = unit_dir / "README.md"
            if unit_readme.exists():
                if verbose:
                    print(f"    {_dim('overview')}  {unit_readme.relative_to(ROOT)}")
                chunks.append(read_md(unit_readme))
                chunks.append("")

        all_chapters = chapter_dirs(unit_dir, chapter_filter)
        for ci, ch_dir in enumerate(all_chapters):
            ch_title = _chapter_title(ch_dir)
            if verbose:
                print(f"    {_dim('ch')}     {ch_dir.name}")

            # Chapter README: # heading → \chapter (via --top-level-division=chapter)
            ch_readme = ch_dir / "README.md"
            if ch_readme.exists():
                chunks.append(read_md(ch_readme))
                chunks.append("")

            # Section files: shift headers by 1 so # → ## → \section
            for sec_dir in section_dirs(ch_dir):
                md_files = sorted(sec_dir.glob("*.md"))
                for md_file in md_files:
                    text = read_md(md_file)
                    text = shift_headers(text, by=1)
                    chunks.append(text)
                    chunks.append("")

            # Page break between chapters (except last in unit)
            if ci < len(all_chapters) - 1:
                chunks.append(raw_latex(r"\clearpage"))
                chunks.append("")

        # Page break between units (except last)
        if ui < len(all_units) - 1:
            chunks.append(raw_latex(r"\cleardoublepage"))
            chunks.append("")

    return "\n".join(chunks)


# ─── PDF generation ───────────────────────────────────────────────────────────

def generate_pdf(
    output: pathlib.Path,
    engine: str,
    toc: bool,
    unit_filter: Optional[int],
    chapter_filter: Optional[int],
    verbose: bool,
) -> None:
    print()
    print(_bold("  Homotopy Type Theory — PDF Generator"))
    print(_dim("  " + "─" * 52))

    check_deps(engine)

    pandoc_ver = subprocess.run(
        ["pandoc", "--version"], capture_output=True, text=True
    ).stdout.split("\n")[0]
    print(f"  pandoc:  {pandoc_ver}")
    print(f"  engine:  {engine}")
    print(f"  output:  {output}")

    if not BOOK_DIR.exists():
        sys.exit(f"\n{_red('Error:')} book/ directory not found at {BOOK_DIR}\n"
                 "Run this script from the project root.")

    print(f"\n  Assembling content...")
    combined = build_combined_markdown(unit_filter, chapter_filter, engine, verbose)

    word_count = len(combined.split())
    line_count = combined.count("\n")
    print(f"  {word_count:,} words  /  {line_count:,} lines assembled")

    with tempfile.TemporaryDirectory(prefix="hott-pdf-") as tmpdir:
        tmp = pathlib.Path(tmpdir)
        md_file = tmp / "book.md"
        md_file.write_text(combined, encoding="utf-8")

        print(f"\n  Running pandoc + {engine}  (this may take a few minutes)...")

        cmd = [
            "pandoc",
            str(md_file),
            "--pdf-engine", engine,
            "--top-level-division=chapter",
            "--highlight-style=tango",
            "--standalone",
            "--wrap=preserve",           # don't re-wrap paragraphs
            "-o", str(output),
        ]

        if not toc:
            cmd += ["--metadata", "toc=false"]

        # Pass TEXINPUTS so LaTeX can find extra style files if present
        import os
        env = os.environ.copy()

        if verbose:
            print(_dim(f"\n  $ {' '.join(cmd[:6])} ..."))

        proc = subprocess.run(cmd, capture_output=True, text=True, env=env)

        if proc.returncode != 0:
            print(f"\n  {_red('Error:')} pandoc / LaTeX compilation failed.")
            print(_dim("\n  ── LaTeX error output (last 60 lines) ──"))
            stderr_lines = (proc.stderr or "").strip().split("\n")
            # Find the most relevant error lines
            error_lines = [l for l in stderr_lines if "Error" in l or "error" in l or l.startswith("!")]
            display_lines = error_lines[-20:] if error_lines else stderr_lines[-60:]
            for line in display_lines:
                print(f"    {line}")
            print()
            print(_dim("  Tip: Run with --dry-run to inspect the assembled markdown."))
            print(_dim(f"  Tip: Check the temp file that was written to {tmp}/book.md"))
            sys.exit(1)

        if not output.exists():
            sys.exit(f"\n{_red('Error:')} pandoc reported success but {output} was not created.")

        size_mb = output.stat().st_size / 1_048_576
        print(f"\n  {_green('✓')} {_bold('PDF generated successfully')}")
        print(f"  {output}  ({size_mb:.1f} MB)")
        print()


# ─── CLI ──────────────────────────────────────────────────────────────────────

def main() -> None:
    parser = argparse.ArgumentParser(
        description="Generate the HoTT textbook as a PDF",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__,
    )
    parser.add_argument(
        "-o", "--output",
        type=pathlib.Path,
        default=ROOT / "hott-textbook.pdf",
        metavar="FILE",
        help="Output PDF path (default: hott-textbook.pdf)",
    )
    parser.add_argument(
        "--unit", type=int, metavar="N",
        help="Generate only unit N (1–9)",
    )
    parser.add_argument(
        "--chapter", type=int, metavar="N",
        help="Generate only chapter N (0–26)",
    )
    parser.add_argument(
        "--engine",
        choices=["xelatex", "lualatex", "pdflatex"],
        help="LaTeX engine to use (default: auto-detect, prefers xelatex)",
    )
    parser.add_argument(
        "--no-toc", action="store_true",
        help="Omit the table of contents",
    )
    parser.add_argument(
        "--list-units", action="store_true",
        help="List available units and exit",
    )
    parser.add_argument(
        "--dry-run", action="store_true",
        help="Assemble the combined markdown but do not run pandoc; print stats",
    )
    parser.add_argument(
        "-v", "--verbose", action="store_true",
        help="Show each file being processed",
    )

    args = parser.parse_args()

    # ── --list-units ──
    if args.list_units:
        print("\nAvailable units:\n")
        for d in unit_dirs():
            title = _unit_title(d)
            n_chapters = len(chapter_dirs(d))
            n_files = sum(1 for _ in d.rglob("*.md"))
            print(f"  {d.name[:9]}  {title:<42}  {n_chapters} ch / {n_files} files")
        print()
        return

    # ── determine engine ──
    engine = args.engine or detect_engine()
    if engine is None:
        sys.exit(
            _red("Error:") + " no LaTeX engine found.\n"
            "  Install: sudo apt install texlive-xetex texlive-fonts-extra texlive-latex-extra"
        )

    # ── --dry-run ──
    if args.dry_run:
        engine_for_dry = engine
        md = build_combined_markdown(args.unit, args.chapter, engine_for_dry, args.verbose)
        tmp_path = pathlib.Path(tempfile.mktemp(suffix=".md"))
        tmp_path.write_text(md, encoding="utf-8")
        wc = len(md.split())
        lc = md.count("\n")
        print(f"\n  Dry run complete: {wc:,} words / {lc:,} lines")
        print(f"  Assembled markdown: {tmp_path}")
        print()
        return

    # ── generate PDF ──
    output = args.output.resolve()
    generate_pdf(
        output=output,
        engine=engine,
        toc=not args.no_toc,
        unit_filter=args.unit,
        chapter_filter=args.chapter,
        verbose=args.verbose,
    )


if __name__ == "__main__":
    main()
