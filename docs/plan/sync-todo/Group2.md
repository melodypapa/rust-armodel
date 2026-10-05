# Sync todo: Group 2 — PortInterface sets, components, SWC behavior, datatypes

Input: `Group 2 — PortInterface sets, components, SWC behavior, datatypes` of `docs/examples/sync_class_groups.md` · Generated: 2026-08-30 · Queue order = row order
(resume = first class row still `[ ]`; all class rows `[x]` = sync finished — Rule 0009.4)

## Queue (dependency-first)

- [ ] `PortInterfaceMappingSet` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 4.19 · after `PortInterfaceMapping` (Group 1) — **dependency synced+stamped 2026-08-31**)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `MetaDataItem` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 4.4 · member type of `MetaDataItemSet.metaDataItem` — **queued 2026-08-31 per user direction "MetaDataItem is dependent class"; Rule 0016.4 stub → same-pass dependency-first sync**)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `MetaDataItemSet`
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ApplicationCompositeElementInPortInterfaceInstanceRef` (tracker input · **spec CORRECTED 2026-08-31 per-class Phase 0: R23-11 markdown Table D.17 (CP_TPS_SoftwareComponentTemplate.md l.28228–28249, PDF p.953 — appendix letter-numbered table missed by numeric-regex tooling), R4.3.1 Table F.6 fallback exists; NOT XSD-only** — user-confirmed "按 R23-11 Table D.17 同步修正本类并更新队列标注") — Package=...PortInterface::InstanceRefs (leaf → InstanceRefs.py ✓); Base=ARObject, AtpInstanceRef → most-derived AtpInstanceRef ✓ current; Note empty → no class docstring; Aggregated by CompositeNetworkRepresentation.leafElement + SubElementMapping.first/secondElement (both wired). Attrs: base (DataInterface, 0..1, ref; XSD skips serialization) / contextDataPrototype (AppCompositeElementDataPrototype, `*`, ref, ordered) / rootDataPrototype (AutosarDataPrototype, 0..1, ref) / targetDataPrototype (AppCompositeElementDataPrototype, 0..1, ref)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `SymbolProps` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.21) — sub-checklist flipped 2026-09-04 per already-verified short-circuit: marker present in source + quick deviation check clean (base ImplementationProps most-derived ✓, zero own attributes per Table 5.21 ✓, class Note verbatim ✓, inherited symbol reader/writer coverage ✓, six-column checklist with release column ✓)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `PPortPrototype` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 3.6)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `RPortPrototype` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 3.5)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `PRPortPrototype` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 3.7) — sub-checklist completed 2026-09-04 per already-verified short-circuit: marker present in source + spec/XSD shape clean (AtpPrototype heritage, providedRequiredInterfaceTRef 0..1 tref, matching parser/writer accessors, verbatim notes, six-column checklist); stale type deviation removed from method_deviation_by_class_v2.md; Step 9b user-confirmed 2026-09-04 — **2026-09-26 stamp audit: the claimed l.778 marker was found ABSENT from src (the checklist had only the `# Spec:` line — record-vs-src mismatch) → user 9b re-confirmed; written after the `# Spec:` line
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `PortGroup` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 4.94) — sub-checklist flipped 2026-09-04 per already-verified short-circuit: marker present in source + quick deviation check clean. Prior session completed the full 9-step pass but never flipped the sub-checklist
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `InnerPortGroupInCompositionInstanceRef` (tracker input · **spec CORRECTED per-class Phase 0: R23-11 markdown Table D.4 (CP_TPS_SoftwareComponentTemplate.md l.27985, PDF p.943 — appendix letter-numbered table missed by numeric-regex tooling, same as D.17 case), NOT XSD-only** · — member type of `PortGroup.innerGroup`) — Package=...Components::InstanceRefs (leaf → InstanceRefs.py ✓); Base=ARObject, AtpInstanceRef → most-derived AtpInstanceRef ✓ current; Note empty → no class docstring; Aggregated by PortGroup.innerGroup ✓. Attrs: base (CompositionSwComponentType, 0..1, ref, atpDerived → no XML) / context (ordered) (SwComponentPrototype, `*`, ref, CONTEXT-REF, DEST SW-COMPONENT-PROTOTYPE--SUBTYPES-ENUM, seqOffset 20) / target (PortGroup, 0..1, ref, TARGET-REF, DEST PORT-GROUP--SUBTYPES-ENUM, seqOffset 30); current code drops CONTEXT-REF in both reader (parser L5655 commented) and writer (writer L1770 commented)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `RTEEvent` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 7.9 (abstract) · parent of `InitEvent`/`BackgroundEvent` below and member type of `SwcInternalBehavior.event` · direct parent `AbstractEvent` Table 7.8 stamped ✓)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ServerCallPoint` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 7.35 (abstract) · parent of `SynchronousServerCallPoint`/`AsynchronousServerCallPoint` below · attr `timeout` TimeValue · direct parent `AbstractAccessPoint` Table 7.31 stamped ✓ · NOTE: no `ServerCall` meta-class exists in R23-11 — the point classes reference each other)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `VariableDataPrototype` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.31 · member type of `SwcInternalBehavior.arTypedPerInstanceMemory`/`explicitInterRunnableVariable`/`implicitInterRunnableVariable`, `VariableInAtomicSwcInstanceRef.abstractTargetDataElement`, `ArVariableInImplementationDataInstanceRef.rootVariableDataPrototype`, `InvalidationPolicy.dataElement` · parent `DataPrototype` stamped ✓; dependency audit also lists `NvDataInterface.nvData` and `SenderReceiverInterface.dataElement`
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `PerInstanceMemory` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 7.49 · member type of `SwcInternalBehavior.perInstanceMemory`)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `PortInCompositionTypeInstanceRef` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table D.14 (abstract; appendix letter-numbered table, same as D.17/D.4 cases) · iref type of `DelegationSwConnector.innerPort` (XSD serializes as P/R-PORT-IN-COMPOSITION-INSTANCE-REF) · sync concrete subclasses `PPortInCompositionInstanceRef` Table D.15 / `RPortInCompositionInstanceRef` Table D.16 in the same pass · parent `AtpInstanceRef` stamped ✓)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `AssemblySwConnector`
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DataTypeMappingSet` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.4 · member type of `CompositionSwComponentType.dataTypeMapping`)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ApplicationDataType` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.2 (abstract; base of the three concrete Application* data types below — **moved 2026-09-03 restructure ahead of its subclasses**))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ApplicationCompositeElementDataPrototype` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.30 · parent of `ApplicationRecordElement` below · after `ApplicationDataType` (its `type` tref) · parent `DataPrototype` stamped ✓)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `InitEvent` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 7.22 · after `RTEEvent` (parent, Table 7.9)) — sub-checklist finalized 2026-09-05 per already-verified short-circuit: marker present in source + quick deviation check clean (concrete `RTEEvent` subclass ✓, zero own attributes per Table 7.22 ✓, class Note verbatim ✓, inherited parser/writer coverage ✓, six-column checklist with release column ✓)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BackgroundEvent` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 7.16 · after `RTEEvent` (parent, Table 7.9))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `SynchronousServerCallPoint` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 7.36 · after `ServerCallPoint` (parent, Table 7.35) · attr `calledFromWithinExclusiveArea` → `ExclusiveAreaNestingOrder` stamped ✓)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `AsynchronousServerCallPoint` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 7.37 · after `ServerCallPoint` (parent, Table 7.35))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `AsynchronousServerCallResultPoint` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 7.38 · after `AsynchronousServerCallPoint` (its `asynchronousServerCallPoint` ref target) · NOTE: spec Base = AbstractAccessPoint chain, NOT ServerCallPoint)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `VariableInAtomicSwcInstanceRef` (tracker input · **spec CORRECTED 2026-09-03 restructure: R23-11 markdown Table D.1 exists (CP_TPS_SoftwareComponentTemplate.md — appendix letter-numbered table missed by numeric-regex tooling, same as D.17/D.4 cases), NOT XSD-only** · after `VariableDataPrototype` (`abstractTargetDataElement` ref) · `base` AtomicSwComponentType stamped ✓ / `contextPort` PortPrototype stamped ✓)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `VariableInAtomicSWCTypeInstanceRef` (tracker input · **spec CORRECTED 2026-09-03 restructure: R23-11 markdown Table D.18 exists (appendix letter-numbered table, same as D.17/D.4 cases), NOT XSD-only** · after `VariableDataPrototype`)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ArVariableInImplementationDataInstanceRef` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.37, p.322 · after `AbstractImplementationDataTypeElement` (`contextDataPrototype`/`targetDataPrototype` refs) + `VariableDataPrototype` (`rootVariableDataPrototype` ref) · `portPrototype` PortPrototype stamped ✓) — finished, stamped
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DelegationSwConnector` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 3.14 · after `PortInCompositionTypeInstanceRef` (its `innerPort` iref type) · `outerPort` PortPrototype stamped ✓)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ApplicationPrimitiveDataType` (tracker input · R23-11 markdown · **spec ref CORRECTED 2026-09-03 restructure: AUTOSAR_CP_TPS_SoftwareComponentTemplate Table 5.5 (was: DiagnosticExtractTemplate Table 5.6)** · after `ApplicationDataType` (parent, Table 5.2))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ApplicationCompositeDataType` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.6 (multiple tables — resolve in per-class Phase 0) · after `ApplicationDataType` (parent, Table 5.2))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ApplicationRecordElement` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.13 *(existing member)* · after `ApplicationCompositeElementDataPrototype` (parent, Table 5.30))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `RunnableEntityArgument` · only attr `symbol` CIdentifier — no unsynced deps)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ExternalTriggeringPointIdent` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 14.6 · member type of `ExternalTriggeringPoint.ident` (Table 7.39) · no unsynced member deps)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `PortDefinedArgumentValue` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 7.45 *(existing member)* · deps stamped: `value` ValueSpecification ✓ / `valueType` ImplementationDataType ✓)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `CompositionSwComponentType` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 3.10 (multiple tables — resolve in per-class Phase 0) · after `AssemblySwConnector`/`DelegationSwConnector` (aggr `connector`) + `DataTypeMappingSet` (ref `dataTypeMapping`) · deps stamped: `component` SwComponentPrototype ✓ / `constantValueMapping` ConstantSpecificationMappingSet ✓ / `instantiationRTEEventProps` InstantiationRTEEventProps ✓ · ref target `PhysicalDimensionMappingSet` NOT in src — pending 16.4 below)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ApplicationRecordDataType` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.12 (multiple tables — resolve in per-class Phase 0) · after `ApplicationRecordElement` (aggr `element`) + `ApplicationDataType` (parent)) — **skipped by user confirmation; unresolved naming/registry deviation retained; no stamp
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DataTransformationErrorHandlingEnum` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 7.43 · member type of `PortAPIOption.errorHandling`) — row flipped 2026-09-11 per already-verified short-circuit: marker present in source + quick deviation check clean (AREnum kind + both literals match Table 7.43 p.590 ✓; class Note verbatim ✓; literal names/values/`atp.EnumerationLiteralIndex` tags verbatim ✓; value-form serialization on `PortAPIOption.errorHandling` — reader arxml_parser.py:4696/4698, writer arxml_writer.py:4245 ✓; six-column checklist with release column + `# Spec:` line p.590 ✓; focused test `test_PortAPIOptionDependencies.py::TestDataTransformationErrorHandlingEnum` green)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DataTransformationStatusForwardingEnum` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 7.44 · member type of `PortAPIOption.transformerStatusForwarding`)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `SwcSupportedFeature` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 7.46 · member type of `PortAPIOption.supportedFeature`)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `CommunicationBufferLocking` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 7.47 · member type of `PortAPIOption.supportedFeature`)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `SupportBufferLockingEnum` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 7.48 · member type of `CommunicationBufferLocking.supportBufferLocking`)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `PortAPIOption` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 7.42 · after `PortDefinedArgumentValue` (aggr `portArgValue`) · `port` PortPrototype stamped ✓ · aggr target `SwcSupportedFeature` queued above)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `IncludedModeDeclarationGroupSet`
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `SwcInternalBehavior`
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





- `PhysicalDimensionMappingSet` — ref target of `CompositionSwComponentType.physicalDimensionMapping` (Table 3.10); implement before or during the CompositionSwComponentType sync, or record a stub deviation

## Not queued





- `ServerCall` — does **not exist** in the R23-11 meta-model (verified: no class table; Tables 7.35–7.38 jump from `ServerCallPoint` to the point classes). The point classes reference each other (`AsynchronousServerCallResultPoint.asynchronousServerCallPoint` → `AsynchronousServerCallPoint`). Recorded here so nobody hunts for it.
