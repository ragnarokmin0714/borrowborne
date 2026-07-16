#!/usr/bin/env python3
"""Regenerate the embedded CJK subset font for the app.

Collects every character used in the i18n sources, adds the full kana
and CJK-punctuation/fullwidth ranges (so small future edits don't need
a re-subset), and cuts a tiny OTF out of Noto Sans CJK TC.

Usage:
    python3 assets/make_cjk_subset.py path/to/NotoSansCJKtc-Regular.otf

Source font: https://github.com/notofonts/noto-cjk (SIL OFL 1.1).
Output: crates/borrowborne-app/assets/cjk-subset.otf
"""

import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
I18N = ROOT / "crates/borrowborne-app/src/i18n"
OUT = ROOT / "crates/borrowborne-app/assets/cjk-subset.otf"

# Always-included ranges: CJK punctuation, kana, fullwidth forms.
RANGES = ["3000-303F", "3040-309F", "30A0-30FF", "FF01-FF60"]


def used_chars() -> set[str]:
    chars: set[str] = set()
    for path in sorted(I18N.glob("*.rs")):
        chars.update(path.read_text(encoding="utf-8"))
    # Only characters the default egui fonts cannot draw.
    return {c for c in chars if ord(c) >= 0x3000}


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
