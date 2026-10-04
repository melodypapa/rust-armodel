# Sync todo: Group 12 — SwcInternalBehavior & measurement refs

Input: full-repo orphan audit 2026-09-23 (unstamped ∧ untracked, M2 only) · Queue order = row order
(resume = first class row still `[ ]`; all class rows `[x]` = sync finished — Rule 0009.4)

## Queue (dependency-first)

- [ ] `ParameterAccess` — AbstractAccessPoint — R23-11 CP_TPS_SoftwareComponentTemplate Table 7.40, p.586
  - module: M2/AUTOSARTemplates/SWComponentTemplate/SwcInternalBehavior/DataElements/__init__.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `VariableAccessScopeEnum` — AREnum — R23-11 CP_TPS_SoftwareComponentTemplate Table 7.34, p.567
  - module: M2/AUTOSARTemplates/SWComponentTemplate/SwcInternalBehavior/DataElements/__init__.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `VariableAccess` — AbstractAccessPoint — R23-11 CP_TPS_SoftwareComponentTemplate Table 7.33, p.567 (sync completed by scope fix pass 12e743cc9,)
  - module: M2/AUTOSARTemplates/SWComponentTemplate/SwcInternalBehavior/DataElements/__init__.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `InternalTriggeringPoint` — AbstractAccessPoint — R23-11 markdown · Table 7.30 (CP_TPS_SoftwareComponentTemplate)
  - module: M2/AUTOSARTemplates/SWComponentTemplate/SwcInternalBehavior/Trigger.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ModeAccessPoint` — ARObject — R23-11 markdown · Table 9.5 (CP_TPS_SoftwareComponentTemplate)
  - module: M2/AUTOSARTemplates/SWComponentTemplate/SwcInternalBehavior/ModeDeclarationGroup.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ModeSwitchPoint` — AbstractAccessPoint — R23-11 markdown · Table 9.4 (CP_TPS_SoftwareComponentTemplate)
  - module: M2/AUTOSARTemplates/SWComponentTemplate/SwcInternalBehavior/ModeDeclarationGroup.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `AsynchronousServerCallReturnsEvent` — RTEEvent — R23-11 markdown · Table 7.10 (CP_TPS_SoftwareComponentTemplate)
  - module: M2/AUTOSARTemplates/SWComponentTemplate/SwcInternalBehavior/RTEEvents.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DataReceiveErrorEvent` — RTEEvent — R23-11 markdown · Table 7.14 (CP_TPS_SoftwareComponentTemplate)
  - module: M2/AUTOSARTemplates/SWComponentTemplate/SwcInternalBehavior/RTEEvents.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DataReceivedEvent` — RTEEvent — R23-11 markdown · Table 7.13 (CP_TPS_SoftwareComponentTemplate)
  - module: M2/AUTOSARTemplates/SWComponentTemplate/SwcInternalBehavior/RTEEvents.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DataSendCompletedEvent` — RTEEvent — R23-11 markdown · Table 7.11 (CP_TPS_SoftwareComponentTemplate)
  - module: M2/AUTOSARTemplates/SWComponentTemplate/SwcInternalBehavior/RTEEvents.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DataWriteCompletedEvent` — RTEEvent — R23-11 markdown · Table 7.12 (CP_TPS_SoftwareComponentTemplate)
  - module: M2/AUTOSARTemplates/SWComponentTemplate/SwcInternalBehavior/RTEEvents.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `InternalTriggerOccurredEvent` — RTEEvent — R23-11 markdown · Table 7.21 (CP_TPS_SoftwareComponentTemplate)
  - module: M2/AUTOSARTemplates/SWComponentTemplate/SwcInternalBehavior/RTEEvents.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `OperationInvokedEvent` — RTEEvent — R23-11 markdown · Table 7.15 (CP_TPS_SoftwareComponentTemplate)
  - module: M2/AUTOSARTemplates/SWComponentTemplate/SwcInternalBehavior/RTEEvents.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `RteEventInEcuInstanceRef` — AtpInstanceRef — XSD-only (verified against BOTH corpora 2026-09-25: appears only in AtpInstanceRef Subclasses rows, no own table in R23-11 or R4.3.1) — AUTOSAR_00052.xsd group RTE-EVENT-IN-ECU-INSTANCE-REF (l.100559) / complexType (l.100605)
  - module: M2/AUTOSARTemplates/CommonStructure/MeasurementCalibrationSupport/__init__.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `VariableAccessInEcuInstanceRef` — AtpInstanceRef — XSD-only (verified against BOTH corpora 2026-09-25: appears only in AtpInstanceRef Subclasses rows, no own table in R23-11 or R4.3.1) — AUTOSAR_00052.xsd group VARIABLE-ACCESS-IN-ECU-INSTANCE-REF (l.129520) / complexType (l.129566)
  - module: M2/AUTOSARTemplates/CommonStructure/MeasurementCalibrationSupport/__init__.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)
