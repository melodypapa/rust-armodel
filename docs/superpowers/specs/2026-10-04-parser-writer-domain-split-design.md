# Parser/Writer Domain-Split Design

Date: 2026-10-04
Status: Approved (presented 2026-10-04; plan: `docs/superpowers/plans/2026-10-04-parser-writer-domain-split.md`)
Scope: structural decomposition of the hand-written parser/writer for the P2–P4 mass port

## Problem

`src/reader/arxml_reader.rs` (2,318 lines, 39 methods — today `src/parser/…`, renamed to
`reader` by this refactor) and `src/writer/arxml_writer.rs` (2,240 lines, 41 methods) grow
one method per py method. At full parity the reference shapes are py's 16,253-line /
1,218-read-method parser and 15,889-line / 1,438-emitter writer: each file becomes a
multi-thousand-method impl block — unreviewable, merge-hostile, and past the size an agent
can edit reliably. With only 41/2,438 methods ported, this is the cheapest moment to
restructure.

## Decision — domain-module split (P5-shaped), pure-move migration

Terminology first: the reader layer is renamed to match the port vocabulary everything
else already uses (port-checklist columns, skill rules, `readXxx` → `read_xxx`): `src/parser/`
→ `src/reader/`, `ARXMLParser` → `ARXMLReader`, `ParserOptions` → `ReaderOptions`,
`abstract_arxml_parser` → `abstract_arxml_reader`. `ParseError` keeps its name (locked by
code_guide §7), and prose references to py's own `ARXMLParser` class stay as-is. The crate
is 0.x with no external consumers, so the public-API rename is deliberate and cheap now.

Keep `ARXMLReader` / `ARXMLWriter` exactly as they are otherwise; split each across domain
modules. Rust allows multiple `impl Type` blocks anywhere in the crate, so a domain file is
just `impl ARXMLReader { … }` holding one AUTOSAR domain's methods. Module paths stay
identical after the rename (`crate::reader::arxml_reader` → `arxml_reader/mod.rs`), so
lib.rs re-exports (same names, new spelling), hand-written call sites, and the harness do
not change structurally.

```
src/reader/arxml_reader/
  mod.rs            struct + options + load/load_from_reader + tag walk +
                    dispatch seams (delegating arms) + free fns (factory, setters, xsd_to_version)
  common.rs         abstract-level helpers: read_ar_object, read_xml_space,
                    read_identifiable_payload + IdentifiablePayload
  admin_data.rs     documentation.rs      compu_method.rs    data_constr.rs
  keyword.rs        datatypes.rs          base_types.rs     collection.rs
  life_cycle.rs     physical_dimension.rs unit.rs
src/writer/arxml_writer/   same layout with emitters
```

Dispatch stays in `mod.rs` but arms become one-line delegations to the owning module
(`ElementRef::CompuMethod(id) => compu_method::read_payload(self, …)` where a free
function form is needed; most arms keep calling `self.read_xxx` directly — same
trait-less mechanics, just another module). New batches register arms in one place;
the match never grows bodies.

### Alternatives rejected

- **Capability traits per domain** — one implementing type per trait adds indirection
  with no payoff; code_guide §3 already rejects emulating structure with traits.
- **Pull P5 forward (generated dispatch tables now)** — re-litigates the locked P2–P4
  strategy mid-batch and migrates dispatch before the harness covers the corpus. The
  split below is *P5-shaped*: each domain's methods are already grouped, so the later
  table-driven swap per domain is a small mechanical change, not a rewrite.

## Rules (to be added to docs/code_guide.md §8)

1. **Placement:** a method lives in the domain module of the family it reads/writes;
   shared base helpers live in `common.rs`. Placement is a pure move — methods stay on
   the same struct, so **no call site changes, ever**.
2. **Visibility:** methods default private (visible to the defining module; child modules
   reach the struct's private fields since privacy extends to descendants). A method
   called from a sibling domain module is `pub(super)` — never `pub(crate)`/`pub`.
3. **Tests move with their domain.** The harness (`tests/integration/…`) is untouched and
   remains the only contract.
4. **mod.rs budget:** struct + walk + dispatch only; past ~500 lines something belongs in
   a domain module.

## Migration mechanics

Pure cut-paste moves, one domain per commit (`refactor(parser): move compu_method
domain to its own module`), full gates after each (`cargo build && cargo fmt --all &&
cargo clippy --all-targets -- -D warnings && cargo test`), no method bodies edited during
a move. Each domain file starts from the same `use super::*`-free explicit-import
recipe given in the plan; the compiler names any import or `pub(super)` a move needs.
`docs/port_checklist.md` regeneration must stay green throughout (the scanner matches
fn names anywhere under `src/`; verified by re-running it after the first split).

## Non-goals

- Public API structurally unchanged — the deliberate exception is the reader rename
  (`ARXMLParser`→`ARXMLReader`, `ParserOptions`→`ReaderOptions`, `src/parser/`→`src/reader/`);
  nothing else in the surface moves (`ARXMLWriter`/options/`ParseError` re-exports stay).
- No trait hierarchies; no py2rust changes beyond a scanner fix if the first split
  exposes one; no dispatch-table generation (that is P5).
- No fixture, graduation-list, or model changes.
