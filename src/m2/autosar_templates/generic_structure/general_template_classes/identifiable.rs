// M2::AUTOSARTemplates::GenericStructure::GeneralTemplateClasses::Identifiable
//
// Spec classes `Referrable`, `MultilanguageReferrable`, `Identifiable`,
// `Describable` (P0 design §5). Modelled as composition: each struct embeds
// its base (`struct Identifiable { base: Referrable, … }`), see
// `docs/code_guide.md` §3. Implemented in step 2.
