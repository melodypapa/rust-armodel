# Sync todo: Group 1 — Framework & core, PortInterface basics

Input: `Group 1 — Framework & core, PortInterface basics` of `docs/examples/sync_class_groups.md` · Generated: 2026-08-30 · Queue order = row order
(resume = first class row still `[ ]`; all class rows `[x]` = sync finished — Rule 0009.4)

## Queue (dependency-first)

- [x] `ARObject` (tracker input · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 6.1 · commit 1be5c3b)
  - Merge note (2026-10-06): the PR #20 domain split moved this row's two S/T tests into the
    reader/writer `common.rs` test modules (ARXMLParser→ARXMLReader renamed); the empty-with-attrs
    ADMIN-DATA writer fix re-applied to `src/writer/arxml_writer/admin_data.rs`. lib suite 30 tests.
  - Note: py handles this class in the ABSTRACT parser/writer (`readARObject` abstract_arxml_parser.py:493,
    `writeARObject` abstract_arxml_writer.py:81 — S→checksum, T→timestamp, emit order S then T), so it has no
    generated port-checklist rows. Rust: `read_ar_object` (arxml_parser.rs:211), allocation-loop generics
    (arxml_parser.rs:471-476 S/T/UUID/CATEGORY per package element), `write_ar_object_attributes`
    (arxml_writer.rs:160) + `write_identifiable_attributes` from getters. Fixtures: only T exercised
    (Unit_Standard AR-PACKAGE, graduated; CanSystem CAN-COMMUNICATION-CONTROLLER → batch 5); no `S=` in corpus.
  - [x] Step 1 — Extract py reference & Rust model shape
  - [x] Step 2 — Write failing reader test + graduate fixtures (Red) — reader test passed immediately (implementation
    already ported 1:1); graduation N/A: no fixture graduates from ARObject alone (Unit_Standard already graduated;
    CanSystem needs its own batch)
  - [x] Step 3 — Port the reader (Green) — no code change; `read_ar_object` + allocation-loop S/T verified against
    py body line-by-line; `getLifeCyclePeriod` correctly reads no S/T in py and likewise not in Rust
  - [x] Step 4 — Extract writer order; write failing writer test (Red) — py order S then T; writer test seen FAILING:
    empty ADMIN-DATA carrying S/T emitted expanded, py self-closes (minidom + patch_xml rule)
  - [x] Step 5 — Port the writer (Green) — fix: `write_admin_data` routes childless ADMIN-DATA through
    `write_text_element` (which owns the self-closing-iff-attributes rule); writer test green
  - [x] Step 6 — Harness to green (zero warnings + byte-identical) — full `cargo test` green (30 lib + all
    integration gates); no fixture regression (no fixture carries a childless ADMIN-DATA, so byte outputs unchanged)
  - [x] Step 7 — Regenerate the port checklist — ran `--emit-port-checklist` + `--check`: OK; checklist unchanged
    (ARObject methods live in py's ABSTRACT parser/writer, outside the generator's scan — family ledger is the
    py source + this row's tests)
  - [x] Step 8 — Model gaps & deferrals — no model gaps (generated ARObject already carries checksum/timestamp).
    Deferrals: (1) `S` never exercised by the corpus — unit-test coverage only; (2) CanSystem
    CAN-COMMUNICATION-CONTROLLER T round-trip lands with its batch (read path already via allocation loop);
    (3) empty-with-attrs self-closing for other structural writers (e.g. `write_sdg` with GID, no children)
    deferred to the writer-fidelity batch that first needs it — no fixture carries one
  - [x] Step 9 — Verify (9a) + confirm (9b) — 9a gates green (fmt, clippy -D warnings, 8 test suites, py2rust
    --check); 9b port-parity checklist presented and confirmed by the user (github-workflow delivery)

- [x] `ARElement` (tracker input · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 4.3 · commit 9c8dd7a)
  - Note (Step 1): py `readARElement` (arxml_parser.py:1793) and `writeARElement`
    (arxml_writer.py:1889) are pure pass-throughs to the Identifiable chain; ARElement is
    abstract (ARElement : PackageableElement, no own fields — generated struct
    ar_package.rs:2109, no arena, no ElementRef variant). The port = the chain level plus
    the call-graph reroute: among ported domains py routes ONLY KeywordSet (reader 11908 +
    writer 10512) and PhysicalDimension (writer 12574) through the ARElement level;
    SwBaseType/CompuMethod/DataConstr/Unit/Collection/Keyword call the Identifiable
    helpers directly (py is asymmetric per class — e.g. PhysicalDimension reader 13990 →
    readIdentifiable but writer 12574 → writeARElement; ported as pinned, Rule 0013).
    Chain ground truth: writeIdentifiable = S/T attrs + SHORT-NAME + LONG-NAME + UUID attr
    (order S,T,UUID) + DESC + CATEGORY + INTRODUCTION + ADMIN-DATA. Fixtures exercising
    the rerouted tags (KeywordSet_Blueprint, PhysicalDimension_Standard,
    PhysicalDimension_LifeCycle_Standard) are already graduated; graduation N/A (abstract
    level, no ARELEMENT tag exists). No model gaps.
  - [x] Step 1 — Extract py reference & Rust model shape
  - [x] Step 2 — Write failing reader test + graduate fixtures (Red) — direct-call reader test on a
    KEYWORD-SET fragment seen FAILING to compile (`read_ar_element` not found); graduation N/A:
    abstract chain level, no ARELEMENT tag — the rerouted tags' fixtures are already graduated
  - [x] Step 3 — Port the reader (Green) — `read_ar_element` added to reader common.rs as the
    ARElement chain level (pass-through to `read_identifiable_payload`); `read_keyword_set`
    rerouted through it (py's only ported-domain reader caller); test green. Note: first
    fragment draft used SD-child DESC / bare-P shapes — py reads L-2 direct text and P→L-1
    (readLOverviewParagraph arxml_parser.py:2104ff); fragment corrected to py's shapes
  - [x] Step 4 — Extract writer order; write failing writer test (Red) — order from py
    writeIdentifiable body (arxml_writer.py:1874): attrs S,T,UUID then SHORT-NAME, LONG-NAME,
    DESC, CATEGORY, INTRODUCTION, ADMIN-DATA (≠ reader order: reader's SHORT-NAME/UUID/
    CATEGORY belong to the allocation loop); two direct-call tests seen FAILING to compile
    (`write_ar_element`/`IdentifiableAttrs` not found)
  - [x] Step 5 — Port the writer (Green) — `write_ar_element` (writer common.rs: attrs + Start +
    SHORT-NAME + `write_identifiable_parts`) with `IdentifiableAttrs` bundle (clippy ≤7 args);
    `write_keyword_set` + `write_physical_dimension` rerouted (py writeARElement callers);
    both tests green
  - [x] Step 6 — Harness to green (zero warnings + byte-identical) — full `cargo test` green
    (55 lib + all integration gates); the reroute is byte-neutral, all graduated fixtures
    unregressed (KeywordSet_Blueprint / PhysicalDimension_* among them)
  - [x] Step 7 — Regenerate the port checklist — `--emit-port-checklist` + `--check` OK
    (0 drifted); `readARElement`/`writeARElement` rows flipped [x]; port count 43/2438
  - [x] Step 8 — Model gaps & deferrals — no model gaps (no py2rust change; --check green).
    Deferrals: (1) ANNOTATIONS / VARIATION-POINT / SHORT-NAME-FRAGMENTS remain the
    Identifiable/Referrable chain's existing tracked deferrals (see
    `read_identifiable_payload` doc) — ARElement adds none of its own; (2) py's per-class
    call-graph asymmetry (SwBaseType/CompuMethod/DataConstr/Unit/Collection/Keyword call the
    Identifiable helpers directly) is ported as pinned (Rule 0013) — future families for
    those classes must keep the direct call, not "fix" them onto the ARElement level
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ReferenceBase` (member type of `ARPackage.referenceBase` · Rule 0016.4 stub — blocks ARPackage 9b stamp per Rules 0001.10/0012.1 · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 4.14, p.72)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `MultilanguageReferrable` (heritage-chain parent of Identifiable · unstamped Rule 0001.10 stub · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 4.11, content md l.1777–1793)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `HwPin` (R23-11 markdown · AUTOSAR_CP_TPS_ECUResourceTemplate · Table 2.7 · **uuid-move blocker — see the "uuid move work order" section below** · heritage fix: spec Base = ARObject, HwDescriptionEntity, Identifiable, MultilanguageReferrable, Referrable → most-derived direct base Identifiable; code was `HwPin(HwDescriptionEntity)` (Referrable-only))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `HwPinGroup` (R23-11 markdown · AUTOSAR_CP_TPS_ECUResourceTemplate · Table 2.5 · **uuid-move blocker — see the "uuid move work order" section below** · heritage fix: spec Base = ARObject, HwDescriptionEntity, Identifiable, MultilanguageReferrable, Referrable; code was `HwPinGroup(HwDescriptionEntity)` — 1 live UUID in CanSystem.arxml (CAN1))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `HwType` (R23-11 markdown · AUTOSAR_CP_TPS_ECUResourceTemplate · Table 2.3 · **uuid-move blocker — see the "uuid move work order" section below** · heritage fix: spec Base = ARElement…Identifiable; code was `HwType(HwDescriptionEntity)` — 1 live UUID in CanSystem.arxml (AnalogInType))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `HwElement` (R23-11 markdown · AUTOSAR_CP_TPS_ECUResourceTemplate · Table 2.4 · after `HwType` (ref `hwType`) and `HwPinGroup` (aggr `hwPinGroup`) · **uuid-move blocker — see the "uuid move work order" section below** · heritage fix: spec Base = ARElement…Identifiable; code was `HwElement(HwDescriptionEntity)` — 3 live UUIDs in CanSystem.arxml (AI_KL15, AI_KL30, DemoECU)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `FirewallRule` (R23-11 markdown · AUTOSAR_CP_TPS_SystemTemplate · Table 6.236, p.585 (pdf_page.py verified) · **uuid-move blocker** · heritage: `class FirewallRule(ARElement)` already applied (KEEP) — RE-SCOPE (corrected 2026-08-31 via markdown/XSD arbitration — the markdown splits Table 6.236 across pages: l.15346-15351 carries Class/Note/Base, l.15353 bucketSize, l.15363-15373 the 9 rule attrs; XSD group FIREWALL-RULE l.59016-59101 confirms ownership): current body has fabricated `destRefs`/`srcRefs` (`List[RefType]`) NOT in Table 6.236/XSD — REMOVE; replace with the **10** spec attributes from Table 6.236: `bucketSize` (0..1 attr, PositiveInteger — was missing from earlier RE-SCOPE), `dataLinkLayerRule` (0..1), `ddsRule` (0..1), `doIpRule` (0..1), `networkLayerRule` (0..1), `payloadBytePatternRule` (* via PAYLOAD-BYTE-PATTERN-RULES), `refillAmount` (0..1 attr, PositiveInteger), `someipRule` (0..1), `someipSdRule` (0..1), `transportLayerRule` (0..1) — **user decision 2026-08-31: the 14 markdown-only member classes (DataLinkLayerRule, DdsRule, DoIpRule, PayloadBytePatternRulePart, IcmpRule, TcpRule, UdpRule, SomeipProtocolRule, SomeipSdRule, Ipv4Rule, Ipv6Rule, PayloadBytePatternRule, NetworkLayerRule, TransportLayerRule — no Class table in PDF/markdown, unverifiable) are SKIPPED and removed from this queue**; DataLinkLayerRule (c5a1b0de) and DdsRule (86699dd2) were already synced markdown-minimal in Firewall/__init__.py; the other 12 member types are NOT implemented — model FirewallRule's aggregates with attribute-name placeholders per Table 6.236 (full member attribute defs remain a deviation). Reader/writer (readFirewallRule/writeFirewallRule) built when syncing this class.
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `PortInterfaceBlueprintMapping` (XSD-only · no R23-11/R4.3.1 PDF/markdown Class table · member type of `BlueprintMappingSet.blueprintMap` wrapper `BLUEPRINT-MAPS` choice · AUTOSAR_00052.xsd complexType `PORT-INTERFACE-BLUEPRINT-MAPPING` l.92477, group l.92440) — needed so `BlueprintMappingSet`'s reader/writer covers all three `BLUEPRINT-MAPS` choice elements (closes the 9b deviation)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `PortPrototypeBlueprintMapping` (XSD-only · no R23-11/R4.3.1 PDF/markdown Class table · member type of `BlueprintMappingSet.blueprintMap` wrapper `BLUEPRINT-MAPS` choice · AUTOSAR_00052.xsd complexType `PORT-PROTOTYPE-BLUEPRINT-MAPPING` l.93034, group l.92997) — needed so `BlueprintMappingSet`'s reader/writer covers all three `BLUEPRINT-MAPS` choice elements (closes the 9b deviation)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BlueprintMappingSet` (R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 3.1 · after `AtpBlueprintMapping` (aggr `blueprintMap`) · **uuid-move blocker — see the "uuid move work order" section below** · heritage fix: spec Base = ARElement…Identifiable —; code was `BlueprintMappingSet(ARObject)` (CommonStructure/StandardizationTemplate/BlueprintMapping.py:8))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ConstantSpecificationMappingSet` (R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.119 · heritage fix: spec Base = ARElement…Identifiable —; code was `ConstantSpecificationMappingSet(ARObject)` (CommonStructure/Constants/__init__.py:804))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `StandardNameEnum` (R23-11 markdown · AUTOSAR_FO_TPS_StandardizationTemplate · Table 2.1 · member type of `StructuredReq.appliesTo` · `AREnum`)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `StructuredReq` (R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.31 · **uuid-move blocker — see the "uuid move work order" section below** · heritage fix: spec Base = ARObject, DocumentViewSelectable, Identifiable, MultilanguageReferrable, Paginateable, Referrable, Traceable —; code was `StructuredReq(ARObject)` (MSR/Documentation/BlockElements/RequirementsTracing.py:123))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `TraceableText` (R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.30, p.313 · after `StructuredReq` (same Base closure) · **uuid-move blocker — see the "uuid move work order" section below** · heritage fix —; code was `TraceableText(ARObject)` (RequirementsTracing.py:58))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `Identifiable` (heritage-chain parent of CollectableElement/AtpBlueprint/AtpBlueprintable/ARPackage · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 4.4, content md l.1650–1695)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `CollectableElement` (direct spec base of ARPackage + PackageableElement · Rule 0016.4 wrong-base stub — prerequisite for the ARPackage heritage fix · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 13.3, class table body md l.10589+)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `PackageableElement` (child of CollectableElement · STAMPED R23-11 — **drift pass only** per Rule 0012.3: heritage fix · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 4.2, p.54)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ARPackage` (tracker input · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 4.1)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `AUTOSAR` (tracker input · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table E.1 (Appendix E "Mentioned Class Tables") · **heritage check resolved 2026-09-02**: spec Base = **ARObject** (Table E.1) → Step 3 decision = **restructure to ARObject** (user choice); `CollectableElement` element-registry reimplemented on the root)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `FileInfoComment` (tracker input · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 2.1)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `AutoCollectEnum` (Rule 0016.4 stub discovered during Collection Step 1, 2026-09-02 — wrong base `Enum` (not `AREnum`) + fabricated literals OFF/ON/AUTO vs Table 13.2 refAll/refNone/refNonStandard; zero consumers outside ElementCollection.py — rename safe; queued ahead of Collection per dependency-first)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `Collection` (tracker input · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 13.1)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `AtpType` (tracker input · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 5.6) — abstract shell (no own attributes, no own XML element); heritage `AtpType(AtpClassifier, ABC)` already matched spec; only deviation was a paraphrased class docstring, now verbatim Table 5.6 Note; Steps 5/6 N/A
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `AtpPrototype` (tracker input · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 5.4) — **DONE; stamped (AbstractStructure.py:165) after 9b confirmation 2026-09-02
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `PortPrototype` (heritage drift pass · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 3.2 · AtpPrototype subclass; needs `AtpBlueprintable` parallel base re-added after AtpPrototype dropped it)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DataPrototype` (heritage drift pass · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.28, p.306 · AtpPrototype subclass; confirm loss of AtpBlueprintable is spec-correct)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ModeDeclarationGroupPrototype` (heritage drift pass · R23-11 markdown · AUTOSAR_CP_TPS_BSWModuleDescriptionTemplate · Table 4.9 (R4.3.1 Table 5.10; R23-11 re-number) · AtpPrototype subclass; confirm loss of AtpBlueprintable is spec-correct)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `RootSwCompositionPrototype` (heritage drift pass · R23-11 markdown · AUTOSAR_CP_TPS_SystemTemplate · Table 4.1 · AtpPrototype subclass; confirm loss of AtpBlueprintable is spec-correct)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `SwComponentPrototype` (heritage drift pass · R23-11 markdown · **AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 3.11, p.77** (row's `DiagnosticExtractTemplate Table 8.x` citation was wrong — resolved in Step 1: the class's Table 3.11 Package = `M2::AUTOSARTemplates::SWComponentTemplate::Composition` matches the source file; the other markdown hits are SystemTemplate Table 11.4 / FO AbstractPlatformSpecification Table 3.4, different classes) · AtpPrototype subclass; confirm loss of AtpBlueprintable is spec-correct)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `AtpStructureElement` (tracker input · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 5.5)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `AtpDefinition` (tracker input · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 11.3) — abstract shell (no own attributes, no own XML element); heritage fix `Identifiable` → `Referrable` (spec most-derived base); class docstring verbatim Table 11.3 Note; Steps 5/6 N/A
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BlueprintPolicy` (member type of AtpBlueprint.blueprintPolicy — the Rule 0001.10 placeholder that blocked AtpBlueprint's stamp; queued per user 2026-09-03 · R23-11 markdown · AUTOSAR_FO_TPS_StandardizationTemplate · **Table C.18, p.164** — CORRECTED: todo said C.17/p.163; appendix-letter table so pdf_page.py can't index it, located via pypdf; attributeName (String, 1, attr) confirmed in markdown body + XSD group BLUEPRINT-POLICY l.9211 · — AtpBlueprint's blueprintPolicy reader/writer REMAINS deferred to the concrete BlueprintPolicy subclasses; AtpBlueprint was stamped on 2026-09-26 after this reader/writer gap was accepted as a deviation)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `AtpBlueprint` (heritage mixin branch of ARPackage (Table 4.1 Base) · R23-11 markdown · AUTOSAR_CP_TPS_BSWModuleDescriptionTemplate · Table D.11, p.305)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `AtpBlueprintable` (tracker input · **source corrected R4.3.1→R23-11** (the class exists in R23-11; R4.3.1 citation was a mis-detection — see Step 8) · R23-11/AUTOSAR_FO_TPS_StandardizationTemplate · Table C.14, p.162 · **heritage fix**: re-parent `AtpBlueprintable(PackageableElement)` → `(Identifiable)` — Base = ARObject, Identifiable, MultilanguageReferrable, Referrable (no PackageableElement/CollectableElement in its chain; element aggregation lives on Identifiable so subclasses keep it); do AFTER the Identifiable row above) (synced b7cf0309)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `AtpBlueprintMapping` (tracker input · **R23-11** corpus — corrected from the todo's R4.3.1 citation: R23-11 Table C.13 (FO_StandardizationTemplate PDF p.162) is the authoritative spec; the R4.3.1 Table 4.4 "AtpBlueprintMapping" is mislabeled BlueprintPolicy content, already synced as BlueprintPolicy R23-11 C.18 · AUTOSAR_00052.xsd group ATP-BLUEPRINT-MAPPING l.6888) — abstract shell carrying two `atpAbstract` derived association ends (`atpBlueprint` → `atpBlueprintRef`, `atpBlueprintedElement` → `atpBlueprintedElementRef`) added 2026-09-03 after the stamp was unblocked; they are serialized only on the concrete `BlueprintMapping` subclass (BLUEPRINT-REF / DERIVED-OBJECT-REF), so the abstract class has no own XML element (reader/writer `[—]`).
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ApplicationDeferredDataType` (tracker input · R23-11 markdown · AUTOSAR_FO_TPS_AbstractPlatformSpecification · Table 3.17)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `PortInterfaceMapping` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 4.20)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `AutosarDataType` (**NEW — parent of `AbstractImplementationDataType`** · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.1, p.232 · **heritage fix: code `AutosarDataType(AtpType, ABC)` → spec most-derived direct base `ARElement`**) · **stamped
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `AbstractImplementationDataType` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.14, p.267 · **after `AutosarDataType` (base)**)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `AbstractImplementationDataTypeElement` ) — finished, stamped
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DataInterface` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 3.19, p.87 · **before `NvDataInterface` / `ParameterInterface` / `SenderReceiverInterface` (base of all three)**) — finished, stamped
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ParameterInterface` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 2.2, p.41 · after `DataInterface` (base))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `NvDataInterface` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 11.5, p.664 · after `DataInterface` (base) and `VariableDataPrototype` (member type))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `HandleInvalidEnum` (**NEW — member type of `InvalidationPolicy.handleInvalid` · `AREnum`** · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 4.3, p.97 (SystemTemplate Table 6.6, p.306 — unused, R23-11 SWCT table exists)) — finished, stamped
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `InvalidationPolicy` (**NEW — member type of `SenderReceiverInterface.invalidationPolicy`** · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 4.2, p.97 · after `VariableDataPrototype` (member `dataElement`) and `HandleInvalidEnum` (member `handleInvalid`))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `SenderReceiverInterface` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 4.1, p.94 · after `DataInterface` (base), `VariableDataPrototype`, `InvalidationPolicy` (member types)) — finished, stamped
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `Trigger` (**NEW — member type of `TriggerInterface.trigger` and `TriggerMapping.firstTrigger`/`secondTrigger`** · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 4.13, p.109 (BSW Table 4.16, p.46 — same class, R23-11 SWCT table is authoritative))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `TriggerInterface` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 4.12, p.109 · after `Trigger` (member type))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `TriggerMapping` (**NEW — member type of `TriggerInterfaceMapping.triggerMapping`** · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 4.31, p.134 · after `Trigger` (both attributes are refs to it))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `TriggerInterfaceMapping` and `TriggerMapping` (member type))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `IdentCaption` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 14.4, p.851 · **before `ModeAccessPointIdent` (base)**)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ModeAccessPointIdent` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 14.5, p.852 · after `IdentCaption` (base))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ModeDeclarationMappingSet` )
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `SubElementRef` (**NEW — member type of `SubElementMapping.firstElement`/`secondElement`** · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 4.33, p.138)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `TextTableMapping` (**NEW — member type of `SubElementMapping.textTableMapping`** · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 4.36, p.145 (SystemTemplate Table 5.25, p.230 — unused, R23-11 SWCT table exists) · batch 9b confirmed 2026-09-05)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `SubElementMapping` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 4.32, p.137 · after `SubElementRef` and `TextTableMapping` (member types) · batch 9b confirmed 2026-09-05)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `FlatInstanceDescriptor` (**NEW — member type of `FlatMap.instance`** · R23-11 markdown · AUTOSAR_CP_TPS_SystemTemplate · Table 14.2, p.967 (BSW Table D.32 — appendix, R23-11 SystemTemplate table is authoritative) ·)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `FlatMap` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SystemTemplate · Table 14.1, p.966 · **heritage fix: code `FlatMap(AtpBlueprintable)` → spec most-derived direct base `ARElement`** · after `FlatInstanceDescriptor` (member type) · batch 9b confirmed 2026-09-05)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ProgramminglanguageEnum` (**NEW — member type of `Implementation.programmingLanguage` · `AREnum`** · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 8.2, p.621 · batch 9b confirmed 2026-09-05)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `Compiler` (**NEW — member type of `Implementation.compiler`** · R23-11 markdown · AUTOSAR_CP_TPS_BSWModuleDescriptionTemplate · Table 7.7, p.133 (SWCT Table 8.3, p.621 — same class, BSW chapter 7 owns the Implementation cluster))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `Linker` (**NEW — member type of `Implementation.linker`** · R23-11 markdown · AUTOSAR_CP_TPS_BSWModuleDescriptionTemplate · Table 7.8, p.134 (SWCT Table 8.4, p.622 — same class))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `Code` (**NEW — member type of `Implementation.codeDescriptor`** · R23-11 markdown · AUTOSAR_CP_TPS_BSWModuleDescriptionTemplate · Table 7.2, p.130 (SWCT Table 8.5, p.622 — same class))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DependencyOnArtifact` (**NEW — member type of `Implementation.generatedArtifact` / `requiredArtifact` / `requiredGeneratorTool` (three attributes, one type)** · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.91, p.413 (BSW Table 7.3, p.131 — same class))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ResourceConsumption` (**NEW — member type of `Implementation.resourceConsumption`** · R23-11 markdown · AUTOSAR_CP_TPS_BSWModuleDescriptionTemplate · Table 8.1, p.138 (SystemTemplate Table 5.44, p.261 — same class))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `SwcBswMapping` (**NEW — member type of `Implementation.swcBswMapping`** · R23-11 markdown · AUTOSAR_CP_TPS_BSWModuleDescriptionTemplate · Table 5.46, p.110 (SWCT Table 11.1, p.656 — same class) · **heritage fix: code `SwcBswMapping(AtpStructureElement)` → spec most-derived direct base `ARElement`**)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BuildActionInvocator`
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BuildActionEntity`
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BuildEngineeringObject`
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BuildActionIoElement`
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BuildActionEnvironment` (**NEW — member type of `BuildActionManifest.buildActionEnvironment` · absent from `src/`** · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 10.4, p.370)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BuildAction` (**NEW — member type of `BuildActionManifest.buildAction` / `.dynamicAction` / `.startAction` / `.tearDownAction` · absent from `src/`** · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 10.2, p.366)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BuildActionManifest` (**NEW — member type of `Implementation.buildActionManifest` · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 10.1, p.365 (BSW Table 7.9, p.135 — same class)) — finished, stamped
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `Implementation` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_BSWModuleDescriptionTemplate · Table 7.1, p.128 (SWCT Table 8.1, p.621 — same class; the todo's "multiple tables — resolve in per-class Phase 0" is resolved: BSW chapter 7 owns the Implementation cluster) · **last row of the queue**)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

## Pending 16.4 resolution (NEW — not in src)





_(none)_ — every class added to the queue by the 2026-09-04 review has a **Class** or
**Enumeration** table in the R23-11 markdown corpus (verified class-by-class), so no
Skip / XSD-derive decision is pending. The classes that are absent from `src/` but
present in the spec (`BuildActionEntity`, `BuildActionInvocator`, `BuildEngineeringObject`,
`BuildActionIoElement`, `BuildAction`, `BuildActionEnvironment`, `TextTableValuePair`,
`MappingDirectionEnum`, `ImplementationDataTypeSubElementRef`,
`ApplicationCompositeDataTypeSubElementRef`) are implemented from their spec tables —
they are not XSD-derived and are therefore not 16.4 cases.

## Not queued





Deliberately out of the queue (2026-09-04 review). Each entry below is a **subclass**
edge, not a `Base` or member-type edge, so it does not block its parent's stamp
(`PortInterfaceMapping` Step-8 precedent):

- `ImplementationDataType` — subclass of `AbstractImplementationDataType` (SWCT Table
  5.15, md l.7820). **Already stamped R23-11 while its base is not** — an inversion;
  run a Rule 0012.3 drift pass once the base lands.
- `ImplementationDataTypeElement` — subclass of `AbstractImplementationDataTypeElement`
  (SWCT Table 5.17, p.270); UNSTAMPED. Drift candidate after the base lands.
- `SwcImplementation` — subclass of `Implementation` (SWCT Table 8.7, p.623); UNSTAMPED.
  Audit candidate once `Implementation` lands (`BswImplementation` is already stamped).
- `BswServiceDependencyIdent` (DiagnosticExtract Table 5.16, p.240),
  `DiagnosticParameterIdent` (DiagnosticExtract Table 4.7, p.37),
  `ExternalTriggeringPointIdent` (SWCT Table 14.6, p.852) — the other three
  `IdentCaption` subclasses (SWCT Table 14.4 Subclasses row).
- `VariableAndParameterInterfaceMapping` (SWCT Table 4.21, p.125),
  `ClientServerInterfaceMapping` (SWCT Table 4.23, p.128), `ModeInterfaceMapping`
  (SWCT Table 4.26, p.130) — the other `PortInterfaceMapping` subclasses; already
  recorded as future queue candidates in that row's Step 8.
- `ApplicationDataType` (SWCT Table 5.2, p.232) — the second `AutosarDataType`
  subclass; audit candidate once the `AutosarDataType` heritage fix lands.

Also out of scope here (already recorded elsewhere in this file): the
`ModeDeclarationGroupPrototype` / `HwPinGroupContent` drift rows and the ten
uuid-move heritage rows.


---
