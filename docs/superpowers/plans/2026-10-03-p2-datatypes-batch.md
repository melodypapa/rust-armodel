# P2 Datatype-Blueprints Batch Implementation Plan (batch 2 of 7)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Graduate the 9 remaining datatype-blueprint fixtures to byte-identical round-trip by porting the CompuMethod, DataConstr, KeywordSet, and ApplicationDataType families (roadmap spec §5.1 batch 2).

**Architecture:** Same as batch 1 (plan `2026-10-03-p2-infra-and-common-payload.md`): hand-port py reader/writer methods into the existing dispatch seams (`read_element_payload` / `write_ar_package_element`), model gaps fixed in `tools/py2rust` + regeneration. All Identifiable-payload plumbing already exists from batch 1.

**Tech Stack:** Rust stable / edition 2021, quick-xml 0.42, slotmap; py2rust (Python stdlib `ast`).

**Spec:** `docs/superpowers/specs/2026-10-03-p2-p6-migration-roadmap-design.md` §5. Execute on branch `p2-datatypes-batch`.

---

## Ground truth established by recon (2026-10-03)

1. **Fixtures in this batch (9):** `AUTOSAR_Datatypes`, `AUTOSAR_MOD_AISpecification_ApplicationDataType_Blueprint`, `AUTOSAR_MOD_AISpecification_CompuMethod_Blueprint`, `AUTOSAR_MOD_AISpecification_CompuMethod_LifeCycle_Standard`, `AUTOSAR_MOD_AISpecification_DataConstr_Blueprint`, `AUTOSAR_MOD_AISpecification_DataConstr_LifeCycle_Standard`, `AUTOSAR_MOD_AISpecification_KeywordSet_Blueprint`, `AUTOSAR_MOD_AISpecification_PhysicalDimension_LifeCycle_Standard`, `AUTOSAR_MOD_AISpecification_Unit_LifeCycle_Standard`.
2. **4 are already byte-identical** (verified empirically, nothing to do — only gate-list edits): `PhysicalDimension_LifeCycle_Standard`, `Unit_LifeCycle_Standard`, `CompuMethod_LifeCycle_Standard`, `DataConstr_LifeCycle_Standard`. They exercise only batch-1 families (LifeCycleInfoSet, PhysicalDimension, Unit).
3. **py itself does NOT round-trip these files byte-identically** — its writer unescapes `&quot;` in text (py's harness normalizes). Our quick-xml `BytesText` **re-escapes `"` → `&quot;`**, so our raw byte-compare matches the original fixtures. Our bar stays raw-byte-identical (stricter than py; verified quick-xml 0.42 behavior with a scratch program).
4. **Empty-element rule (critical, py `patch_xml` regex `\<([\w-]+)\/\>`):** minidom self-closes all empty elements; the regex expands only **attribute-less** ones. Net py output: empty element **with attributes → self-closing** (`<L-2 L="EN"/>`), **without attributes → expanded** (`<TAG></TAG>`). Our writer currently expands everything → must add the rule in `write_text_element`'s neighborhood. Attribute census for the batch: `L`, `DEST`, `BASE`, `INTERVAL-TYPE`, LIST `TYPE` — so the rule matters for `<L-2 L="EN"/>` (KeywordSet has ~thousands of empty L-2) and any attribute-carrying empty element.
5. **Attribute census (whole batch):** only `L`(4601), `DEST`(2583), `BASE`(2370), `INTERVAL-TYPE`(2148), `TYPE`(41), plus namespaces. No MASK/VALIDITY/SHORT-LABEL-attr/HELP-ENTRY/SUP/SUB — those py branches stay untriggered (warning path or absent in model).
6. **Model gaps found:**
   - `TYPE-TREF` collapses to `Option<String>` but fixtures carry `BASE`/`DEST` → same fix as batch-1 RefType: remove `TRefType` from `typemap.PRIMITIVES` (arena `t_ref_types` + `compare_t_ref_type` already exist).
   - Everything else exists: CompuMethod/Compu/CompuScales/CompuScale/CompuScaleConstantContents/CompuScaleRationalFormula/CompuConst/CompuConstTextContent/CompuNominatorDenominator/CompuRationalCoeffs, Limit (interval_type + value), DataConstr/DataConstrRule/InternalConstrs/PhysConstrs/ScaleConstr, Keyword/KeywordSet, ApplicationDataType chain (AutosarDataType→ApplicationDataType→ApplicationCompositeDataType→…), ApplicationArrayElement, ApplicationRecordElement, SwDataDefProps (variants/conditional have no model class — py reads/writes through the wrapper; the Rust side does the same).
   - Heterogeneous fields are `Option<ElementRef>`: `Compu.compu_content` (variant `CompuScales`), `CompuScale.compu_scale_contents` (variants `CompuScaleConstantContents`/`CompuScaleRationalFormula`), `CompuConst.compu_const_content_type` (variant `CompuConstTextContent`). Check `ElementRef` variant names before compiling.
7. **Emission orders that matter** (py writer bodies, verified this session):
   - `writeCompuMethod`: SHORT-NAME/LONG-NAME/DESC/CATEGORY/INTRODUCTION/ADMIN-DATA (chain), DISPLAY-FORMAT, UNIT-REF, COMPU-INTERNAL-TO-PHYS, COMPU-PHYS-TO-INTERNAL.
   - `setCompu`: S/T attrs, COMPU-SCALES, COMPU-DEFAULT-VALUE. `setCompuScales` → per `writeCompuScale(key="COMPU-SCALE")`: S/T, A2L-DISPLAY-TEXT, COMPU-INVERSE-VALUE, SHORT-LABEL, SYMBOL, DESC, MASK, LOWER-LIMIT, UPPER-LIMIT, contents. Contents: constant → `COMPU-CONST` + `VT` child; rational → `COMPU-RATIONAL-COEFFS` + `COMPU-NUMERATOR` then `COMPU-DENOMINATOR` (each wrapping `V` children in list order).
   - `setChildLimitElement`: S/T attrs, `INTERVAL-TYPE` attr, text. Reader (`getChildLimitElement`): S/T, INTERVAL-TYPE (or None), text.
   - `writeDataConstr`: chain, then DATA-CONSTR-RULES wrapper; per rule: S/T, CONSTR-LEVEL, **PHYS-CONSTRS, INTERNAL-CONSTRS** (writer order — phys first, unlike the reader). Per constrs: S/T, LOWER-LIMIT, UPPER-LIMIT (+MAX-GRADIENT/MAX-DIFF/MONOTONY/SCALE-CONSTRS/UNIT-REF in py's exact orders — see `writeDataConstr` neighbors in py; port only fields the Rust model has).
   - `writeKeyword`: chain, ABBR-NAME, CLASSIFICATIONS wrapper + CLASSIFICATION children. `writeKeywordSet`: chain (ARElement), KEYWORDS wrapper + KEYWORD children.
   - Datatypes: chain = `writeIdentifiable` + `setSwDataDefProps("SW-DATA-DEF-PROPS")` **last within the chain** (py `writeDataPrototype`), then per-class extras: primitive (nothing); array (DYNAMIC-ARRAY-SIZE-PROFILE, ELEMENT); record (ELEMENTS wrapper of APPLICATION-RECORD-ELEMENT); implementation (DYNAMIC-ARRAY-SIZE-PROFILE, IS-STRUCT-WITH-OPTIONAL-ELEMENT, SUB-ELEMENTS, symbol-props?, TYPE-EMITTER — port the model-backed subset).
   - `setSwDataDefProps`: wrapper `SW-DATA-DEF-PROPS` (S/T) → `SW-DATA-DEF-PROPS-VARIANTS` → `SW-DATA-DEF-PROPS-CONDITIONAL` (S/T) → children in py order; port the model-backed subset: ANNOTATIONS(skip—no fixture), DISPLAY-PRESENTATION, BASE-TYPE-REF, DATA-CONSTR-REF, COMPU-METHOD-REF, SW-ADDR-METHOD-REF, SW-ALIGNMENT, SW-CALIBRATION-ACCESS, IMPLEMENTATION-DATA-TYPE-REF, STEP-SIZE, SW-INTENDED-RESOLUTION, UNIT-REF, DISPLAY-FORMAT.
   - `writeApplicationRecordElement`: chain(prototype: TYPE-TREF after SwDataDefProps), IS-OPTIONAL.
   - `setApplicationArrayElement` (`ELEMENT` wrapper): prototype chain, ARRAY-SIZE-HANDLING, ARRAY-SIZE-SEMANTICS, INDEX-DATA-TYPE-REF, MAX-NUMBER-OF-ELEMENTS.
8. **Reader orders** (py, find-based so wrapper-relative): `readCompuMethod` 8484 (chain, DISPLAY-FORMAT, UNIT-REF, COMPU-INTERNAL-TO-PHYS, COMPU-PHYS-TO-INTERNAL via `getCompu` 8466 → `getCompuScales` → `readCompuScale` 8451 → `readCompuScaleContents`/`readCompuConst` 8419/`readCompuRationCoeffs` 8435); `readDataConstr` 8791 → `readDataConstrRule` 8781 → `readInternalConstrs` 8740 / `readPhysConstrs` 8766 / `readScaleConstr` 8754; `readKeyword*` 11887–11906; `readApplicationPrimitiveDataType` 7147 (→ `readAutosarDataType` 7139 = chain + SwDataDefProps via `getSwDataDefProps` 7022), `readApplicationArrayDataType` 8949 (+`readApplicationArrayElement` 8937), `readApplicationRecordDataType` 7165 (+Elements 7156, Element 7151), `readImplementationDataType` 7189 (+SubElements/Element 7180/7171).
9. **Batch-1 lessons applied:** generated classes forward all base accessors at top level (no `base()` hops in parser/writer); clippy rejects >7-arg functions (bundle into structs); `writeIdentifiable` element order is SHORT-NAME, LONG-NAME, DESC, CATEGORY, INTRODUCTION, ADMIN-DATA; graduation touches three lists (`WARNING_FREE_SOURCES`, `FORMAT_SOURCES`, `P2_P4_PENDING` in `all_fixtures.rs`); `cargo fmt` before clippy; pipefail.
10. **Keyword/KeywordSet/record-elements/array-elements are NOT AR-PACKAGE elements** — no dispatch arms; nested readers insert directly into arenas (`document.keywords`, `document.keyword_sets`… check exact arena names). Only KEYWORD-SET gets an `ElementRef` arm (it's a package element in KeywordSet_Blueprint). Same for the datatypes: the five AR-PACKAGE-element tags needing arms are COMPU-METHOD, DATA-CONSTR, KEYWORD-SET, APPLICATION-{PRIMITIVE,ARRAY,RECORD}-DATA-TYPE, IMPLEMENTATION-DATA-TYPE.

## Tasks

### Task 0: TRefType model fix + regeneration

Same shape as batch-1 Task 0: remove `"TRefType"` from `PRIMITIVES` in `tools/py2rust/typemap.py` (keep the explanatory comment), regenerate, extend `tools/py2rust/tests/test_model_gaps.py` with an assertion that `ApplicationCompositeElementDataPrototype.type_t_ref` is `Option<TRefTypeId>`, then `cargo fmt && cargo clippy --all-targets -- -D warnings && cargo test` and fix fallout in hand-written code only. Commit: `feat(py2rust): TRefType arena fields (P2 batch 2 Task 0)`.

- [ ] Test first (extend test_model_gaps.py), see it fail
- [ ] typemap edit + regenerate + `--check`
- [ ] Gates green, commit

### Task 1: empty-element self-closing rule in the writer

- [ ] Extend the `write_text_element` neighborhood (`src/writer/abstract_arxml_writer.rs`): when `text` is `None` **or empty**, emit `Event::Empty` if the prepared `BytesStart` carries attributes, else the existing expanded Start/End pair. Non-empty text unchanged. Unit test in the same file: `<L-2 L="EN"/>` (attr → self-closing) and `<SD></SD>`-style (no attr → expanded), plus a non-empty case.
- [ ] `cargo test` — all 11 FORMAT_SOURCES must still pass (they contain no attribute-carrying empty elements; if one fails, that fixture DID contain one and py's output self-closes it too — investigate before proceeding).
- [ ] Commit: `feat(writer): py empty-element rule — self-closing only with attributes (P2 batch 2 Task 1)`.

### Task 2: Limit reader/writer helpers

- [ ] Reader: `get_child_limit_element(element, key, document) -> Result<Option<LimitId>, ParseError>` in `arxml_parser.rs` (S/T via attrs, INTERVAL-TYPE → `IntervalTypeEnum::try_from` with py's warning on unsupported, text via `node.text`), arena `document.limits` (verify name).
- [ ] Writer: `write_limit_element(writer, key, limit_id, document)` mirroring `setChildLimitElement` (S/T attrs, INTERVAL-TYPE attr, text).
- [ ] Unit tests for both (interval-type round-trip). Commit: `feat(p2): Limit read+write helpers (P2 batch 2 Task 2)`.

### Task 3: Compu family (COMPU-METHOD arm) → graduates CompuMethod_Blueprint, AUTOSAR_Datatypes

- [ ] Reader: `get_compu_const` (COMPU-CONST/VT → CompuConstTextContent; `getCompuConst` for COMPU-INVERSE-VALUE/COMPU-DEFAULT-VALUE wrappers), `get_compu_scales`/`read_compu_scale` (fields per ground truth 7; contents via ElementRef variants), `get_compu` (COMPU-SCALES + COMPU-DEFAULT-VALUE), `read_compu_method` (chain + DISPLAY-FORMAT + UNIT-REF + two Compus). `readCompuNominatorDenominator`: collect `V` texts in order.
- [ ] Writer: mirror per ground truth 7 (`set_compu`, `set_compu_scales`, `write_compu_scale`, contents by ElementRef variant, `write_compu_method`). Dispatch arms `ElementRef::CompuMethod`.
- [ ] Unit test: round-trip a COMPU-METHOD with rational-coeffs and text-const scales through parse→serialize and compare against a hand-written expected XML string.
- [ ] Graduate `AUTOSAR_MOD_AISpecification_CompuMethod_Blueprint.arxml` + `AUTOSAR_Datatypes.arxml` in all three lists; `arxml-format` + `diff` to debug. Commit: `feat(p2): Compu family; graduate 2 fixtures (P2 batch 2 Task 3)`.

### Task 4: DataConstr family → graduates the 2 DataConstr fixtures

- [ ] Reader: `read_data_constr` (chain + DATA-CONSTR-RULES/RULE: CONSTR-LEVEL, INTERNAL-CONSTRS, PHYS-CONSTRS, SCALE-CONSTRS/SCALE-CONSTR with DESC/limits/short-label/VALIDITY).
- [ ] Writer: `write_data_constr` per ground truth 7 (rule order: CONSTR-LEVEL, PHYS-CONSTRS, INTERNAL-CONSTRS).
- [ ] Graduate both DataConstr fixtures (three lists); verify. Commit: `feat(p2): DataConstr family; graduate 2 fixtures (P2 batch 2 Task 4)`.

### Task 5: KeywordSet family → graduates KeywordSet_Blueprint

- [ ] Reader: KEYWORDS/KEYWORD (chain payload + ABBR-NAME + CLASSIFICATIONS/CLASSIFICATION texts) inserting into `document.keywords`; `read_keyword_set` chain. Dispatch arm `ElementRef::KeywordSet`.
- [ ] Writer: mirror ground truth 7. The empty `<L-2 L="EN"/>` elements exercise Task 1's rule.
- [ ] Graduate KeywordSet_Blueprint (three lists); verify. Commit: `feat(p2): KeywordSet family; graduate KeywordSet_Blueprint (P2 batch 2 Task 5)`.

### Task 6: ApplicationDataType family → graduates ApplicationDataType_Blueprint

- [ ] Reader: `get_sw_data_def_props` (wrapper walk, model-backed subset per ground truth 7 — unknown conditional children py reads but the model lacks are skipped silently **only if** no fixture carries them; TYPE-EMITTER lives on ImplementationDataType, not here), then `read_application_primitive/record/array_data_type` + record-elements + array-element per ground truth 8. Dispatch arms for the three tags.
- [ ] Writer: `set_sw_data_def_props` + the three emitters + record/array element emitters per ground truth 7.
- [ ] Graduate ApplicationDataType_Blueprint (three lists); verify. Commit: `feat(p2): ApplicationDataType family; graduate blueprint (P2 batch 2 Task 6)`.

### Task 7: ImplementationDataType (AUTOSAR_Datatypes already graduated in Task 3 — port remaining fields if its diff shows gaps)

- [ ] Run `arxml-format` on AUTOSAR_Datatypes; if byte-identical after Task 3, this task is a no-op (checklist note only). If TYPE-EMITTER/SUB-ELEMENTS diffs appear, port `read_implementation_data_type`/writer per ground truth 8. Commit if code changed: `feat(p2): ImplementationDataType fields (P2 batch 2 Task 7)`.

### Task 8: graduate the 4 free wins + close-out

- [ ] Add `PhysicalDimension_LifeCycle_Standard`, `Unit_LifeCycle_Standard`, `CompuMethod_LifeCycle_Standard`, `DataConstr_LifeCycle_Standard` to all three lists (they pass already per ground truth 2).
- [ ] Regenerate checklist (`--emit-port-checklist`), full gate: `cargo build && cargo fmt --all -- --check && cargo clippy --all-targets -- -D warnings && cargo test`.
- [ ] Commit: `chore(p2): graduate 4 LifeCycle fixtures; regenerate checklist (P2 batch 2 Task 8)`.
- [ ] Push branch, open PR closing a new tracking issue; note deferred items (MASK, VALIDITY, MONOTONY, ANNOTATIONS-in-props, value-specs) on the PR body.

**Batch 2 done when:** 9 fixtures byte-identical + warning-free (11 → 20 graduated), 32/32 model-equality stays green, checklist regenerated.
