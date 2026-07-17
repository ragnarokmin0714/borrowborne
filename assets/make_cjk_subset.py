#!/usr/bin/env python3
"""Regenerate the embedded CJK subset font for the app.

Collects every non-ASCII character used anywhere in the app sources
(i18n strings AND symbol glyphs like │ ▼ ◉ that egui's proportional
fonts lack), adds the full kana and CJK-punctuation/fullwidth ranges,
and cuts a tiny OTF out of Noto Sans CJK TC.

Usage:
    python3 assets/make_cjk_subset.py path/to/NotoSansCJKtc-Regular.otf

Source font: https://github.com/notofonts/noto-cjk (SIL OFL 1.1).
Output: crates/borrowborne-app/assets/cjk-subset.otf

The `fonts_cover_i18n` test verifies the result; symbols the source
font lacks entirely (emoji, alchemical marks) must instead be chars
egui's own fonts carry — the test tells you which ones fail.
"""

import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
SRC = ROOT / "crates/borrowborne-app/src"
OUT = ROOT / "crates/borrowborne-app/assets/cjk-subset.otf"

# Always-included ranges: CJK punctuation, kana, fullwidth forms.
RANGES = ["3000-303F", "3040-309F", "30A0-30FF", "FF01-FF60"]


def used_chars() -> set[str]:
    chars: set[str] = set()
    for path in sorted(SRC.rglob("*.rs")):
        chars.update(path.read_text(encoding="utf-8"))
    # Everything beyond ASCII: the subsetter silently skips codepoints
    # the source font lacks, and the coverage test judges the result.
    return {c for c in chars if ord(c) > 0x7F}


def main() -> None:
    if len(sys.argv) != 2:
        sys.exit(__doc__)
    source = sys.argv[1]
    unicodes = ",".join(f"{ord(c):04X}" for c in sorted(used_chars()))
    unicodes = ",".join(RANGES + [unicodes]) if unicodes else ",".join(RANGES)
    OUT.parent.mkdir(parents=True, exist_ok=True)
    subprocess.run(
        [
            sys.executable,
            "-m",
            "fontTools.subset",
            source,
            f"--unicodes={unicodes}",
            f"--output-file={OUT}",
            "--layout-features=",  # strip OpenType features: plain text only
            "--no-hinting",
            "--desubroutinize",
        ],
        check=True,
    )
    print(f"{OUT} ({OUT.stat().st_size / 1024:.0f} KiB)")


if __name__ == "__main__":
    main()
