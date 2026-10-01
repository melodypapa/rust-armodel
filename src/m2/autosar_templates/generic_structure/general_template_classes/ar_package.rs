// M2::AUTOSARTemplates::GenericStructure::GeneralTemplateClasses::ARPackage
//
// Spec classes `PackageableElement`, `ARElement`, `ARPackage`,
// `ReferenceBase` (P0 design §5). Children are stored as `Vec<Id<ARPackage>>`
// into a `Document`-owned arena — never `Vec<ARPackage>` by value
// (`docs/code_guide.md` §4). Implemented in step 2.
