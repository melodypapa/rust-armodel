"""Tests for --emit-port-checklist (plan 2026-10-03, Task 1)."""
import ast
import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))

from py2rust.port_checklist import collect_py_methods, rust_name, check_ported

PY_ROOT = pathlib.Path(__file__).resolve().parents[3] / "target" / "py-armodel" / "src" / "armodel"
SRC_ROOT = pathlib.Path(__file__).resolve().parents[3] / "src"


def test_collect_finds_reader_and_writer_methods():
    methods = collect_py_methods(PY_ROOT)
    names = [name for name, _ in methods]
    assert "readCompuMethod" in names
    assert "writeCompuMethod" in names
    # admin-data reading is split across helper-named methods in py
    # (getAdminData is a getter, not read*); the checklist tracks read*/write*.
    assert "readAdminDataSdgs" in names
    assert "writeSwBaseType" in names
    # non reader/writer methods are excluded
    assert "load" not in names
    assert "getChildElementOptionalLiteral" not in names


def test_snake_case_mapping():
    assert rust_name("readCompuMethod") == "read_compu_method"
    assert rust_name("readARPackage") == "read_ar_package"
    assert rust_name("writeSwBaseType") == "write_sw_base_type"


def test_status_scan_sees_ported_methods(tmp_path):
    (tmp_path / "lib.rs").write_text("mod parser { fn read_admin_data() {} }\n")
    assert check_ported("readAdminData", tmp_path)
    assert not check_ported("readCompuMethod", tmp_path)
