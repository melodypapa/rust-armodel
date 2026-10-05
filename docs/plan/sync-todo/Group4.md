# Sync todo: Group 4 — BSW behavior policies & ServiceNeeds A

Input: `Group 4 — BSW behavior policies & ServiceNeeds A` of `docs/examples/sync_class_groups.md` · Generated: 2026-08-30 · Queue order = row order
(resume = first class row still `[ ]`; all class rows `[x]` = sync finished — Rule 0009.4)

## Queue (dependency-first)

- [ ] `BswPerInstanceMemoryPolicy` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_BSWModuleDescriptionTemplate · Table 5.2 · aggregated by `BswInternalBehavior` · **NOT in src** — class must be created when this row is synced · **Step 1 finding: XSD-only — no table in any markdown corpus (R23-11/R4.3.1/R4.4.0 grep → XSD hits only); syncs from `AUTOSAR_00052.xsd` complexType `BSW-PER-INSTANCE-MEMORY-POLICY` line 12370 → marker. Base chain ARObject→BswApiOptions→own; own attr `arTypedPerInstanceMemory` (ref, 0..1, DEST VARIABLE-DATA-PROTOTYPE) → `arTypedPerInstanceMemoryRef: Optional[RefType]`; VARIATION-POINT present → VariationPointCapable mixin) — verified R23-11 XSD `AUTOSAR_00052.xsd`
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BswClientPolicy` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_BSWModuleDescriptionTemplate · Table 5.2 · aggregated by `BswInternalBehavior` · **NOT in src** — class must be created when this row is synced · **Step 1 finding: XSD-only — no own table in any markdown corpus (R23-11/R4.3.1/R4.4.0 grep → aggregation rows only); syncs from `AUTOSAR_00052.xsd` complexType `BSW-CLIENT-POLICY` line 9616 (group line 9585) → marker. Base chain ARObject→BswApiOptions→own; own attr `requiredClientServerEntry` (ref, 0..1, DEST BSW-MODULE-CLIENT-SERVER-ENTRY) → `requiredClientServerEntryRef: Optional[RefType]`; VARIATION-POINT present → VariationPointCapable mixin; aggregated via `CLIENT-POLICYS` wrapper — XSD order right after `BSW-PER-INSTANCE-MEMORY-POLICYS`; needs `addClientPolicy` on BswInternalBehavior) — verified R23-11 XSD `AUTOSAR_00052.xsd`
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BswInternalTriggeringPointPolicy` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_BSWModuleDescriptionTemplate · Table 5.2 · aggregated by `BswInternalBehavior` · **NOT in src** — class must be created when this row is synced · **Step 1 finding: XSD-only — no own table in any markdown corpus (R23-11/R4.3.1/R4.4.0 grep → no markdown hits); syncs from `AUTOSAR_00052.xsd` complexType `BSW-INTERNAL-TRIGGERING-POINT-POLICY` line 10850 (group line 10819) → marker. Base chain ARObject→BswApiOptions→own; own attr `bswInternalTriggeringPoint` (ref, 0..1, DEST BSW-INTERNAL-TRIGGERING-POINT) → `bswInternalTriggeringPointRef: Optional[RefType]`; VARIATION-POINT present → VariationPointCapable mixin; aggregated via `INTERNAL-TRIGGERING-POINT-POLICYS` wrapper — XSD sequenceOffset order after `INCLUDED-MODE-DECLARATION-GROUP-SETS`; needs `addInternalTriggeringPointPolicy` on BswInternalBehavior) — verified R23-11 XSD `AUTOSAR_00052.xsd`
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BswParameterPolicy` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_BSWModuleDescriptionTemplate · Table 5.2 · aggregated by `BswInternalBehavior` · **NOT in src** — class must be created when this row is synced · **Step 1 finding: XSD-only — no own table in any markdown corpus (R23-11/R4.3.1/R4.4.0 grep → no markdown hits); syncs from `AUTOSAR_00052.xsd` complexType `BSW-PARAMETER-POLICY` line 12325 (group line 12294) → marker. Base chain ARObject→BswApiOptions→own; own attr `perInstanceParameter` (ref, 0..1, DEST PARAMETER-DATA-PROTOTYPE) → `perInstanceParameterRef: Optional[RefType]`; VARIATION-POINT present → VariationPointCapable mixin; aggregated via `PARAMETER-POLICYS` wrapper — XSD sequenceOffset order after `INTERNAL-TRIGGERING-POINT-POLICYS`; needs `addParameterPolicy` on BswInternalBehavior) — verified R23-11 XSD `AUTOSAR_00052.xsd`
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BswReleasedTriggerPolicy` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_BSWModuleDescriptionTemplate · Table 5.2 · aggregated by `BswInternalBehavior` · **NOT in src** — class must be created when this row is synced · **Step 1 finding: XSD-only — no own table in any markdown corpus (R23-11/R4.3.1/R4.4.0 grep → no markdown hits); syncs from `AUTOSAR_00052.xsd` complexType `BSW-RELEASED-TRIGGER-POLICY` line 12446 (group line 12415) → marker. Base chain ARObject→BswApiOptions→own; own attr `releasedTrigger` (ref, 0..1, DEST TRIGGER) → `releasedTriggerRef: Optional[RefType]`; VARIATION-POINT present → VariationPointCapable mixin; aggregated via `RELEASED-TRIGGER-POLICYS` wrapper — XSD sequenceOffset order after `PARAMETER-POLICYS`; needs `addReleasedTriggerPolicy` on BswInternalBehavior) — verified R23-11 XSD `AUTOSAR_00052.xsd`
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BswDataSendPolicy` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_BSWModuleDescriptionTemplate · Table 5.2 · aggregated by `BswInternalBehavior` · **NOT in src** — class must be created when this row is synced · **Step 1 finding: XSD-only — no own table in any markdown corpus (R23-11/R4.3.1/R4.4.0 grep → no markdown hits); syncs from `AUTOSAR_00052.xsd` complexType `BSW-DATA-SEND-POLICY` line 9802 (group line 9758) → marker. Base chain ARObject→BswApiOptions→own; own attrs `providedData` (ref, 0..1, DEST VARIABLE-DATA-PROTOTYPE) → `providedDataRef: Optional[RefType]` AND `proviedeData` (ref, 0..1, DEST VARIABLE-DATA-PROTOTYPE, **atp.Status="obsolete"** — legacy misspelled `PROVIEDE-DATA-REF`, modeled as optional legacy member with full coverage + accepted deviation, Rule 0019 pattern); VARIATION-POINT present → VariationPointCapable mixin; aggregated via `SEND-POLICYS` wrapper — XSD sequenceOffset order after `RELEASED-TRIGGER-POLICYS`; needs `addSendPolicy` on BswInternalBehavior) — verified R23-11 XSD `AUTOSAR_00052.xsd`
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BswInternalBehavior` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_BSWModuleDescriptionTemplate · Table 5.2 (multiple tables — resolve in per-class Phase 0) · after all 6 NEW policy classes above (deferred full sync;) · **Step 1 finding: own table = R23-11 `AUTOSAR_CP_TPS_BSWModuleDescriptionTemplate` Table 5.2, p.68 (page-split, pp.67-68; sibling copy Table 10.4 Swc TPS p.652); Base most-derived = `InternalBehavior` (matches src); class Note captured verbatim; 22 own attrs all `*` aggr in displayed order = arTypedPerInstanceMemory, bswPerInstanceMemoryPolicy, clientPolicy, distinguishedPartition, entity, event, exclusiveAreaPolicy, includedDataTypeSet, includedModeDeclarationGroupSet, internalTriggeringPoint, internalTriggeringPointPolicy, modeReceiverPolicy, modeSenderPolicy, parameterPolicy, perInstanceParameter, receptionPolicy, releasedTriggerPolicy, schedulerNamePrefix, sendPolicy, serviceDependency, triggerDirectImplementation, variationPointProxy — field list matches src exactly. Gaps to close: (a) type 21 untyped list fields; (b) missing accessors addArTypedPerInstanceMemory/addExclusiveAreaPolicy/addModeReceiverPolicy/setModeReceiverPolicies/addPerInstanceParameter/addTriggerDirectImplementation/addVariationPointProxy; (c) reader/writer miss 7 BSW wrappers (Swc-side helpers exist): AR-TYPED-PER-INSTANCE-MEMORYS, EXCLUSIVE-AREA-POLICYS (BSW flavor), INCLUDED-DATA-TYPE-SETS, MODE-RECEIVER-POLICYS, PER-INSTANCE-PARAMETERS, VARIATION-POINT-PROXYS, TRIGGER-DIRECT-IMPLEMENTATIONS; (d) docstrings stale → wipe + rewrite from Table 5.2 notes; child classes BswExclusiveAreaPolicy/BswModeReceiverPolicy/BswTriggerDirectImplementation fully modeled) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ServiceNeeds` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_BSWModuleDescriptionTemplate · Table 12.6 (multiple tables — resolve in per-class Phase 0) · **Step 1 finding: own table = BSW TPS Table 12.6, p.228 — pure abstract marker class, Base most-derived = `Identifiable`, ZERO own attributes (empty Attribute section); class Note missing from the BSW render, taken verbatim from sibling copy Swc TPS Table 7.52, p.603 (Package row: M2::AUTOSARTemplates::CommonStructure::ServiceNeeds — matches src location); aggregated by Bsw/SwcServiceDependency.serviceNeeds (aggregation rows belong to the dependency classes); no own XML element (XSD `SERVICE-NEEDS` = abstract choice-group ref, AUTOSAR_00052.xsd line 10996) → Steps 5/6 N/A; src already spec-shaped (ABC + abstract guard + Identifiable base) → sync = test + docstring + checklist only) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DiagEventDebounceAlgorithm` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_BSWModuleDescriptionTemplate · Table 12.32 (multiple tables — resolve in per-class Phase 0))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DiagEventDebounceMonitorInternal` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_BSWModuleDescriptionTemplate · Table 12.35 (multiple tables — resolve in per-class Phase 0))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `EcuStateMgrUserNeeds` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_BSWModuleDescriptionTemplate · Table 12.14 (multiple tables — resolve in per-class Phase 0))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DltUserNeeds` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_BSWModuleDescriptionTemplate · Table 12.16 (multiple tables — resolve in per-class Phase 0))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DiagnosticComponentNeeds` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 13.64))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DiagnosticUploadDownloadNeeds` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_BSWModuleDescriptionTemplate · Table 12.29 (multiple tables — resolve in per-class Phase 0))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DiagnosticsCommunicationSecurityNeeds` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_BSWModuleDescriptionTemplate · Table 12.27 (multiple tables — resolve in per-class Phase 0))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `FunctionInhibitionNeeds` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_BSWModuleDescriptionTemplate · Table 12.19 (multiple tables — resolve in per-class Phase 0))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `GlobalSupervisionNeeds` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 13.4)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `HardwareTestNeeds` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_BSWModuleDescriptionTemplate · Table 12.40 (multiple tables — resolve in per-class Phase 0))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `SupervisedEntityCheckpointNeeds` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_BSWModuleDescriptionTemplate · Table 12.30 (multiple tables — resolve in per-class Phase 0))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `SyncTimeBaseMgrUserNeeds` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_BSWModuleDescriptionTemplate · Table 12.17 (multiple tables — resolve in per-class Phase 0))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BswMgrNeeds` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 13.8)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `CryptoKeyManagementNeeds` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 13.11)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `CryptoServiceJobNeeds` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 13.10)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DiagnosticControlNeeds` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 13.63 · **Step 1 finding: own table = Swc TPS Table 13.63, p.812 (clean render); concrete Class (XSD complexType `DIAGNOSTIC-CONTROL-NEEDS` `abstract="false"`, AUTOSAR_00052.xsd group line 33948 `<xsd:sequence/>` empty); Base row includes DiagnosticCapabilityElement → most-derived base = `DiagnosticCapabilityElement` — FIXED from src's `ServiceNeeds` (Rule 0001.2, same as DiagnosticComponentNeeds precedent 5c4c0963); ZERO own attributes (audiences/diagRequirement/securityAccessLevel are inherited from the base's own table); Note verbatim (identical in XSD doc); XSD element in BOTH choice groups (11470 BSW-side, 12589 Swc-side, order COMPONENT→CONTROL→ENABLE); dispatch was MISSING → added full 5-place pattern delegating to the read/writeDiagnosticCapabilityElement base helpers + createDiagnosticControlNeeds factory
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DiagnosticEventManagerNeeds` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 13.14)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DiagnosticRequestFileTransferNeeds` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 13.43 · **Step 1 finding: own table = Swc TPS Table 13.43, p.795 (clean render, body after caption); concrete Class (XSD complexType `DIAGNOSTIC-REQUEST-FILE-TRANSFER-NEEDS` `abstract="false"`, AUTOSAR_00052.xsd group line 42163 `<xsd:sequence/>` empty); Base row includes DiagnosticCapabilityElement → most-derived = `DiagnosticCapabilityElement` — FIXED from src's `ServiceNeeds` (Rule 0001.2); ZERO own attributes; Note verbatim (identical in XSD doc); XSD element in BOTH choice groups, order OPERATION-CYCLE→REQUEST-FILE-TRANSFER→RESPONSE-ON-EVENT; dispatch was MISSING → added full 5-place pattern delegating to the CapabilityElement base helpers + createDiagnosticRequestFileTransferNeeds factory
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DoIpActivationLineNeeds` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 13.60)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DoIpGidNeeds` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 13.55 · **Step 1 finding: own table = Swc TPS Table 13.55, p.805 (body at md 23760-23767, before caption 23769 — artifact); concrete Class (XSD complexType `DO-IP-GID-NEEDS` `abstract="false"`, AUTOSAR_00052.xsd group line 48502 `<xsd:sequence/>` empty); Base row includes DoIpServiceNeeds → most-derived = `DoIpServiceNeeds` — FIXED from src's `ServiceNeeds` (Rule 0001.2); ZERO own attributes; Note verbatim; XSD element in BOTH choice groups (11488 BSW, 12607 Swc, order ACTIVATION-LINE→GID→GID-SYNCHRONIZATION); dispatch was MISSING → added full 5-place pattern + createDoIpGidNeeds factory
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DoIpGidSynchronizationNeeds` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 13.56 · **Step 1 finding: own table = Swc TPS Table 13.56, p.806 (body at md 23771-23775, after caption 23791 — caption-body interleaving artifact); concrete Class (XSD complexType `DO-IP-GID-SYNCHRONIZATION-NEEDS` `abstract="false"`, AUTOSAR_00052.xsd group line 48530 `<xsd:sequence/>` empty); Base row includes DoIpServiceNeeds → most-derived = `DoIpServiceNeeds` — FIXED from src's `ServiceNeeds` (Rule 0001.2); ZERO own attributes; Note verbatim; XSD element in BOTH choice groups (11489 BSW, 12608 Swc); dispatch was MISSING → added full 5-place pattern + createDoIpGidSynchronizationNeeds factory
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





_All confirmed classes are recorded in the queue above; no separate confirmation records are maintained here._

- `BswPerInstanceMemoryPolicy` — not in `src` (NEW) · **(NEW)**; 16.4 decision required: **Skip** (deviation row) or **Derive-from-XSD** (then move into the queue with a 9-step sub-checklist) → **QUEUED 2026-09-11** as a 9-step row before `BswInternalBehavior`
- `BswClientPolicy` — not in `src` (NEW) · **(NEW)**; 16.4 decision required: **Skip** (deviation row) or **Derive-from-XSD** (then move into the queue with a 9-step sub-checklist) → **QUEUED 2026-09-11** as a 9-step row before `BswInternalBehavior`
- `BswInternalTriggeringPointPolicy` — not in `src` (NEW) · **(NEW)**; 16.4 decision required: **Skip** (deviation row) or **Derive-from-XSD** (then move into the queue with a 9-step sub-checklist) → **QUEUED 2026-09-11** as a 9-step row before `BswInternalBehavior`
- `BswParameterPolicy` — not in `src` (NEW) · **(NEW)**; 16.4 decision required: **Skip** (deviation row) or **Derive-from-XSD** (then move into the queue with a 9-step sub-checklist) → **QUEUED 2026-09-11** as a 9-step row before `BswInternalBehavior`
- `BswReleasedTriggerPolicy` — not in `src` (NEW) · **(NEW)**; 16.4 decision required: **Skip** (deviation row) or **Derive-from-XSD** (then move into the queue with a 9-step sub-checklist) → **QUEUED 2026-09-11** as a 9-step row before `BswInternalBehavior`
- `BswDataSendPolicy` — not in `src` (NEW) · **(NEW)**; 16.4 decision required: **Skip** (deviation row) or **Derive-from-XSD** (then move into the queue with a 9-step sub-checklist) → **QUEUED 2026-09-11** as a 9-step row before `BswInternalBehavior`

## Not queued





_(none)_
