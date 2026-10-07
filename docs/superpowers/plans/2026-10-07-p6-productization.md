# P6 Productization Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development
> (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use
> checkbox (`- [ ]`) syntax for tracking.

**Goal:** Ship the two kept CLIs (`arxml-format`, `arxml-dump`) at py flag/behavior parity,
publish the crate, and complete release engineering (version 1.0.0, docs.rs, README,
CHANGELOG, CI correction).

**Architecture:** CLI parity is verified against the pinned py CLI implementations
(`target/py-armodel/src/armodel/cli/`) with golden-output tests — py is run once at
plan-execution time to produce expected bytes, committed under `tests/golden/`, and the Rust
CLIs are driven to byte-equality. Release engineering is ordinary cargo/CI work. Py CLI
tools outside the kept two (`connector2xlsx`, `connector-update`, `format-xml`,
`uuid-checker`, `swc-list`, `system-signal`, `file-list`, `memory-section`,
`os-config-export`) stay unported — roadmap non-goal.

**Tech Stack:** Rust + clap 4 (already), `cargo publish` / `cargo doc`, GitHub Actions
(`.github/workflows/quality.yml`, already), pinned py-armodel for goldens.

**Prerequisite:** P2–P4 complete — the byte-identical bar and the model areas `arxml-dump`
walks (PortInterface, BSW descriptions) must exist first. `arxml-format` work (Task 1) can
start as soon as P2–P4's format gate covers the fixtures used in its tests; `arxml-dump`
(Task 2) needs the System/communication and BSW batches landed.

---

## File structure

- Modify: `src/bin/arxml-format.rs` — exit codes, flag behavior gaps (Task 1)
- Modify: `src/bin/arxml-dump.rs` — 1:1 report output (Task 2)
- Create: `tests/golden/arxml_format/<fixture>.expected.arxml` — py-produced goldens (Task 1)
- Create: `tests/golden/arxml_dump/<fixture>.expected.txt` — py-produced goldens (Task 2)
- Create: `tests/integration/cli_parity.rs` + `[[test]]` entry in `Cargo.toml` (Tasks 1–2)
- Modify: `Cargo.toml` — version 1.0.0 + metadata (Task 3)
- Create: `CHANGELOG.md` (Task 3)
- Modify: `README.md` — quickstart against the real API (Task 4)
- Delete: `.travis.yml`; Modify: `AGENTS.md` CI section (Task 5)

---

### Task 1: `arxml-format` flag/behavior parity

py reference: `target/py-armodel/src/armodel/cli/arxml_format_cli.py:66-77` — flags:
`-v/--verbose`, `--log FILE`, `-w/--warning`, `--remove-admin-data`, `--unescape-entities`,
positional `INPUT OUTPUT`. Rust already has all six flags plus Rust-only `--no-validate`
(PR #22; keep, it gates the libxml schema validation py does not have). Deliberately
dropped (roadmap §7.1): py's `arxml_format.log` file side-effect — Rust logs to stderr only.

What must be verified to claim parity: **exit codes** (0 success; 1 parse/write failure;
argparse/clap usage error = 2) and **flag behaviors** (`-w` wires to `ParserOptions
{ warning: true }` collect-and-continue; `--remove-admin-data` actually strips every
`ADMIN-DATA` before writing; `--unescape-entities` mirrors py's `patch_xml` post-pass).

**Files:**
- Modify: `src/bin/arxml-format.rs`
- Create: `tests/golden/arxml_format/` (py-generated)
- Create: `tests/integration/cli_parity.rs`
- Modify: `Cargo.toml` (`[[test]] name = "cli_parity"`)

- [ ] **Step 1: Produce py goldens (once, at execution time)**

```bash
mkdir -p tests/golden/arxml_format
python3 target/py-armodel/src/armodel/cli/arxml_format_cli.py \
  -w tests/integration/test_files/AdminDataWhitespace.arxml \
     tests/golden/arxml_format/AdminDataWhitespace.expected.arxml
python3 target/py-armodel/src/armodel/cli/arxml_format_cli.py \
  -w --unescape-entities tests/integration/test_files/AdminDataWhitespace.arxml \
     tests/golden/arxml_format/AdminDataWhitespace.unescaped.expected.arxml
```

Expected: two golden files created; py exits 0. If a needed fixture exercises
`--remove-admin-data` (one carrying ADMIN-DATA — `AdminDataWhitespace.arxml` does), also
produce a `--remove-admin-data` golden the same way.

- [ ] **Step 2: Write the failing parity test**

```rust
//! P6 CLI parity gates: arxml-format output and exit codes vs pinned py goldens.

use std::process::Command;

fn run_format(args: &[&str]) -> (i32, Vec<u8>) {
    let output = Command::new(env!("CARGO_BIN_EXE_arxml-format"))
        .args(args)
        .output()
        .expect("arxml-format should launch");
    (output.status.code().unwrap_or(-1), output.stdout)
}

#[test]
fn format_matches_py_golden() {
    let tmp = tempfile::tempdir().unwrap();
    let out_path = tmp.path().join("out.arxml");
    let out_str = out_path.to_str().unwrap();
    let (code, _) = run_format(&[
        "--no-validate",
        "-w",
        "tests/integration/test_files/AdminDataWhitespace.arxml",
        out_str,
    ]);
    assert_eq!(code, 0);
    let produced = std::fs::read(&out_path).unwrap();
    let expected = std::fs::read("tests/golden/arxml_format/AdminDataWhitespace.expected.arxml")
        .unwrap();
    assert_eq!(produced, expected, "arxml-format output diverges from py golden");
}

#[test]
fn format_exit_1_on_missing_input() {
    let (code, _) = run_format(&["--no-validate", "-w", "tests/integration/test_files/nope.arxml", "/tmp/unused.arxml"]);
    assert_eq!(code, 1);
}

#[test]
fn format_usage_error_exits_2() {
    let (code, _) = run_format(&["--no-such-flag"]);
    assert_eq!(code, 2);
}
```

- [ ] **Step 3: Run to verify Red, fix gaps, verify Green**

Run: `cargo test --test cli_parity`
Expected: initially FAIL on whichever aspect diverges (likely exit-code wiring in
`src/bin/arxml-format.rs` — clap gives 2 on usage errors for free; ensure the parse-failure
path is `process::exit(1)` and success returns without explicit exit). Fix
`src/bin/arxml-format.rs` until green — no library changes; the binary is the only allowed
`exit()` site (code_guide).

- [ ] **Step 4: Commit**

```bash
git add -A && git commit -m "feat(p6): arxml-format flag/exit-code parity vs py goldens"
```

### Task 2: `arxml-dump` 1:1 output port

py reference: `target/py-armodel/src/armodel/cli/arxml_dump_cli.py` — `cli_main` (getopt
`--arxml`, multiple files allowed; Rust's `-a/--arxml` already matches the flag) walks
`document.getARPackages()` → `show_ar_package(indent, pkg)` recursively: sub-packages, then
`getClientServerInterfaces()` → `show_client_server_interface`, then BSW module descriptions
→ `show_bsw_module_description`. The SWC/datatype sections in py are **commented out** —
do not port them (YAGNI; 1:1 means py as pinned, Rule 0013 spirit).

Format strings to mirror exactly (from the pinned py bodies — the complete walk is four
helpers; `%s` = `" " * indent` padding):

```python
def show_ar_package(indent, ar_package):
    print("%s-%s (Pkg)" % (" " * indent, ar_package.short_name))
    for sub_package in ar_package.getARPackages():
        show_ar_package(indent + 2, sub_package)          # pre-order pass
    for cs_interface in ar_package.getClientServerInterfaces():
        show_client_server_interface(indent + 2, cs_interface)
    for child_pkg in ar_package.getARPackages():
        show_ar_package(indent + 2, child_pkg)            # second pass — port both
    for bsw_module_description in ar_package.getBswModuleDescriptions():
        show_bsw_module_description(indent + 2, bsw_module_description)

def show_client_server_interface(indent, cs_interface):
    print("%s%s" % (" " * indent, cs_interface.short_name))
    for operation in cs_interface.getOperations():
        print("%sOperation:%s" % (" " * (indent + 2), operation.short_name))
        for argument in operation.getArgumentDataPrototypes():
            print("%s         :%s (%s: %s)" % (" " * (indent + 2), argument.short_name,
                  argument.direction, argument.typeTRef.value))

def show_bsw_module_description(indent, description):
    print("%s-%s" % (" " * indent, description.short_name))
    for behavior in description.getInternalBehaviors():
        show_bsw_internal_behavior(indent + 2, behavior)

def show_bsw_internal_behavior(indent, behavior):
    print("%s-%s" % (" " * indent, behavior.short_name))
    for event in behavior.getBswModeSwitchEvents():
        print("%s-%s" % (" " * (indent + 2), event.short_name))
    for event in behavior.getBswTimingEvents():
        print("%s-%s" % (" " * (indent + 2), event.short_name))
        print("%s-%s: %s" % (" " * (indent + 4), "StartsOnEventRef", event.startsOnEventRef.value))
        starts_on_event = document.find(event.startsOnEventRef.value)      # path lookup
        print("%s-%s: %s" % (" " * (indent + 4), "StartsOnEvent", starts_on_event.short_name))
        print("%s-%s: %s" % (" " * (indent + 4), "ImplementedEntryRef", starts_on_event.implementedEntryRef.value))
        implemented_entry = document.find(starts_on_event.implementedEntryRef.value)
        print("%s-%s: %s" % (" " * (indent + 4), "ImplementedEntry", implemented_entry.short_name))
        print("%s-%s: %d" % (" " * (indent + 6), "Service Id", implemented_entry.serviceId))
```

Note `show_ar_package` walks sub-packages **twice** (pre-order print pass, then the
child-walk pass before CS interfaces' siblings) — port both passes or the golden diff will
reorder lines. The SWC/datatype sections in py are commented out — do not port them (YAGNI;
1:1 means py as pinned).

- [ ] **Step 3a: Add the path-lookup helper** — py resolves REF values via
  `AUTOSAR.getInstance().find("/Pkg/Element")`; the Rust `Document` has no counterpart.
  Add a minimal `fn find_full_name(&self, path: &str) -> Option<ElementRef>` to the binary's
  support code (or `impl Document` if it generalizes): walk `ARPackage` short-names
  segment by segment, then match the last segment against package elements by `short_name`.
  Port it TDD-style: failing unit test on a hand-built two-package document first, then the
  implementation.

- [ ] **Step 3b: Port the walk** into `src/bin/arxml-dump.rs` helper-for-helper from the
  quoted bodies. Output via `println!`; indentation via `" ".repeat(indent)`. Any py
  print-path the golden corpus cannot reach is recorded as a deferred item on the PR
  (Rule 0008 discipline).

**Files:**
- Modify: `src/bin/arxml-dump.rs`
- Create: `tests/golden/arxml_dump/CanSystem.expected.txt` (py-generated)
- Modify: `tests/integration/cli_parity.rs`

- [ ] **Step 1: Produce the py golden**

```bash
mkdir -p tests/golden/arxml_dump
python3 target/py-armodel/src/armodel/cli/arxml_dump_cli.py \
  --arxml tests/integration/test_files/CanSystem.arxml \
  > tests/golden/arxml_dump/CanSystem.expected.txt
```

- [ ] **Step 2: Write the failing golden test** (same `Command` harness as Task 1;
  `CARGO_BIN_EXE_arxml-dump`, args `["-a", "tests/integration/test_files/CanSystem.arxml"]`,
  assert stdout bytes == golden bytes, exit code 0). See it FAIL — the current Rust stub
  prints packages only.

- [ ] **Step 4: Green + commit**

```bash
cargo test --test cli_parity
git add -A && git commit -m "feat(p6): arxml-dump 1:1 report output vs py golden"
```

### Task 3: Versioning + crate metadata + CHANGELOG

**Files:**
- Modify: `Cargo.toml` (version → `1.0.0`)
- Create: `CHANGELOG.md`

- [ ] **Step 1: Bump and enrich metadata**

```toml
[package]
name = "armodel"
version = "1.0.0"
description = "Rust library to parse and generate AUTOSAR ARXML."
authors = ["melodypapa <melodypapa@github.com>"]
repository = "https://github.com/melodypapa/rust_armodel"
readme = "README.md"
edition = "2021"
license = "MIT"
keywords = ["autosar", "arxml", "automotive", "xml"]
categories = ["parser-implementations", "development-tools"]
```

(Fix the description's grammar — current text reads "to parse and generate of AUTOSAR
ARXML". `license-file` not needed; MIT + `LICENSE` file must exist — verify with
`ls LICENSE*`.)

- [ ] **Step 2: Create the changelog**

```markdown
# Changelog

## 1.0.0 — 2026-XX-XX

First crates.io release.

- Full ARXML reader/writer parity with py-armodel (pinned revision) on the 32-fixture
  corpus: zero parser warnings, byte-identical written output, structural model equality.
- `arxml-format` and `arxml-dump` CLIs at py flag/behavior parity (`--no-validate` is
  Rust-only: opt-out of the bundled XSD validation).
- Generated m2 model tree via `tools/py2rust` from the pinned py-armodel.
- XSD validation (R23-11) bundled; `arxml-validator` CLI (exit 0/1/2).

## 0.1.1

P0–P5 internal milestones (walking skeleton, py2rust converter, domain-split port,
table-driven dispatch).
```

Fill the real date at execution time.

- [ ] **Step 3: Commit**

```bash
git add -A && git commit -m "chore(p6): 1.0.0 versioning, metadata, changelog"
```

### Task 4: README quickstart against the real API

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Rewrite the quickstart** to the real public API (this is the shape the
  library actually exposes — verify against `src/lib.rs` exports before committing):

````markdown
```rust
use armodel::reader::arxml_reader::{default_options, ARXMLReader};
use armodel::writer::arxml_writer::ARXMLWriter;
use armodel::Document;

let mut document = Document::new();
ARXMLReader::new(default_options()).load("input.arxml", &mut document)?;
ARXMLWriter::new().save("output.arxml", &document)?;
```
````

- [ ] **Step 2: Verify every README code block compiles** (`cargo build` after pasting into
  `examples/` ad hoc, or `cargo test --doc` if the blocks become doctests) and commit:

```bash
git add README.md && git commit -m "docs(p6): README quickstart against the real API"
```

### Task 5: Release engineering — publish, docs, CI correction

- [ ] **Step 1: docs.rs render check**

```bash
cargo doc --no-deps 2>&1 | grep -c "warning"
```

Expected: 0 warnings on the public API (private-item leaks and broken intra-doc links are
the usual offenders — fix in the library, not the docs build).

- [ ] **Step 2: crates.io name check + publish**

```bash
cargo search armodel --limit 5     # if the plain name is taken, decide fallback now
cargo publish --dry-run
cargo publish
```

(roadmap §7.2: fallback name e.g. `rust-armodel` is decided at P6 entry — this step is that
decision point; record the outcome in CHANGELOG/README if the fallback fires.)

- [ ] **Step 3: Retire Travis, correct AGENTS.md**

```bash
git rm .travis.yml
```

Rewrite the AGENTS.md **CI** section to: GitHub Actions `quality.yml` runs build + `fmt
--check` + `clippy -D warnings` + test on stable; branch `main`. (Note: memory of this
repo's docs says Travis is stale — `.github/workflows/quality.yml` already exists, so this
is a docs deletion, not a CI migration.)

- [ ] **Step 4: Tag the release**

```bash
git tag -a v1.0.0 -m "armodel 1.0.0 — first crates.io release"
git push origin main v1.0.0
```

### Task 6: P6 exit checklist

- [ ] `arxml-format`: flags `-v/--verbose`, `--log`, `-w/--warning`, `--remove-admin-data`,
  `--unescape-entities`, positional INPUT OUTPUT — all present; exit codes 0/1/2 verified by
  `tests/integration/cli_parity.rs`; py's log-file side-effect recorded as deliberately
  dropped.
- [ ] `arxml-dump`: golden-identical report on the corpus fixture set it supports; deferred
  py print-paths listed on the PR.
- [ ] `cargo publish` succeeded (or fallback name recorded); docs.rs renders clean.
- [ ] CHANGELOG, README, version 1.0.0, tag pushed; AGENTS.md CI section corrected.
