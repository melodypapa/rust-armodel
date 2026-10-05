# Sync todo: Group 3 — Constants, CompuMethod, DataDictionary, Documentation

Input: `Group 3 — Constants, CompuMethod, DataDictionary, Documentation` of `docs/examples/sync_class_groups.md` · Generated: 2026-08-30 · Queue order = row order
(resume = first class row still `[ ]`; all class rows `[x]` = sync finished — Rule 0009.4)

## Queue (dependency-first)

- [ ] `ChapterEnumBreak` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.61 · enum member type of `Paginateable.chapterBreak`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `KeepWithPreviousEnum` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.76 · enum member type of `Paginateable.keepWithPrevious`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `Paginateable` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.75 (mixin) · parent of `MultiLanguageParagraph`, `MlFigure`, `MsrQueryChapter`, `MsrQueryTopic1`, `MsrQueryP1` below · attrs BREAK/KEEP-WITH-PREVIOUS) — verified R23-11 (model+docstrings/checklist/parser/writer 20e6ee88, LIST aggregator coverage a06638fb)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `MultilanguageLongName` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 4.6 · member type of `GeneralAnnotation.label` below) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `GraphicFitEnum` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.21 · enum member type of `Graphic.editfit`/`fit`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `GraphicNotationEnum` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.22, p.305 · enum member type of `Graphic.notation` below · **was NOT in src — created** · 8 XSD literals BMP/EPS/GIF/JPG/PDF/PNG/SVG/TIFF, indices 0-7 · queued dependency-first per Rule 16.5) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `Graphic` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.20 · member type of `LGraphic.graphic` below · **15 spec attrs (all 0..1, all xml.attribute=true); src had only 6 — all 9 missing ones added** · Base `ARObject`+`EngineeringObject` both stamped ✓ · `GraphicFitEnum` ✓ · `GraphicNotationEnum` ✓) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `SingleLanguageLongName` · needed by `SingleLanguageReferrable.longName1` and `Xref.label1`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `SingleLanguageReferrable` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 4.12 (abstract) · **NOT in src** · Package M2::AUTOSARTemplates::GenericStructure::GeneralTemplateClasses::Identifiable · Base `ARObject , Referrable` · attr `longName1` SingleLanguageLongName 0..1 aggr · base of `Std`, `Xdoc`, `Xfile`, `XrefTarget`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `MimeTypeString` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · **Table 4.55** (`Primitive`, p.111) · **NOT in src** · **NOT queued anywhere before this audit** — missed by all earlier closure passes · Package `M2::AUTOSARTemplates::GenericStructure::GeneralTemplateClasses::PrimitiveTypes` → `PrimitiveTypes.py` (same file as `UriString`) · missing **primitive** → implement as an `ARLiteral` subclass (Rule 0001.10) · member type of `Url.mimeType` below → queued immediately before `Url` (dependency-first, Rule 0016.5)) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `Url` (dependency · **XSD-only** (no markdown/PDF table) · AUTOSAR_00052.xsd `complexType name="URL"` line 128502 (+ `attributeGroup URL` line 128494; attr `MIME-TYPE`) · **NOT in src** · member type of `Std.url` / `Xdoc.url` / `Xfile.url` (0..1 aggr) · 16.4 decision: **Derive-from-XSD** → carries · deps: `mimeType` MimeTypeString queued immediately above (added 2026-09-12) / value type `UriString` stamped ✓)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `Br` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.33 · **NOT in src** · Package M2::MSR::Documentation::TextModel::InlineTextElements · Base `ARObject` · no own attributes · member type of `MixedContentForParagraph.br`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `Std` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.37 · **created** · Package M2::MSR::Documentation::TextModel::InlineTextElements · Base `ARObject , Referrable , SingleLanguageReferrable` · attrs date(DateTime)/position(String)/state(String)/subtitle(String)/url(Url) all 0..1 · member type of `MixedContentForParagraph.std`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `Xdoc` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.40 · **created** · Package M2::MSR::Documentation::TextModel::InlineTextElements · Base `ARObject , Referrable , SingleLanguageReferrable` · attrs date/number/position/publisher/state/url all 0..1 · member type of `MixedContentForParagraph.xdoc`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `Xfile` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.41 · **created** · Package M2::MSR::Documentation::TextModel::InlineTextElements · Base `ARObject , Referrable , SingleLanguageReferrable` · attrs tool/toolVersion/url all 0..1 · member type of `MixedContentForParagraph.xfile`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `XrefTarget` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.43 · **created** · Package M2::MSR::Documentation::TextModel::InlineTextElements · Base `ARObject , Referrable , SingleLanguageReferrable` · no own attributes · member type of `MixedContentForParagraph.xrefTarget`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ResolutionPolicyEnum` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.46 · **created** · Package M2::MSR::Documentation::TextModel::InlineAttributeEnums · AREnum; literals noSloppy/sloppy; member type of `Xref.resolutionPolicy`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ShowContentEnum` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.47 · **created** · Package M2::MSR::Documentation::TextModel::InlineAttributeEnums · AREnum; literals noShowContent/showContent; member type of `Xref.showContent`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ShowResourceAliasNameEnum` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.48 · **created** · Package M2::MSR::Documentation::TextModel::InlineAttributeEnums · AREnum; literals noShowAliasName/showAliasName; member type of `Xref.showResourceAliasName`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ShowResourceCategoryEnum` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.49 · **created** · Package M2::MSR::Documentation::TextModel::InlineAttributeEnums · AREnum; literals noShowCategory/showCategory; member type of `Xref.showResourceCategory`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ShowResourceLongNameEnum` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.50 · **created** · Package M2::MSR::Documentation::TextModel::InlineAttributeEnums · AREnum; literals noShowLongName/showLongName; member type of `Xref.showResourceLongName`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ShowResourceNumberEnum` (dependency · R23-11 markdown · Table 9.51 · **created** · AREnum · member type of `Xref.showResourceNumber`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ShowResourcePageEnum` (dependency · R23-11 markdown · Table 9.52 · **created** · AREnum · member type of `Xref.showResourcePage`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ShowResourceShortNameEnum` (dependency · R23-11 markdown · Table 9.53 · **created** · AREnum · member type of `Xref.showResourceShortName`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ShowResourceTypeEnum` (dependency · R23-11 markdown · Table 9.54 · **created** · AREnum · member type of `Xref.showResourceType`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ShowSeeEnum` (dependency · R23-11 markdown · Table 9.55 · **created** · AREnum · member type of `Xref.showSee`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `Xref` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.42 · **created** · Package M2::MSR::Documentation::TextModel::InlineTextElements · Base `ARObject` · 12 attrs: label1(SingleLanguageLongName aggr)/referrable(Referrable ref)/resolutionPolicy + showContent + showResourceAliasName/Category/LongName/Number/Page/ShortName/Type + showSee (enums, all 0..1) · member type of `MixedContentForParagraph.xref`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `MixedContentForParagraph` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.2 (abstract, <<atpMixedString>>) · **created** · Package M2::MSR::Documentation::TextModel::InlineTextModel · Base `ARObject` · 13 attrs: br(Br)/e(EmphasisText ✓)/ft(SlParagraph)/ie(IndexEntry ✓)/std(Std)/sub(Superscript ✓)/sup(Superscript ✓)/trace(Traceable ref ✓)/tt(Tt ✓)/xdoc(Xdoc)/xfile(Xfile)/xref(Xref)/xrefTarget(XrefTarget) · parent of `LParagraph` and `SlParagraph`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `SlParagraph` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · **Table E.71** (appendix letter-numbered table; class table has no own attributes) · **created** · Package M2::MSR::Documentation::TextModel::SingleLanguageData · Base `ARObject , MixedContentForParagraph` (cyclic with MixedContentForParagraph; resolved by syncing this class after the base) · `MixedContentForParagraph.ft` member type · XSD `SL-PARAGRAPH` defines mixed text, inherited paragraph content, and deprecated optional `L` attribute; marker deferred pending 9b) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `LParagraph` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.92 (p.348) · member type of `MultiLanguageParagraph.l1` below · **no own spec attributes (`-` row); content comes from `Base` = ARObject , LanguageSpecific , MixedContentForParagraph** · unblocked once `MixedContentForParagraph` stamped) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `FrameEnum` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.65 · Package M2::MSR::Documentation::BlockElements::OasisExchangeTable → `OasisExchangeTable.py` · AREnum; literals all/bottom/none/sides/top/topbot indices 0-5 · aggregated by `Table.frame` + `MlFigure.frame`); only the checklist `release` column was added (Rule 0012.3 drift-pass fix)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `AlignEnum` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.67 · **NOT in src — created** · Package OasisExchangeTable → `OasisExchangeTable.py` · AREnum; literals center/justify/left/right indices 0-3 · aggregated by `Colspec.align`, `Entry.align`, `Tgroup.align`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ValignEnum` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.69 · **NOT in src — created** · Package OasisExchangeTable → `OasisExchangeTable.py` · AREnum; literals bottom/middle/top indices 0-2 · aggregated by `Entry.valign`, `Row.valign`, `Tbody.valign`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `OrientEnum` (dependency · **XSD-only** (no R23-11 § no R4.3.1 markdown/PDF table) · `AUTOSAR_00052.xsd` `simpleType name="ORIENT-ENUM--SIMPLE"` line 140997 · **NOT in src — created** · Package OasisExchangeTable → `OasisExchangeTable.py` · AREnum; literals land/port · member type of `Table.orient` · carries) — verified XSD
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `TableSeparatorString` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.72 (`Primitive`) · **NOT in src — created** · Package OasisExchangeTable → `OasisExchangeTable.py` · **primitive** → `ARLiteral` subclass (Rule 0001.10); Tags `xml.xsd.customType=TABLE-SEPARATOR-STRING` `xml.xsd.pattern=[0-1]` `xml.xsd.type=string` · member type of every `colsep`/`rowsep` in the cluster) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `NameTokens` (dependency · **discovered 2026-09-14 during `DocumentViewSelectable` Step 1 closure audit (gap missed by the 2026-09-14 Table closure audit)** · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 4.56 (`Primitive`) · **NOT in src — created** · Package `M2::AUTOSARTemplates::GenericStructure::GeneralTemplateClasses::PrimitiveTypes` → `PrimitiveTypes.py` (same file as `NameToken`/`MimeTypeString`) · **primitive** → `ARLiteral` subclass (Rule 0001.10) · Tags `xml.xsd.customType=NMTOKENS-STRING` `xml.xsd.type=NMTOKENS` · member type of `DocumentViewSelectable.si` (Table 9.77, mult 1) · queued immediately before `DocumentViewSelectable` (Rule 0016.5)) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ViewTokens` (dependency · **discovered 2026-09-14 during `DocumentViewSelectable` Step 1 closure audit (gap missed by the 2026-09-14 Table closure audit)** · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.78 (`Primitive`) · **NOT in src — created** · Package `M2::AUTOSARTemplates::GenericStructure::GeneralTemplateClasses::PrimitiveTypes` → `PrimitiveTypes.py` · **primitive** → `ARLiteral` subclass (Rule 0001.10) · Tags `xml.xsd.customType=VIEW-TOKENS` `xml.xsd.pattern=(-?[a-zA-Z_]+)(+-?[a-zA-Z_]+)*` `xml.xsd.type=string` · member type of `DocumentViewSelectable.view` (Table 9.77, 0..1) · queued immediately before `DocumentViewSelectable` (Rule 0016.5)) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DocumentViewSelectable` (tracker input · **moved 2026-09-14 Table closure audit ahead of `Table`/`Row`** · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.77 · abstract `ARObject` mixin · **in src** (`PaginationAndView.py`) · needs 2 new attrs `si`(NameTokens 1)/`view`(ViewTokens 0..1) + reusable read/write helpers (Rule 0001.7) · **unblocked after `NameTokens` + `ViewTokens` synced** · Base of `Table`, `Row`, and `MultiLanguageParagraph`/`MlFigure`/`MsrQueryChapter`/`MsrQueryTopic1`/`MsrQueryP1`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `Colspec` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · **Table E.20** (appendix letter-numbered table; the markdown `Table E.21: Colspec` title is a rendering mislabel — that block is actually the `Compiler` class; Colspec located via PDF text search p.433) · Package M2::MSR::Documentation::BlockElements::OasisExchangeTable → `OasisExchangeTable.py` · Base ARObject; attrs align(AlignEnum 0..1)/colname(String 0..1)/colnum(String 0..1)/colsep(TableSeparatorString 0..1)/colwidth(String 0..1)/rowsep(TableSeparatorString 0..1) all attr · member type of `Tgroup.colspec`)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `Entry` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.71 · **NOT in src — created** · Package OasisExchangeTable · Base `ARObject` · 12 attrs align(AlignEnum)/bgcolor(String 1)/colname/colsep(TableSeparatorString)/entryContents(DocumentationBlock 1 aggr)/morerows/nameend/namest/rotate/rowsep/spanname/valign(ValignEnum) · member type of `Row.entry`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `Row` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.70 · **NOT in src — created** · Package OasisExchangeTable · Base `ARObject , DocumentViewSelectable , Paginateable` · attrs entry(Entry 1..* aggr)/rowsep(TableSeparatorString)/valign(ValignEnum) · member type of `Tbody.row`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `Tbody` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.68 · **NOT in src — created** · Package OasisExchangeTable · Base `ARObject` · attrs row(Row 1..* aggr)/valign(ValignEnum) · member type of `Tgroup.tbody`/`tfoot`/`thead`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `Tgroup` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.66 · **NOT in src — created** · Package OasisExchangeTable · Base `ARObject` · attrs align(AlignEnum)/cols(Integer 1)/colsep(TableSeparatorString)/colspec(Colspec * aggr)/rowsep(TableSeparatorString)/tbody(Tbody 1)/tfoot(Tbody 0..1)/thead(Tbody 0..1) · member type of `Table.tgroup`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `Table` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.63 · member type of `TopicContent.table` + `TraceableTable.table` · **NOT in src — created** · Package OasisExchangeTable · Base `ARObject , DocumentViewSelectable , Paginateable` · 10 attrs colsep(TableSeparatorString)/float(FloatEnum ✓)/frame(FrameEnum)/helpEntry(String)/orient(OrientEnum)/pgwide(NameToken)/rowsep(TableSeparatorString)/tableCaption(Caption ✓ aggr)/tabstyle(NameToken)/tgroup(Tgroup 1..* aggr) · **unblocked once the 10 dependency rows above are synced**)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `TraceableTable` (dependency · **XSD-only** (no R23-11 markdown 9.x/E.x table, no R4.3.1 table, no PDF table — verified 2026-09-16 via grep of all three corpora + pdf_page.py) · AUTOSAR_00052.xsd `complexType name="TRACEABLE-TABLE"` line 125345 (+ group line 125328) · member type of `TopicContent.traceableTable` · **NOT in src** — class must be created when this row is synced · 16.4 decision: Derive-from-XSD → carries) — verified XSD
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `TopicContent` (dependency · R23-11 markdown · **AUTOSAR_FO_TPS_GenericStructureTemplate Table E.81 (appendix letter-numbered table missed by numeric-regex tooling, same as Group2 D.17/D.4 cases)** · member type of `MsrQueryP1.msrQueryResultP1` below · <<atpMixed>> · `traceableTable` TraceableTable remains pending 16.4) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `MultiLanguageParagraph` (tracker input · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.4 · after `Paginateable` (parent) + `DocumentViewSelectable` (parent, moved above) + `LParagraph` (aggr `l1`)) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `AreaEnumNohref` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.18, p.301 · **NOT in src — created** · Package M2::MSR::Documentation::BlockElements::Figure → `Figure.py` · AREnum; literal nohref (XSD NOHREF) index 0 · aggregated by `Area.nohref` · queued dependency-first per Rule 0016.5) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `AreaEnumShape` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.19, p.302 · **NOT in src — created** · Package M2::MSR::Documentation::BlockElements::Figure → `Figure.py` · AREnum; literals circle/default/poly/rect (XSD CIRCLE/DEFAULT/POLY/RECT) indices 0-3 · aggregated by `Area.shape` · queued dependency-first per Rule 0016.5) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `Area` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.17, p.301 · member type of `Map.area` · **NOT in src — created** · 22 attrs (see 2026-09-16 audit note) · Base `ARObject` · deps `AreaEnumNohref` + `AreaEnumShape` queued immediately above) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `Map` (tracker input · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.23 · after `Area` (aggr `area`) — Area NOT in src, pending 16.4 below) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `LGraphic` (tracker input · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.25 · after `Graphic` (aggr `graphic`) + `Map` (aggr `map`) · parent `LanguageSpecific` stamped ✓ · *(existing member)*) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `MlFigure` (tracker input · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.24 · after `DocumentViewSelectable`+`Paginateable` (parents) + `LGraphic` (aggr `lGraphic`) · deps stamped: `figureCaption` Caption ✓ / `verbatim` MultiLanguageVerbatim ✓) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `MsrQueryResultChapter` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.87 · member type of `MsrQueryChapter.msrQueryResultChapter` · **NOT in src** — class must be created when this row is synced) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `MsrQueryChapter` (tracker input · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.84 · after `DocumentViewSelectable`+`Paginateable` (parents) · deps: `msrQueryProps` MsrQueryProps stamped ✓ / `msrQueryResultChapter` NOT in src — pending 16.4 below) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `MsrQueryResultTopic1` (dependency · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.88 · member type of `MsrQueryTopic1.msrQueryResultTopic1` · **created** · one ordered `topic1` aggregation) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `MsrQueryTopic1` (tracker input · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.83 · after `DocumentViewSelectable`+`Paginateable` (parents) · deps: `msrQueryProps` stamped ✓ / `MsrQueryResultTopic1` stamped ✓ · deferred stub completed) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `MsrQueryP1` (tracker input · R23-11 markdown · AUTOSAR_FO_TPS_GenericStructureTemplate · Table 9.82 · after `DocumentViewSelectable`+`Paginateable` (parents) + `TopicContent` (aggr `msrQueryResultP1`) · `msrQueryProps` stamped ✓ · deferred stub completed) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `CompuContent` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.63 (abstract) · parent of `CompuScales` below · member of `Compu.compuContent` below · existing stub completed) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `CompuConstContent` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.72 (abstract) · parent of `CompuConstTextContent`, `CompuConstNumericContent`, `CompuConstFormulaContent` · member type of `CompuConst.compuConstContentType`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `CompuConstTextContent` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.67 · subtype of `CompuConstContent`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `CompuConstNumericContent` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.68 · subtype of `CompuConstContent`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `CompuConstFormulaContent` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · **Table B.1** (appendix; stale queue citation was 5.70) · subtype of `CompuConstContent`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `CompuConst` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.71 · member type of `Compu.compuDefaultValue` + `CompuScale.compuInverseValue` below) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `CompuScaleContents` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.66 (abstract) · parent of `CompuScaleRationalFormula` and `CompuScaleConstantContents` · member type of `CompuScale.compuScaleContents` below) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `CompuNominatorDenominator` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.75) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `CompuRationalCoeffs` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.69 · member type of `CompuScaleRationalFormula.compuRationalCoeffs`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `CompuScaleRationalFormula` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.73 · subtype of `CompuScaleContents` · member type of `CompuScale.compuScaleContents`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `CompuScaleConstantContents` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.74 · subtype of `CompuScaleContents` · member type of `CompuScale.compuScaleContents`) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `Compu` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.62 · member type of `CompuMethod.compuInternalToPhys`/`compuPhysToInternal` below · after `CompuContent` (aggr `compuContent`) + `CompuConst` (aggr `compuDefaultValue`)) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `CompuMethod` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.61 · ref target of `SwAxisIndividual.compuMethod` below · after `Compu` (aggr `compuInternalToPhys`/`compuPhysToInternal`) · `unit` Unit stamped ✓) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `CompuScale` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.64 · member type of `CompuScales.compuScale` below · after `CompuConst` (aggr `compuInverseValue`) + `CompuScaleContents` (aggr `compuScaleContents`) · `desc` MultiLanguageOverviewParagraph stamped ✓) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `CompuScales` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.65 · after `CompuScale` (aggr `compuScale`, row added 2026-09-03) + `CompuContent` (parent)) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DataConstrRule` — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `DataConstr` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.82 · ref target of `SwAxisIndividual.dataConstr` below · after `DataConstrRule` (aggr `dataConstrRule`)) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `CompositeValueSpecification` — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `ArrayValueSpecification` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.111 · after `CompositeValueSpecification` (parent, Table 5.110) · `element` ValueSpecification stamped ✓) — completed with deviation; **stamped 2026-09-26
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `RecordValueSpecification` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.112 · after `CompositeValueSpecification` (parent, Table 5.110) · `field` ValueSpecification stamped ✓) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `CompositeRuleBasedValueArgument` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.136 · member type of `CompositeRuleBasedValueSpecification.compoundPrimitiveArgument` below · base ARObject, no complex members) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `CompositeRuleBasedValueSpecification` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.135 · after `CompositeValueSpecification` (aggr `argument`) + `CompositeRuleBasedValueArgument` (aggr `compoundPrimitiveArgument`) · parent `AbstractRuleBasedValueSpecification` stamped ✓) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `SwValueCont` — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `SwCalprmAxisSet`
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `SwAxisIndividual` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.50 · after `CompuMethod` (ref `compuMethod`) + `DataConstr` (ref `dataConstr`) · `inputVariableType` ApplicationPrimitiveDataType queued in Group2 ✓ · deps stamped: `swAxisGeneric` SwAxisGeneric ✓ / `swVariableRef` SwVariableRefProxy ✓ / `unit` Unit ✓ / parent SwCalprmAxisTypeProps ✓) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `SwAxisGrouped` — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `SwRecordLayoutGroupContent` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.100 · member type of `SwRecordLayoutGroup.swRecordLayoutGroupContentType` below)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `SwGenericAxisParamType` (dependency · R23-11 markdown · ref target of `SwRecordLayoutGroup.swGenericAxisParamType` + `SwRecordLayoutV.swGenericAxisParamType` · **NOT in src** — class must be created when this row is synced)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `SwRecordLayoutV` — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `SwRecordLayout` (dependency · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 5.97 · ref target of `SwRecordLayoutGroup.swRecordLayout` below · cyclic record-layout family) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `AsamRecordLayoutSemantics` — finished, stamped
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `RecordLayoutIteratorPoint` — finished, stamped
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `SwRecordLayoutGroup` — finished, stamped
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `GeneralAnnotation` (tracker input · R23-11 markdown · AUTOSAR_CP_TPS_SoftwareComponentTemplate · Table 4.56 (multiple tables — resolve in per-class Phase 0) · after `MultilanguageLongName` (aggr `label`) · `annotationText` DocumentationBlock stamped ✓) — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `FirewallActionEnum` (dependency · **re-queued 2026-09-22 — supersedes Group7's stale single-line 16.4 record** · **XSD-only re-confirmed** (no R23-11/R4.3.1 markdown Enumeration table, no PDF caption; the sys L49490+ Firewall rows are ECUC literal-mapping tables, not a class table; consumers `StateDependentFirewall.defaultAction` Table 6.234 + `FirewallRuleProps.action` Table 6.235) · AUTOSAR_00052.xsd complexType `FIREWALL-ACTION-ENUM` line 136671 ("List of actions that the Firewall is able to perform.") + `--SIMPLE` line 136683 · **in src** (AdaptivePlatform/PlatformModuleDeployment/Firewall/__init__.py, AREnum, checklist present, NO marker) · **known deviation to arbitrate in Step 1: src literal order BLOCK=index 0 / ALLOW=index 1 contradicts the XSD `--SIMPLE` order and both ECUC mapping tables, which list ALLOW first → fix to allow=ALLOW index 0, block=BLOCK index 1 (wire values uppercase per XSD)** · marker at 9b · AREnum — Steps 5/6 N/A (value form on consuming classes)) — verified XSD
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `CompuGenericMath` — row closed 2026-09-26 via already-verified short-circuit, no sync needed (old 5-column checklist gains release column on next drift pass, Rule 0012.3); 2026-09-26 location drift fix (Rule 0007, user-directed): class moved DataDefProperties.py → M2/MSR/AsamHdo/ComputationMethod.py per spec Package row M2::MSR::AsamHdo::ComputationMethod; 2026-09-26 **RE-SYNCED — short-circuit superseded** by full 9-step run (user gate: "run the full 9-steps again and verify everything"): G1–G5 fixed (LEVEL typed PrimitiveIdentifier, atpMixedString text round-trips, readARObject S/T symmetry, verbatim Tags tail, 6-col checklist), deviation-tracker entry added — verified R23-11
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)

- [ ] `Ref` — Primitive `Ref`, CP SWCT Table 5.35, p.318 — finished, stamped · dependency added 2026-09-27 (dependency-first: member type of `AbstractEnumerationValueVariationPoint.enumTableRef`, Group8) · NOT in src before the sync — v1 ships the unrelated legacy class `RefType`
  - module: M2/AUTOSARTemplates/GenericStructure/GeneralTemplateClasses/PrimitiveTypes.py (spec Package `M2::AUTOSARTemplates::GenericStructure::GeneralTemplateClasses::PrimitiveTypes`; placed directly before `RefType`)
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





- `Area` — not in `src` (NEW) · R23-11 markdown · Table 9.17 · **(NEW)**; 16.4 decision required: **Skip** (deviation row) or **Derive-from-XSD** (then move into the queue with a 9-step sub-checklist) → **QUEUED 2026-09-11** as a 9-step row before `Map`
- `MsrQueryResultChapter` — not in `src` (NEW) · R23-11 markdown · Table 9.87 · **(NEW)**; 16.4 decision required: **Skip** (deviation row) or **Derive-from-XSD** (then move into the queue with a 9-step sub-checklist) → **QUEUED 2026-09-11** as a 9-step row before `MsrQueryChapter`
- `MsrQueryResultTopic1` — not in `src` (NEW) · R23-11 markdown · Table 9.88 · **(NEW)**; 16.4 decision required: **Skip** (deviation row) or **Derive-from-XSD** (then move into the queue with a 9-step sub-checklist) → **QUEUED 2026-09-11** as a 9-step row before `MsrQueryTopic1`
- `SwGenericAxisParamType` — **added 2026-09-03 restructure** · not in `src` · R23-11 markdown · ref target of `SwRecordLayoutGroup.swGenericAxisParamType` + `SwRecordLayoutV.swGenericAxisParamType`; 16.4 decision required: **Skip** (deviation row) or **Derive-from-XSD** (then move into the queue with a 9-step sub-checklist) → **QUEUED 2026-09-11** as a 9-step row before `SwRecordLayoutV`
- `TraceableTable` — **added 2026-09-11 dependency audit** · not in `src` · R23-11 markdown · member type of `TopicContent.traceableTable` (the TopicContent row already said "traceableTable TraceableTable remains pending 16.4", but no entry existed here); 16.4 decision required: **Skip** (deviation row) or **Derive-from-XSD** (then move into the queue with a 9-step sub-checklist) → **QUEUED 2026-09-11** as a 9-step row before `TopicContent`

## Not queued





_(none)_
