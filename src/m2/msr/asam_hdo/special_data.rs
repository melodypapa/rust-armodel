// M2::MSR::AsamHdo::SpecialData
//
// Spec classes `Sd`, `Sdf`, `Sdg`, `SdgCaption`, `SdgContents` (P0 design
// §5). `Sd` carries an explicit `xml_space: Option<XmlSpace>` field — this is
// how whitespace fidelity is preserved in the round trip. Implemented in
// step 2.
