# Sync todo: Group 7 — ECU resource, Crypto/IDS, DoIP, Firewall, remaining

Input: `Group 7 — ECU resource, Crypto/IDS, DoIP, Firewall, remaining` of `docs/examples/sync_class_groups.md` · Generated: 2026-08-30 · Queue order = row order
(resume = first class row still `[ ]`; all class rows `[x]` = sync finished — Rule 0009.4)

## Queue (dependency-first)

- [ ] `ComponentInCompositionInstanceRef` (re-sync · **Rule 0012.3 drift misclassified, spec table found 2026-09-22** · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · **Table D.13** (identical sibling copies: TimingExtensions Table D.19; R4.3.1 swc D.14 / TimingExtensions B.14) · in src (SWComponentTemplate/Composition/InstanceRefs.py) — re-run the full 9-step against the table, then re-issue the marker as · **CORRECTION 2026-09-22: Table D.13 has 3 attrs (base, contextComponent ordered, targetComponent) — the earlier "5 members" note (contextPPort/targetModeGroup) belonged to a neighbouring instance-ref table, not D.13 (all four sibling copies re-checked)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `SdClientConfig` (re-sync · **Rule 0012.3 drift misclassified, spec table found 2026-09-22** · **R4.3.1 fallback** (no R23-11 table) · autosar/R4.3.1/markdown/AUTOSAR_TPS_SystemTemplate.md · **Table 6.172**, PDF p.356 · in src (SystemTemplate/Fibex/Fibex4Ethernet/EthernetTopology.py) — re-run the full 9-step against the R4.3.1 table and re-issue the marker as with release column R4.3.1 (Rule 0016.3) · Step 1 must diff every existing XSD-based member against the R4.3.1 text — drift expected both ways
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `HwAttributeDef` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_ECUResourceTemplate · Table 2.13 · *(existing member)*)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `HwCategory` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_ECUResourceTemplate · Table 2.11 · after `HwAttributeDef` (aggr `hwAttributeDef`))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `HwAttributeValue` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_ECUResourceTemplate · Table 2.2)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `HwAttributeLiteralDef` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_ECUResourceTemplate · Table 2.14)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `CryptoKeySlot` (tracker input · R23-11 markdown · AUTOSAR_FO_TPS_SecurityExtractTemplate · **Table B.5** (**spec table found 2026-09-22 re-verification — NOT XSD-only**; appendix letter-numbered table, missed by earlier numeric-regex search; Step 1 Phase 0 correction below)) · **9-step reset 2026-09-22** — the earlier partial pass (Steps 1-8) ran against Table B.5 but created the four member types from XSD; those types are verified genuinely XSD-only (no tables in either corpus) and stand — the CLASS re-sync starts fresh from Step 1 for a clean table-based pass before batch 9b
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `AbstractDoIpLogicAddressProps` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SystemTemplate · Table 6.208) — **stamped 2026-09-26
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DoIpLogicTargetAddressProps` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SystemTemplate · Table 6.209) — **stamped 2026-09-26
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DoIpLogicTesterAddressProps` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SystemTemplate · Table 6.210) — **stamped 2026-09-26
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DoIpTpConfig` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SystemTemplate · Table 6.205 · after the DoIp props classes)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `FirewallRuleProps` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SystemTemplate · Table 6.235, p.584 (pdf_page.py verified) · after `FirewallRule` (ref target of its `matchingEgressRule`/`matchingIngressRule`) · spec facts (extracted 2026-08-31 from markdown l.15333-15342): Note = "Firewall rule that is defined by an action that is performed if the referenced pattern matches."; Base = ARObject; Aggregated by = StateDependentFirewall.firewallRuleProps (* aggr); 3 attributes: `action` (FirewallActionEnum, 0..1, attr — "Action that is performed by the firewall if the matching Rule is fulfilled."), `matchingEgressRule` (ordered) (FirewallRule, *, ref — "This element defines an egress rule expression against which the network traffic is matched."), `matchingIngressRule` (ordered) (FirewallRule, *, ref — "This element defines an ingress rule expression against which the network traffic is matched."))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `IdsPlatformInstantiation` (tracker input · R23-11 markdown · AUTOSAR_FO_TPS_SecurityExtractTemplate · **Table B.13** (**spec table found 2026-09-22 re-verification — NOT XSD-only**; appendix letter-numbered table, missed by earlier numeric-regex search — resolve in per-class Phase 0))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `IdsmModuleInstantiation` (tracker input · R23-11 markdown · AUTOSAR_FO_TPS_SecurityExtractTemplate · **Table B.14** (**spec table found 2026-09-22 re-verification — NOT XSD-only**; appendix letter-numbered table — resolve in per-class Phase 0))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `PlatformModuleEthernetEndpointConfiguration` (tracker input · R23-11 markdown · AUTOSAR_FO_TPS_SecurityExtractTemplate · **Table B.19** (**spec table found 2026-09-22 re-verification — NOT XSD-only**; appendix letter-numbered table — resolve in per-class Phase 0))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `CommunicationControllerMapping` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_SystemTemplate · Table 3.134 · aggregated by `ECUMapping` · **NOT in src** — class must be created when this row is synced)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `HwPortMapping` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_SystemTemplate · Table 3.135 · aggregated by `ECUMapping` · **NOT in src** — class must be created when this row is synced)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ECUMapping` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SystemTemplate · Table 3.133 · after both NEW classes above (aggrs))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `VariableDataPrototypeInSystemInstanceRef` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SystemTemplate · **Table B.3** (**spec table found 2026-09-22 re-verification — NOT XSD-only**; appendix letter-numbered table, same class of miss as Group2 D.17/D.4 — resolved in per-class Phase 0: table confirmed at markdown L25407, PDF p.1004 verified by direct pypdf search))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ComponentInSystemInstanceRef` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SystemTemplate · **Table B.1** (**spec table found 2026-09-22 re-verification — NOT XSD-only**; appendix letter-numbered table — resolved in per-class Phase 0: table confirmed at markdown L25342, PDF p.1000 verified by direct pypdf search))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `PortPrototypeBlueprintInitValue` (tracker input · R23-11 markdown · AUTOSAR_FO_TPS_StandardizationTemplate · Table 4.10 · *(existing member)*)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `PortPrototypeBlueprint` (tracker input · R23-11 markdown · AUTOSAR_FO_TPS_StandardizationTemplate · Table 4.9 · after `PortPrototypeBlueprintInitValue` (aggr `initValue`))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `Keyword` (tracker input · R4.3.1 markdown · AUTOSAR_TPS_StandardizationTemplate · Table 6.2 · *(existing member)*)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `KeywordSet` (tracker input · R4.3.1 markdown · AUTOSAR_TPS_StandardizationTemplate · Table 6.1 · after `Keyword` (aggr `keyword`))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DiagnosticServiceInstance` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_DiagnosticExtractTemplate · Table 4.26 · ref target of `DiagnosticServiceTable.serviceInstance` · **NOT in src** — class must be created when this row is synced)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DiagnosticServiceTable` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_DiagnosticExtractTemplate · Table 4.16 · after `DiagnosticServiceInstance` (ref `serviceInstance`))
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DiagnosticCommonElement` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_DiagnosticExtractTemplate · Table 4.1)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DiagnosticAuthRoleProxy` (dependency · discovered 2026-09-23 from DiagnosticAccessPermission closure · R23-11 markdown · AUTOSAR_CP_TPS_DiagnosticExtractTemplate · Table 4.33)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DiagnosticSession` (dependency · discovered 2026-09-23 from DiagnosticAccessPermission closure · R23-11 markdown · AUTOSAR_CP_TPS_DiagnosticExtractTemplate · Table 4.30 · incl. DiagnosticJumpToBootLoaderEnum Table 4.31 if absent)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DiagnosticSecurityLevel` (dependency · discovered 2026-09-23 from DiagnosticAccessPermission closure · R23-11 markdown · AUTOSAR_CP_TPS_DiagnosticExtractTemplate · Table 4.32)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DiagnosticEnvironmentalCondition` (dependency · discovered 2026-09-23 from DiagnosticAccessPermission closure · R23-11 markdown · AUTOSAR_CP_TPS_DiagnosticExtractTemplate · Table 4.35 · closure members DiagnosticEnvConditionFormula Table 4.36 / DiagnosticLogicalOperatorEnum Table 4.37 — create small ones in-pass if needed)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DiagnosticAccessPermission` (tracker input · missing member class of DiagnosticServiceInstance.accessPermission · R23-11 markdown · AUTOSAR_CP_TPS_DiagnosticExtractTemplate · Table 4.29 · after all 4 dependencies above)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)
