//! `ARXMLParser::load` with release detection, `read_admin_data`,
//! `read_ar_packages` (P0 design §6). Mirrors py's `arxml_parser.py`
//! method-for-method so the P2–P4 port stays reviewable.

use std::io::BufRead;
use std::path::Path;

use quick_xml::reader::Reader;

use crate::m2::autosar_templates::autosar_top_level_structure::Document;
use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_object::ARObject;
use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_package::ARPackageId;
use crate::m2::msr::asam_hdo::admin_data::{AdminData, AdminDataId};
use crate::m2::msr::asam_hdo::special_data::{Sd, Sdf, Sdg, SdgCaption, SdgContents, SdgId};
use crate::m2::msr::documentation::text_model::language_data_model::{LPlainText, XmlSpace};
use crate::m2::msr::documentation::text_model::multilanguage_data::MultiLanguagePlainText;
use crate::parser::abstract_arxml_parser::{
    build_dom_from_reader, find, find_all, get_child_element_string, get_short_name, Node,
    ParseError,
};

/// py `ARXMLParser(options)` — `warning: true` (the default) collects warnings
/// and continues; `warning: false` fails on the first problem.
#[derive(Debug, Clone)]
pub struct ParserOptions {
    pub warning: bool,
}

impl Default for ParserOptions {
    fn default() -> Self {
        Self { warning: true }
    }
}

/// py's default options (`warning: true`).
pub fn default_options() -> ParserOptions {
    ParserOptions::default()
}

/// py `ARXMLParser`
#[derive(Debug)]
pub struct ARXMLParser {
    options: ParserOptions,
    warnings: Vec<String>,
}

impl Default for ARXMLParser {
    fn default() -> Self {
        Self::new(ParserOptions::default())
    }
}

impl ARXMLParser {
    pub fn new(options: ParserOptions) -> Self {
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

    /// py `load`
    pub fn load(&mut self, path: &Path, document: &mut Document) -> Result<(), ParseError> {
        self.load_from_reader(Reader::from_file(path)?, document)
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

    /// py `readARObject` — reads the `S` (checksum) and `T` (timestamp)
    /// attributes.
    fn read_ar_object(&mut self, element: &Node, ar_object: &mut ARObject) {
        if let Some(checksum) = element.attrs.get("S") {
            ar_object.set_checksum(checksum.as_str());
        }
        if let Some(timestamp) = element.attrs.get("T") {
            ar_object.set_timestamp(timestamp.as_str());
        }
    }

    /// py `readWhitespaceControlled` — reads the element's own `xml:space`.
    fn read_xml_space(&mut self, element: &Node) -> Result<Option<XmlSpace>, ParseError> {
        match element.attrs.get("xml:space").map(String::as_str) {
            Some(value) => match XmlSpace::try_from(value) {
                Ok(xml_space) => Ok(Some(xml_space)),
                Err(_) => {
                    let message = format!(
                        "Unsupported xml:space value <{value}> on <{}>",
                        element.name
                    );
                    self.not_implemented(message)?;
                    Ok(None)
                }
            },
            None => Ok(None),
        }
    }

    /// py `getAdminData` — returns the allocated `AdminData` id.
    fn read_admin_data(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<AdminDataId, ParseError> {
        let mut admin_data = AdminData::new();
        self.read_ar_object(element, admin_data.base_mut());

        if let Some(language) = get_child_element_string(element, "LANGUAGE") {
            admin_data.set_language(language);
        }

        // py getMultiLanguagePlainText — USED-LANGUAGES is a single
        // MultiLanguagePlainText collecting the L-10 entries. (py's
        // readARObject on the USED-LANGUAGES element is dropped in P0:
        // MultiLanguagePlainText has no ARObject base here.)
        if let Some(used_languages_node) = find(element, "USED-LANGUAGES") {
            let mut paragraph = MultiLanguagePlainText::new();
            for l10_node in find_all(used_languages_node, "L-10") {
                let mut l10 = LPlainText::new();
                if let Some(l) = l10_node.attrs.get("L") {
                    l10.set_l(l.as_str());
                }
                if let Some(xml_space) = self.read_xml_space(l10_node)? {
                    l10.set_xml_space(xml_space);
                }
                // py readLanguageSpecific: setValue(element.text) — the text
                // itself; the DOM already applied the whitespace rule.
                if let Some(value) = &l10_node.text {
                    l10.set_value(value.as_str());
                }
                let l10_id = document.l_plain_texts.insert(l10);
                paragraph.push_l10(l10_id);
            }
            let paragraph_id = document.multi_language_plain_texts.insert(paragraph);
            admin_data.set_used_languages(paragraph_id);
        }

        // py readAdminDataSdgs
        if let Some(sdgs_node) = find(element, "SDGS") {
            for child in find_all(sdgs_node, "*") {
                if child.name == "SDG" {
                    let sdg_id = self.read_sdg(child, document)?;
                    admin_data.push_sdg(sdg_id);
                } else {
                    self.not_implemented(format!("Unsupported SDG <{}>", child.name))?;
                }
            }
        }

        // py readAdminDataDocRevisions — DocRevision is a P0 placeholder;
        // flag DOC-REVISIONS content instead of silently dropping it.
        if let Some(doc_revisions_node) = find(element, "DOC-REVISIONS") {
            if !find_all(doc_revisions_node, "*").is_empty() {
                self.not_implemented("DOC-REVISIONS are not supported in P0".to_string())?;
            }
        }

        Ok(document.admin_datas.insert(admin_data))
    }

    /// py `getSdg` — reads one `SDG` element (recursively) and returns its id.
    fn read_sdg(&mut self, element: &Node, document: &mut Document) -> Result<SdgId, ParseError> {
        let mut sdg = Sdg::new();
        self.read_ar_object(element, sdg.base_mut());
        if let Some(gid) = element.attrs.get("GID") {
            sdg.set_gid(gid.as_str());
        }

        // py readSdgCaption — SDG-CAPTION with a required SHORT-NAME.
        if let Some(caption_node) = find(element, "SDG-CAPTION") {
            let mut caption = SdgCaption::new();
            self.read_ar_object(caption_node, caption.base_mut().base_mut());
            let caption_short_name = get_short_name(caption_node)?;
            caption.set_short_name(caption_short_name);
            if find(caption_node, "DESC").is_some() {
                self.not_implemented("SDG-CAPTION/DESC is not supported in P0".to_string())?;
            }
            let caption_id = document.sdg_captions.insert(caption);
            sdg.set_sdg_caption(caption_id);
        }

        // py readSd / readSdf / nested getSdg, collected into an SdgContents
        // that is attached only when non-empty.
        let mut contents = SdgContents::new();

        for sd_node in find_all(element, "SD") {
            let mut sd = Sd::new();
            self.read_ar_object(sd_node, sd.base_mut());
            if let Some(gid) = sd_node.attrs.get("GID") {
                sd.set_gid(gid.as_str());
            }
            if let Some(xml_space) = self.read_xml_space(sd_node)? {
                sd.set_xml_space(xml_space);
            }
            if let Some(value) = &sd_node.text {
                sd.set_value(value.as_str());
            }
            let sd_id = document.sds.insert(sd);
            contents.push_sd(sd_id);
        }

        for sdf_node in find_all(element, "SDF") {
            let mut sdf = Sdf::new();
            self.read_ar_object(sdf_node, sdf.base_mut());
            if let Some(gid) = sdf_node.attrs.get("GID") {
                sdf.set_gid(gid.as_str());
            }
            if let Some(value) = &sdf_node.text {
                sdf.set_value(value.as_str());
            }
            let sdf_id = document.sdfs.insert(sdf);
            contents.push_sdf(sdf_id);
        }

        for sdg_node in find_all(element, "SDG") {
            let nested_id = self.read_sdg(sdg_node, document)?;
            contents.push_sdg(nested_id);
        }

        if !contents.is_empty() {
            let contents_id = document.sdg_contents.insert(contents);
            sdg.set_sdg_contents_type(contents_id);
        }

        Ok(document.sdgs.insert(sdg))
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
        let uuid = element.attrs.get("UUID").cloned();
        let category = get_child_element_string(element, "CATEGORY").map(str::to_string);

        if let Some(package) = document.ar_packages.get_mut(id) {
            if let Some(admin_data) = admin_data {
                package.set_admin_data(admin_data);
            }
            if let Some(uuid) = uuid {
                package.set_uuid(uuid);
            }
            if let Some(category) = category {
                package.set_category(category);
            }
        }

        // py readARPackageElements / readReferenceBases — no element kinds are
        // dispatched in P0; flag unexpected content instead of dropping it.
        if let Some(elements_node) = find(element, "ELEMENTS") {
            if !find_all(elements_node, "*").is_empty() {
                self.not_implemented("AR-PACKAGE/ELEMENTS are not supported in P0".to_string())?;
            }
        }
        if let Some(reference_bases_node) = find(element, "REFERENCE-BASES") {
            if !find_all(reference_bases_node, "*").is_empty() {
                self.not_implemented("REFERENCE-BASES are not supported in P0".to_string())?;
            }
        }

        self.read_ar_packages(element, Some(id), document)?;
        Ok(id)
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
        ARXMLParser::new(default_options())
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
        let result = ARXMLParser::new(default_options()).load_from_reader(
            Reader::from_str("<?xml version=\"1.0\"?><WRONG/>"),
            &mut document,
        );
        assert!(matches!(result, Err(ParseError::UnexpectedRoot(_))));
    }

    #[test]
    fn admin_data_round_trips_through_the_model() {
        let document = parse_sample();

        let admin_data = document.get_admin_data().unwrap();

        let sdg = document.get_sdg(admin_data.get_sdgs()[0]).unwrap();
        assert_eq!(sdg.get_gid(), Some("demo"));

        let contents = document
            .get_sdg_contents(sdg.get_sdg_contents_type().unwrap())
            .unwrap();
        let sd = document.get_sd(contents.get_sd()[0]).unwrap();
        assert_eq!(sd.get_gid(), Some("purpose"));
        assert_eq!(sd.get_xml_space(), Some(XmlSpace::Preserve));
        assert_eq!(sd.get_value(), Some("special   data"));

        let mlpt = document
            .get_multi_language_plain_text(admin_data.get_used_languages().unwrap())
            .unwrap();
        let l10 = document.get_l_plain_text(mlpt.get_l10s()[0]).unwrap();
        assert_eq!(l10.get_l(), Some("EN"));
        assert_eq!(l10.get_value(), Some("English"));
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
