"""Gate tests for --emit-dispatch-tables (P5, roadmap 2026-10-03 §6)."""
import re
import shutil
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[3]
MAIN = REPO / "tools" / "py2rust" / "main.py"


def _emit(tmp_path: Path) -> tuple[str, str]:
    """Run the emitter against a COPY of the repo src tree: the handler
    existence scan reads --out, so an empty temp dir would yield empty tables."""
    out = tmp_path / "src"
    shutil.copytree(REPO / "src", out)
    subprocess.run(
        [sys.executable, str(MAIN), "--py-armodel", str(REPO / "target" / "py-armodel"),
         "--out", str(out), "--emit-dispatch-tables"],
        check=True,
    )
    reader = (out / "reader" / "arxml_reader" / "dispatch_tables.rs").read_text()
    writer = (out / "writer" / "arxml_writer" / "dispatch_tables.rs").read_text()
    return reader, writer


def test_reader_table_is_sorted_and_covers_ported_handlers(tmp_path):
    reader, _ = _emit(tmp_path)
    tags = re.findall(r'\(\s*"([A-Z0-9-]+)",', reader)
    assert tags, "no reader table entries parsed"
    assert tags == sorted(tags), "reader table must be sorted by tag for binary search"
    assert len(tags) == len(set(tags)), "duplicate tags mean registry pairing went wrong"
    assert "COMPU-METHOD" in tags          # ported handler (read_compu_method exists)
    assert "DATA-CONSTR" in tags


def test_unported_methods_produce_no_table_entry(tmp_path):
    reader, writer = _emit(tmp_path)
    assert "read_sender_com_spec" not in reader        # not ported at P5 start
    assert "write_sender_com_spec" not in writer
