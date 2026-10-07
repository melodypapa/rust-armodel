//! The ARXML reader: document entry points (`load`), release detection
//! (`xsd_to_version`), the AR-PACKAGE walk (`read_ar_packages`,
//! `read_ar_package`), and the `read_element_payload` dispatch seam that
//! delegates to the per-domain reader modules (common, admin_data,
//! documentation, compu_method, data_constr, keyword, datatypes, base_types,
//! collection, life_cycle, physical_dimension, unit — see
//! docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
//! Mirrors py's `arxml_parser.py` method-for-method so the P2–P4 port stays
//! reviewable.

use std::io::BufRead;
use std::path::Path;
use std::sync::LazyLock;

use quick_xml::reader::Reader;

use crate::m2::autosar_templates::autosar_top_level_structure::Document;
use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_object::{
    ARObject, ElementRef,
};
use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_package::{
    ARPackageId, ReferenceBase,
};
use crate::m2::element_registry::{
    element_factory_for_tag, element_set_category, element_set_checksum, element_set_timestamp,
    element_set_uuid,
};
use crate::m2::msr::asam_hdo::admin_data::{AdminData, AdminDataId};
use crate::m2::msr::asam_hdo::special_data::{Sd, Sdf, Sdg, SdgCaption, SdgContents, SdgId};
use crate::m2::msr::documentation::block_elements::list_elements::{ARList, Item, ListEnum};
use crate::m2::msr::documentation::block_elements::pagination_and_view::{
    ChapterEnumBreak, KeepWithPreviousEnum, Paginateable,
};
use crate::m2::msr::documentation::text_model::block_elements::{
    DocumentationBlock, DocumentationBlockId,
};
use crate::m2::msr::documentation::text_model::language_data_model::{
    LLongName, LOverviewParagraph, LParagraph, LPlainText, XmlSpace,
};
use crate::m2::msr::documentation::text_model::multilanguage_data::{
    MultiLanguageOverviewParagraph, MultiLanguageOverviewParagraphId, MultiLanguageParagraph,
    MultiLanguagePlainText, MultilanguageLongName, MultilanguageLongNameId,
};
use crate::reader::abstract_arxml_reader::{
    build_dom_from_reader, find, find_all, get_child_element_optional_boolean,
    get_child_element_optional_ref_type, get_child_element_ref_type_list, get_child_element_string,
    get_short_name, Node, ParseError,
};
use crate::validation::ARXMLValidator;

mod admin_data;
mod base_types;
mod collection;
mod common;
mod compu_method;
mod data_constr;
mod datatypes;
mod dispatch_tables;
mod documentation;
mod keyword;
mod life_cycle;
mod physical_dimension;
mod unit;

/// py `ARXMLParser(options)` — `warning: true` (the default) collects warnings
/// and continues; `warning: false` fails on the first problem.
#[derive(Debug, Clone)]
pub struct ReaderOptions {
    pub warning: bool,
    /// Pre-parse XSD gate (py `options={"validate": False}` escape hatch).
    pub validate: bool,
}

impl Default for ReaderOptions {
    fn default() -> Self {
        Self {
            warning: true,
            validate: true,
        }
    }
}

/// py's default options (`warning: true`).
pub fn default_options() -> ReaderOptions {
    ReaderOptions::default()
}

/// py `ARXMLParser`
#[derive(Debug)]
pub struct ARXMLReader {
    options: ReaderOptions,
    warnings: Vec<String>,
}

impl Default for ARXMLReader {
    fn default() -> Self {
        Self::new(ReaderOptions::default())
    }
}

impl ARXMLReader {
    pub fn new(options: ReaderOptions) -> Self {
        Self {
            options,
            warnings: Vec::new(),
        }
    }

    /// Warnings collected so far (only populated in `warning: true` mode).
    pub fn get_warnings(&self) -> &[String] {
        &self.warnings
    }

    /// py `raiseError` — log-and-continue in warning mode, hard error otherwise.
    fn raise_error(&mut self, message: String) -> Result<(), ParseError> {
        if self.options.warning {
            self.warnings.push(message);
            Ok(())
        } else {
            Err(ParseError::InvalidElement {
                element: "ARXML".to_string(),
                reason: message,
            })
        }
    }

    /// py `notImplemented`
    fn not_implemented(&mut self, message: String) -> Result<(), ParseError> {
        self.raise_error(message)
    }

    /// py `load` — validates the bytes against the bundled release schema
    /// before any model construction (escape hatch: `ReaderOptions::validate`).
    pub fn load(&mut self, path: &Path, document: &mut Document) -> Result<(), ParseError> {
        let data = std::fs::read(path)?;
        self.validate_or_fail(&data)?;
        self.load_from_reader(Reader::from_reader(&data[..]), document)
    }

    /// Unresolvable schemas are NOT errors (spec §4 AMENDED): the writer's
    /// default `AUTOSAR_4-0-3.xsd` and legacy documents have no bundled
    /// schema — processing continues unvalidated in every mode.
    fn validate_or_fail(&mut self, data: &[u8]) -> Result<(), ParseError> {
        if !self.options.validate {
            return Ok(());
        }
        let Some(validator) = ARXMLValidator::for_document(data) else {
            return Ok(());
        };
        let errors = validator.validate(data);
        if errors.is_empty() {
            return Ok(());
        }
        for error in &errors {
            let message = format!(
                "schema validation ({}): line {}, col {}: {}",
                validator.release(),
                error.line,
                error.column,
                error.message
            );
            if self.options.warning {
                self.warnings.push(message);
            } else {
                return Err(ParseError::SchemaValidation {
                    count: errors.len(),
                    line: error.line,
                    column: error.column,
                    message: error.message.clone(),
                });
            }
        }
        Ok(())
    }

    fn load_from_reader<R: BufRead>(
        &mut self,
        reader: Reader<R>,
        document: &mut Document,
    ) -> Result<(), ParseError> {
        let root = build_dom_from_reader(reader)?;
        if root.name != "AUTOSAR" {
            return Err(ParseError::UnexpectedRoot(root.name.clone()));
        }

        // py detectNamespace + getAUTOSARInfo: P0 records the schema
        // location; the XSD file name maps to a release via py's
        // `xsd_to_version_mapping` (conftest.py), defaulting to R23-11.
        if let Some(location) = root.attrs.get("xsi:schemaLocation") {
            document.set_schema_location(location.clone());
        }
        let xsd = document
            .get_schema_location()
            .split_whitespace()
            .nth(1)
            .unwrap_or("");
        document.set_ar_release(xsd_to_version(xsd));

        // py load: document.setAdminData(self.getAdminData(root, "ADMIN-DATA"))
        if let Some(admin_data_node) = find(&root, "ADMIN-DATA") {
            let admin_data = self.read_admin_data(admin_data_node, document)?;
            document.set_admin_data(admin_data);
        }
        // FILE-INFO-COMMENT and INTRODUCTION arrive in P1 (absent in this file).

        // py load: self.readARPackages(root, document)
        self.read_ar_packages(&root, None, document)?;
        Ok(())
    }

    /// py `readARPackages` — recurses into nested AR-PACKAGES. `parent` is
    /// `None` at the document root.
    fn read_ar_packages(
        &mut self,
        element: &Node,
        parent: Option<ARPackageId>,
        document: &mut Document,
    ) -> Result<(), ParseError> {
        if let Some(packages_node) = find(element, "AR-PACKAGES") {
            for child in find_all(packages_node, "*") {
                if child.name == "AR-PACKAGE" {
                    self.read_ar_package(child, parent, document)?;
                } else {
                    self.not_implemented(format!("Unsupported ARPackage <{}>", child.name))?;
                }
            }
        }
        Ok(())
    }

    /// py `readARPackage` — allocates the package first (so children can link
    /// to it), then fills its own fields in one scoped arena borrow, then
    /// recurses.
    fn read_ar_package(
        &mut self,
        element: &Node,
        parent: Option<ARPackageId>,
        document: &mut Document,
    ) -> Result<ARPackageId, ParseError> {
        let short_name = get_short_name(element)?;
        let id = document.add_ar_package(parent, &short_name);

        // Scalar pieces are read before the arena borrow so the fill below is
        // a single scoped block (`docs/code_guide.md` §4).
        let admin_data = match find(element, "ADMIN-DATA") {
            Some(admin_data_node) => Some(self.read_admin_data(admin_data_node, document)?),
            None => None,
        };
        // py readIdentifiable → readARObject: the package's own S/T attrs.
        let checksum = element.attrs.get("S").cloned();
        let timestamp = element.attrs.get("T").cloned();
        let uuid = element.attrs.get("UUID").cloned();
        let category = get_child_element_string(element, "CATEGORY").map(str::to_string);
        // py readIdentifiable on AR-PACKAGE: LONG-NAME/DESC/INTRODUCTION are
        // read before the arena borrow so the fill below stays one scoped block.
        let long_name = self.get_multilanguage_long_name(element, document)?;
        let desc = self.get_multi_language_overview_paragraph(element, document)?;
        let introduction = self.get_documentation_block(element, document)?;

        if let Some(package) = document.ar_packages.get_mut(id) {
            if let Some(admin_data) = admin_data {
                package.set_admin_data(admin_data);
            }
            if let Some(checksum) = checksum {
                package.set_checksum(checksum);
            }
            if let Some(timestamp) = timestamp {
                package.set_timestamp(timestamp);
            }
            if let Some(uuid) = uuid {
                package.set_uuid(uuid);
            }
            if let Some(category) = category {
                package.set_category(category);
            }
            // ARPackage forwards only a subset; the payload fields live on
            // its CollectableElement → Identifiable / MultilanguageReferrable
            // bases.
            let identifiable = package.base_mut().base_mut();
            if let Some(long_name) = long_name {
                identifiable.base_mut().set_long_name(long_name);
            }
            if let Some(desc) = desc {
                identifiable.set_desc(desc);
            }
            if let Some(introduction) = introduction {
                identifiable.set_introduction(introduction);
            }
        }

        // py readARPackageElements — dispatch through the P1 tag→constructor
        // registry. The common Identifiable parts (S/T attributes, UUID,
        // SHORT-NAME, CATEGORY) are read; per-class payload (DESC, ADMIN-DATA,
        // fields) is the P2–P4 mass port.
        if let Some(elements_node) = find(element, "ELEMENTS") {
            for child in find_all(elements_node, "*") {
                let factory = match element_factory_for_tag(&child.name) {
                    Some(factory) => factory,
                    None => {
                        self.not_implemented(format!(
                            "Unsupported ARPackage element <{}>",
                            child.name
                        ))?;
                        continue;
                    }
                };
                let short_name = get_short_name(child)?;
                let element_ref = factory(document, Some(id), &short_name);
                if let Some(checksum) = child.attrs.get("S") {
                    element_set_checksum(document, &element_ref, checksum);
                }
                if let Some(timestamp) = child.attrs.get("T") {
                    element_set_timestamp(document, &element_ref, timestamp);
                }
                if let Some(uuid) = child.attrs.get("UUID") {
                    element_set_uuid(document, &element_ref, uuid);
                }
                if let Some(category) = get_child_element_string(child, "CATEGORY") {
                    element_set_category(document, &element_ref, category);
                }
                // py readARPackageElements dispatches per class after the
                // common Identifiable parts; per-class payload lands arm by
                // arm (Tasks 4-9).
                self.read_element_payload(child, element_ref, document)?;
            }
        }
        // py readReferenceBases
        if let Some(bases_node) = find(element, "REFERENCE-BASES") {
            for base_node in find_all(bases_node, "REFERENCE-BASE") {
                let mut base = ReferenceBase::new();
                self.read_ar_object(base_node, base.base_mut());
                if let Some(label) = get_child_element_string(base_node, "SHORT-LABEL") {
                    base.set_short_label(label);
                }
                if let Some(value) = get_child_element_optional_boolean(base_node, "IS-DEFAULT") {
                    base.set_is_default(value);
                }
                if let Some(value) = get_child_element_optional_boolean(base_node, "IS-GLOBAL") {
                    base.set_is_global(value);
                }
                if let Some(value) =
                    get_child_element_optional_boolean(base_node, "BASE-IS-THIS-PACKAGE")
                {
                    base.set_base_is_this_package(value);
                }
                // RefType values live in the arena; the model links by id.
                for r#ref in get_child_element_ref_type_list(
                    base_node,
                    "GLOBAL-IN-PACKAGE-REFS/GLOBAL-IN-PACKAGE-REF",
                ) {
                    let ref_id = document.ref_types.insert(r#ref);
                    base.push_global_in_package_ref(ref_id);
                }
                for global in find_all(base_node, "GLOBAL-ELEMENTS/GLOBAL-ELEMENT") {
                    if let Some(text) = &global.text {
                        base.push_global_elements(text.as_str());
                    }
                }
                if let Some(r#ref) = get_child_element_optional_ref_type(base_node, "PACKAGE-REF") {
                    let ref_id = document.ref_types.insert(r#ref);
                    base.set_package_ref(ref_id);
                }
                let base_id = document.reference_bases.insert(base);
                if let Some(package) = document.ar_packages.get_mut(id) {
                    package.push_reference_base(base_id);
                }
            }
        }

        self.read_ar_packages(element, Some(id), document)?;
        Ok(id)
    }

    /// py's per-class read dispatch (the tag→create+read chain in
    /// readARPackageElements). The generated tag table serves ported
    /// families (P5); the shrinking hand match below is the strangler
    /// remainder until the last arm moves.
    fn read_element_payload(
        &mut self,
        element: &Node,
        element_ref: ElementRef,
        document: &mut Document,
    ) -> Result<(), ParseError> {
        if let Some(handler) = dispatch_tables::lookup_read_handler(element.name.as_str()) {
            return handler(self, element, element_ref, document);
        }
        match element_ref {
            ElementRef::ApplicationPrimitiveDataType(id) => {
                self.read_application_primitive_data_type(element, id, document)
            }
            ElementRef::ApplicationArrayDataType(id) => {
                self.read_application_array_data_type(element, id, document)
            }
            ElementRef::ApplicationRecordDataType(id) => {
                self.read_application_record_data_type(element, id, document)
            }
            ElementRef::ImplementationDataType(id) => {
                self.read_implementation_data_type(element, id, document)
            }
            ElementRef::Collection(id) => self.read_collection(element, id, document),
            ElementRef::LifeCycleInfoSet(id) => {
                self.read_life_cycle_info_set(element, id, document)
            }
            ElementRef::PhysicalDimension(id) => {
                self.read_physical_dimension(element, id, document)
            }
            ElementRef::Unit(id) => self.read_unit(element, id, document),
            // Remaining families land in later P2 batches.
            _ => Ok(()),
        }
    }
}

/// py `xsd_to_version_mapping` (tests/integration_tests/conftest.py) —
/// XSD file name to release, defaulting to R23-11 (P0 design §6).
fn xsd_to_version(xsd: &str) -> &'static str {
    match xsd {
        "autosar.xsd" => "3.2.3",
        "AUTOSAR_4-0-3.xsd" => "4.0.3",
        "AUTOSAR_4-1-0.xsd" => "4.1.0",
        "AUTOSAR_4-1-1.xsd" => "4.1.1",
        "AUTOSAR_4-1-2.xsd" => "4.1.2",
        "AUTOSAR_4-1-3.xsd" => "4.1.3",
        "AUTOSAR_4-2-1.xsd" => "4.2.1",
        "AUTOSAR_4-2-2.xsd" => "4.2.2",
        "AUTOSAR_00043.xsd" => "4.3.0",
        "AUTOSAR_00044.xsd" => "4.3.1",
        "AUTOSAR_00046.xsd" => "4.4.0",
        "AUTOSAR_00048.xsd" => "R19-11",
        "AUTOSAR_00049.xsd" => "R20-11",
        "AUTOSAR_00050.xsd" => "R21-11",
        "AUTOSAR_00051.xsd" => "R22-11",
        "AUTOSAR_00052.xsd" => "R23-11",
        "AUTOSAR_00053.xsd" => "R24-11",
        _ => "R23-11",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_object::ElementRef;

    const SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<AUTOSAR xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd">
  <ADMIN-DATA>
    <USED-LANGUAGES>
      <L-10 L="EN" xml:space="preserve">English</L-10>
    </USED-LANGUAGES>
    <SDGS>
      <SDG GID="demo">
        <SD GID="purpose" xml:space="preserve">special   data</SD>
      </SDG>
    </SDGS>
  </ADMIN-DATA>
  <AR-PACKAGES>
    <AR-PACKAGE>
      <SHORT-NAME>Outer</SHORT-NAME>
      <AR-PACKAGES>
        <AR-PACKAGE>
          <SHORT-NAME>Inner</SHORT-NAME>
        </AR-PACKAGE>
      </AR-PACKAGES>
    </AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#;

    fn parse_sample() -> Document {
        let mut document = Document::new();
        ARXMLReader::new(default_options())
            .load_from_reader(Reader::from_str(SAMPLE), &mut document)
            .unwrap();
        document
    }

    #[test]
    fn load_sets_schema_location_and_release() {
        let document = parse_sample();
        assert_eq!(
            document.get_schema_location(),
            "http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd"
        );
        assert_eq!(document.get_ar_release(), "R21-11");
    }

    #[test]
    fn load_rejects_wrong_root() {
        let mut document = Document::new();
        let result = ARXMLReader::new(default_options()).load_from_reader(
            Reader::from_str("<?xml version=\"1.0\"?><WRONG/>"),
            &mut document,
        );
        assert!(matches!(result, Err(ParseError::UnexpectedRoot(_))));
    }

    const REFERENCE_BASE_SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<AUTOSAR xmlns="http://autosar.org/schema/r4.0" xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00052.xsd">
  <AR-PACKAGES>
    <AR-PACKAGE>
      <SHORT-NAME>Pkg</SHORT-NAME>
      <REFERENCE-BASES>
        <REFERENCE-BASE>
          <SHORT-LABEL>AUTOSAR</SHORT-LABEL>
          <IS-DEFAULT>false</IS-DEFAULT>
          <IS-GLOBAL>true</IS-GLOBAL>
          <BASE-IS-THIS-PACKAGE>false</BASE-IS-THIS-PACKAGE>
          <PACKAGE-REF DEST="AR-PACKAGE">/AUTOSAR/Platform</PACKAGE-REF>
        </REFERENCE-BASE>
      </REFERENCE-BASES>
    </AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#;

    #[test]
    fn reference_bases_parse_into_the_arena() {
        let mut document = Document::new();
        ARXMLReader::new(default_options())
            .load_from_reader(Reader::from_str(REFERENCE_BASE_SAMPLE), &mut document)
            .unwrap();

        let pkg = document
            .get_ar_package(document.get_ar_packages()[0])
            .unwrap();
        let base_id = pkg.get_reference_bases()[0];
        let base = document.reference_bases.get(base_id).unwrap();
        assert_eq!(base.get_short_label(), Some("AUTOSAR"));
        assert_eq!(base.get_is_default(), Some(false));
        assert_eq!(base.get_is_global(), Some(true));
        assert_eq!(base.get_base_is_this_package(), Some(false));
        let package_ref = document
            .ref_types
            .get(base.get_package_ref().unwrap())
            .unwrap();
        assert_eq!(package_ref.get_dest(), Some("AR-PACKAGE"));
        assert_eq!(package_ref.get_value(), Some("/AUTOSAR/Platform"));
    }

    const SCHEMA_VALID_SAMPLE: &str = r#"<?xml version="1.0"?>
<AUTOSAR xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00046.xsd">
  <AR-PACKAGES>
    <AR-PACKAGE><SHORT-NAME>P</SHORT-NAME></AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#;

    const SCHEMA_INVALID_SAMPLE: &str = r#"<?xml version="1.0"?>
<AUTOSAR xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00046.xsd">
  <ADMIN-DATA>
    <SDGS><SDG/></SDGS>
    <USED-LANGUAGES><L-10 L="EN">English</L-10></USED-LANGUAGES>
  </ADMIN-DATA>
  <AR-PACKAGES>
    <AR-PACKAGE><SHORT-NAME>P</SHORT-NAME></AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#;

    /// `NamedTempFile` deletes on drop, so the path is kept (leaked into
    /// the OS temp dir) and returned.
    fn write_temp(contents: &str) -> std::path::PathBuf {
        let file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(file.path(), contents).unwrap();
        file.into_temp_path().keep().unwrap()
    }

    #[test]
    fn load_rejects_schema_invalid_file_in_strict_mode() {
        // Control: the schema-valid counterpart passes the strict gate.
        let valid_path = write_temp(SCHEMA_VALID_SAMPLE);
        let mut valid_document = Document::new();
        ARXMLReader::new(ReaderOptions {
            warning: false,
            validate: true,
        })
        .load(&valid_path, &mut valid_document)
        .unwrap();

        let path = write_temp(SCHEMA_INVALID_SAMPLE);
        let mut document = Document::new();
        let result = ARXMLReader::new(ReaderOptions {
            warning: false,
            validate: true,
        })
        .load(&path, &mut document);
        let Err(ParseError::SchemaValidation { count, .. }) = result else {
            panic!("expected SchemaValidation, got {result:?}");
        };
        assert!(count >= 1);
    }

    #[test]
    fn load_warns_and_continues_on_schema_invalid_file_in_warning_mode() {
        let path = write_temp(SCHEMA_INVALID_SAMPLE);
        let mut document = Document::new();
        let mut reader = ARXMLReader::new(default_options());
        reader.load(&path, &mut document).unwrap();
        assert!(reader
            .get_warnings()
            .iter()
            .any(|w| w.contains("schema validation")));
        assert_eq!(document.get_ar_packages().len(), 1); // the document still parsed
    }

    #[test]
    fn validate_false_disables_the_gate() {
        let path = write_temp(SCHEMA_INVALID_SAMPLE);
        let mut document = Document::new();
        ARXMLReader::new(ReaderOptions {
            warning: false,
            validate: false,
        })
        .load(&path, &mut document)
        .unwrap();
    }

    #[test]
    fn load_still_works_for_unresolvable_schemas() {
        let path = write_temp(SAMPLE); // existing SAMPLE: AUTOSAR_00050.xsd — not bundled
        let mut document = Document::new();
        let mut reader = ARXMLReader::new(ReaderOptions {
            warning: false,
            validate: true,
        });
        reader.load(&path, &mut document).unwrap();
        assert!(reader.get_warnings().is_empty());
    }

    #[test]
    fn packages_nest_through_the_document_factory() {
        let document = parse_sample();
        assert_eq!(document.get_ar_packages().len(), 1);

        let outer_id = document.get_ar_packages()[0];
        let outer = document.get_ar_package(outer_id).unwrap();
        assert_eq!(outer.get_short_name(), Some("Outer"));
        assert!(outer.get_parent().is_none());

        let inner_id = outer.get_ar_packages()[0];
        let inner = document.get_ar_package(inner_id).unwrap();
        assert_eq!(inner.get_short_name(), Some("Inner"));
        assert!(
            matches!(inner.get_parent(), Some(ElementRef::ARPackage(parent)) if parent == outer_id)
        );
    }
}
