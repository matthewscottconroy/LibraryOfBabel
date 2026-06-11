#!/usr/bin/env python3
"""
build_pdf.py — Build "Computational Systems and Synthetic Biology" as a PDF.

Requirements:
    pandoc >= 2.0, XeLaTeX (texlive-xetex)

Usage:
    python3 build_pdf.py
    python3 build_pdf.py --draft
    python3 build_pdf.py --markdown-only
"""

import argparse, re, subprocess, sys, tempfile
from pathlib import Path

BOOK_ROOT   = Path(__file__).resolve().parent
DEFAULT_OUT = BOOK_ROOT / "computational-systems-synthetic-biology.pdf"

YAML_HEADER = """\
---
title: "Computational Systems and Synthetic Biology"
subtitle: "From Molecular Circuits to Designed Life"
documentclass: book
classoption: [11pt, openright, twoside]
geometry: "inner=1.5in, outer=1.1in, top=1.1in, bottom=1.1in, includefoot"
linestretch: 1.2
toc: true
toc-depth: 2
number-sections: true
colorlinks: true
linkcolor: "black"
urlcolor: "NavyBlue"
toccolor: "black"
---
"""

LATEX_PREAMBLE = r"""
\usepackage{fancyhdr}
\usepackage{xcolor}
\usepackage{titlesec}
\usepackage{microtype}
\usepackage{emptypage}
\usepackage{amsmath}

\definecolor{chapcolor}{RGB}{0,100,60}

\titleformat{\chapter}[display]
  {\normalfont\huge\bfseries\color{chapcolor}}
  {\chaptertitlename\ \thechapter}{18pt}{\Huge}
\titlespacing{\chapter}{0pt}{-10pt}{30pt}

\pagestyle{fancy}
\fancyhf{}
\fancyhead[LE]{\small\itshape\leftmark}
\fancyhead[RO]{\small\itshape\rightmark}
\fancyfoot[C]{\small\thepage}
\renewcommand{\headrulewidth}{0.3pt}
\setlength{\headheight}{14pt}
\fancypagestyle{plain}{\fancyhf{}\fancyfoot[C]{\small\thepage}\renewcommand{\headrulewidth}{0pt}}
"""

def read_file(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except OSError:
        return ""

def strip_front_matter(text: str) -> str:
    if not text.startswith("---"):
        return text
    end = text.find("\n---", 3)
    return text[end + 4:].lstrip("\n") if end != -1 else text

def shift_headings(text: str, by: int) -> str:
    if by == 0:
        return text
    def _r(m):
        return "#" * min(len(m.group(1)) + by, 6) + m.group(2)
    return re.sub(r"^(#{1,6})([ \t].*)$", _r, text, flags=re.MULTILINE)

def clean_first_heading(text: str) -> str:
    prefix_re = re.compile(r"^(?:Chapter\s+\d+[:\s]+|Section\s+[\d.]+[:\s]+|Module\s+\d+[:\s]+)")
    def _r(m):
        return m.group(1) + " " + prefix_re.sub("", m.group(2).lstrip()).strip()
    return re.sub(r"^(#{1,6})\s+(.+)$", _r, text, count=1, flags=re.MULTILINE)

def raw_latex(code: str) -> str:
    return f"\n\n```{{=latex}}\n{code}\n```\n\n"

def escape_latex(text: str) -> str:
    return text.replace("\\","\\textbackslash{}").replace("&","\\&").replace("%","\\%").replace("$","\\$").replace("#","\\#").replace("_","\\_").replace("{","\\{").replace("}","\\}").replace("~","\\textasciitilde{}").replace("^","\\textasciicircum{}")

def _leading_int(name: str) -> int:
    m = re.search(r"\d+", name)
    return int(m.group()) if m else 999

def curriculum_dirs() -> list:
    curriculum = BOOK_ROOT / "curriculum"
    if not curriculum.exists():
        return []
    return sorted(
        (d for d in curriculum.iterdir() if d.is_dir()),
        key=lambda d: _leading_int(d.name),
    )

def section_files(chap: Path) -> list:
    return sorted(
        (f for f in chap.iterdir() if f.suffix == ".md" and f.name != "README.md"),
        key=lambda f: _leading_int(f.name),
    )

def assemble(draft: bool = False) -> str:
    parts = [YAML_HEADER]

    readme = BOOK_ROOT / "README.md"
    if readme.exists():
        def _unnumber(t):
            def _r(m):
                line = m.group(0).rstrip()
                return line if "{.unnumbered" in line else line + " {.unnumbered .unlisted}"
            return re.sub(r"^#{1,6}[ \t].+$", _r, t, count=1, flags=re.MULTILINE)
        parts.append(_unnumber(strip_front_matter(read_file(readme))) + "\n\n")

    cdirs = curriculum_dirs()
    if draft:
        cdirs = cdirs[:3]

    for i, cdir in enumerate(cdirs):
        readme = cdir / "README.md"
        body = strip_front_matter(read_file(readme)) if readme.exists() else f"# {cdir.name}\n"
        body = clean_first_heading(body)
        parts.append(raw_latex(r"\cleardoublepage"))
        parts.append(body + "\n\n")

        for sec in section_files(cdir):
            sec_body = strip_front_matter(read_file(sec))
            sec_body = clean_first_heading(sec_body)
            sec_body = shift_headings(sec_body, 1)
            parts.append(sec_body + "\n\n")

    # Also include learning-sequence.md and resources.md as appendices
    for extra in ("learning-sequence.md", "resources.md"):
        p = BOOK_ROOT / extra
        if p.exists():
            parts.append(raw_latex(r"\appendix") if extra == "learning-sequence.md" else "")
            body = strip_front_matter(read_file(p))
            parts.append(body + "\n\n")

    return "".join(parts)

def find_latex_engine() -> str:
    for e in ("xelatex", "lualatex", "pdflatex"):
        try:
            subprocess.run([e, "--version"], capture_output=True, check=True)
            return e
        except (FileNotFoundError, subprocess.CalledProcessError):
            pass
    return ""

def check_pandoc() -> str:
    try:
        r = subprocess.run(["pandoc", "--version"], capture_output=True, text=True, check=True)
        return r.stdout.splitlines()[0]
    except (FileNotFoundError, subprocess.CalledProcessError):
        return ""

def build_pdf(output: Path, draft: bool) -> None:
    pandoc_ver = check_pandoc()
    if not pandoc_ver:
        sys.exit("pandoc not found.")
    engine = find_latex_engine()
    if not engine:
        sys.exit("No LaTeX engine found.")

    combined = assemble(draft=draft)
    print(f"  markdown: {len(combined):,} chars")

    tmp_md  = tempfile.NamedTemporaryFile(mode="w", suffix=".md", encoding="utf-8", delete=False, dir=BOOK_ROOT, prefix="_combined_")
    tmp_tex = tempfile.NamedTemporaryFile(mode="w", suffix=".tex", encoding="utf-8", delete=False, dir=BOOK_ROOT, prefix="_preamble_")
    try:
        tmp_md.write(combined); tmp_md.close()
        tmp_tex.write(LATEX_PREAMBLE); tmp_tex.close()
        cmd = ["pandoc", str(tmp_md.name), "--from", "markdown+raw_tex+smart", "--to", "pdf",
               "--pdf-engine", engine, "--include-in-header", str(tmp_tex.name),
               "--output", str(output), "--top-level-division=chapter", "--highlight-style=tango", "-V", "lang=en-US"]
        result = subprocess.run(cmd, capture_output=True, text=True, cwd=BOOK_ROOT)
        if result.returncode != 0:
            print((result.stdout + "\n" + result.stderr).strip()[-5000:])
            sys.exit(f"Build failed (exit {result.returncode}).")
    finally:
        Path(tmp_md.name).unlink(missing_ok=True)
        Path(tmp_tex.name).unlink(missing_ok=True)

    print(f"\n✓  PDF written to: {output}")
    print(f"   Size: {output.stat().st_size / 1_048_576:.1f} MB")

def main() -> None:
    parser = argparse.ArgumentParser(description="Build Computational Systems and Synthetic Biology PDF.")
    parser.add_argument("--output", "-o", metavar="FILE", type=Path, default=DEFAULT_OUT)
    parser.add_argument("--draft", action="store_true")
    parser.add_argument("--markdown-only", action="store_true")
    args = parser.parse_args()

    if args.markdown_only:
        print(assemble(draft=args.draft))
        return

    print("Building Computational Systems and Synthetic Biology PDF")
    print("=" * 55)
    build_pdf(args.output, draft=args.draft)

if __name__ == "__main__":
    main()
