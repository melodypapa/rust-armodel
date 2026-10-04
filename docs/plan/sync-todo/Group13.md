# Sync todo: Group 13 — BSW behavior, interfaces & SwcBswMapping

Input: full-repo orphan audit 2026-09-23 (unstamped ∧ untracked, M2 only) · Queue order = row order
(resume = first class row still `[ ]`; all class rows `[x]` = sync finished — Rule 0009.4)

## Queue (dependency-first)

- [ ] `BswApiOptions` — ARObject — XSD-only (Step 1: no own table in either corpus; group BSW-API-OPTIONS AUTOSAR_00052.xsd L9379, R4.3.1 00044.xsd L7279)
  - module: M2/AUTOSARTemplates/BswModuleTemplate/BswBehavior.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BswModuleCallPoint` — Referrable — R23-11 markdown · Table 5.10 (CP_TPS_BSWModuleDescriptionTemplate), p.77
  - module: M2/AUTOSARTemplates/BswModuleTemplate/BswBehavior.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BswDirectCallPoint` — BswModuleCallPoint — R23-11 markdown · Table 5.11 (CP_TPS_BSWModuleDescriptionTemplate), p.78
  - module: M2/AUTOSARTemplates/BswModuleTemplate/BswBehavior.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BswSynchronousServerCallPoint` — BswModuleCallPoint — R23-11 markdown · Table 5.12 (CP_TPS_BSWModuleDescriptionTemplate), p.79
  - module: M2/AUTOSARTemplates/BswModuleTemplate/BswBehavior.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BswInternalTriggeringPoint` — Identifiable — R23-11 markdown · Table 5.28 (CP_TPS_BSWModuleDescriptionTemplate), p.91
  - module: M2/AUTOSARTemplates/BswModuleTemplate/BswBehavior.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BswInterruptEntity` — BswModuleEntity — R23-11 markdown · Table 5.8 (CP_TPS_BSWModuleDescriptionTemplate), p.75
  - module: M2/AUTOSARTemplates/BswModuleTemplate/BswBehavior.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BswModeSwitchAckRequest` — ARObject — R23-11 markdown · Table 5.40 (CP_TPS_BSWModuleDescriptionTemplate), p.103
  - module: M2/AUTOSARTemplates/BswModuleTemplate/BswBehavior.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BswDataReceptionPolicy` — (abstract; Table 5.42 renders no Base row — src intake bases BswApiOptions + VariationPointCapable, XSD group-only) — R23-11 markdown · Table 5.42 (CP_TPS_BSWModuleDescriptionTemplate)
  - module: M2/AUTOSARTemplates/BswModuleTemplate/BswBehavior.py (class EXISTS in src, unstamped — queued per Rule 0016.4 "exists is not a stamp")
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BswQueuedDataReceptionPolicy` — BswDataReceptionPolicy — R23-11 markdown · Table 5.43 (CP_TPS_BSWModuleDescriptionTemplate), p.105
  - module: M2/AUTOSARTemplates/BswModuleTemplate/BswBehavior.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BswAsynchronousServerCallReturnsEvent` — BswScheduleEvent — R23-11 markdown · Table 5.36 (CP_TPS_BSWModuleDescriptionTemplate), p.98
  - module: M2/AUTOSARTemplates/BswModuleTemplate/BswBehavior.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BswDataReceivedEvent` — BswScheduleEvent — R23-11 markdown · Table 5.37 (CP_TPS_BSWModuleDescriptionTemplate), p.99
  - module: M2/AUTOSARTemplates/BswModuleTemplate/BswBehavior.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BswInternalTriggerOccurredEvent` — BswScheduleEvent — R23-11 markdown · Table 5.29 (CP_TPS_BSWModuleDescriptionTemplate), p.91
  - module: M2/AUTOSARTemplates/BswModuleTemplate/BswBehavior.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BswModeManagerErrorEvent` — BswScheduleEvent — R23-11 markdown · Table 5.33 (CP_TPS_BSWModuleDescriptionTemplate), p.95
  - module: M2/AUTOSARTemplates/BswModuleTemplate/BswBehavior.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BswModeSwitchedAckEvent` — BswScheduleEvent — R23-11 markdown · Table 5.32 (CP_TPS_BSWModuleDescriptionTemplate), p.95
  - module: M2/AUTOSARTemplates/BswModuleTemplate/BswBehavior.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BswTimingEvent` — BswScheduleEvent — R23-11 markdown · Table 5.25 (CP_TPS_BSWModuleDescriptionTemplate), p.89
  - module: M2/AUTOSARTemplates/BswModuleTemplate/BswBehavior.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BswEntryRelationshipEnum` — AREnum — R23-11 markdown · Table 4.20 (CP_TPS_BSWModuleDescriptionTemplate), p.52
  - module: M2/AUTOSARTemplates/BswModuleTemplate/BswInterfaces.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BswEntryRelationship` — ARObject — R23-11 markdown · Table 4.19 (CP_TPS_BSWModuleDescriptionTemplate), p.51
  - module: M2/AUTOSARTemplates/BswModuleTemplate/BswInterfaces.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BswEntryRelationshipSet` — ARElement — R23-11 markdown · Table 4.18 (CP_TPS_BSWModuleDescriptionTemplate), p.51
  - module: M2/AUTOSARTemplates/BswModuleTemplate/BswInterfaces.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BswModuleClientServerEntry` — Referrable — R23-11 markdown · Table 4.21 (CP_TPS_BSWModuleDescriptionTemplate), p.54
  - module: M2/AUTOSARTemplates/BswModuleTemplate/BswInterfaces.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `BswModuleDependency` — Identifiable — R23-11 markdown · Table 4.17 (CP_TPS_BSWModuleDescriptionTemplate), p.48
  - module: M2/AUTOSARTemplates/BswModuleTemplate/BswInterfaces.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `SwcBswRunnableMapping` — ARObject — R23-11 markdown · Table 5.47 (CP_TPS_BSWModuleDescriptionTemplate), p.110
  - module: M2/AUTOSARTemplates/CommonStructure/SwcBswMapping.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `SwcBswSynchronizedModeGroupPrototype` — ARObject — R23-11 markdown · Table 5.48 (CP_TPS_BSWModuleDescriptionTemplate), p.111
  - module: M2/AUTOSARTemplates/CommonStructure/SwcBswMapping.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `SwcBswSynchronizedTrigger` — ARObject — R23-11 markdown · Table 5.49 (CP_TPS_BSWModuleDescriptionTemplate), p.111
  - module: M2/AUTOSARTemplates/CommonStructure/SwcBswMapping.py
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)
