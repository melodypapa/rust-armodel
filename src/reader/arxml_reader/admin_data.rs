//! ADMIN-DATA / SDG readers. Part of the arxml_reader domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

impl ARXMLReader {
    /// py `getAdminData` — returns the allocated `AdminData` id.
    pub(super) fn read_admin_data(
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
            // py chain SdgCaption -> MultilanguageReferrable -> Referrable -> ARObject:
            // the generated model keeps every link, so the ARObject base is three hops out
            self.read_ar_object(caption_node, caption.base_mut().base_mut().base_mut());
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

        // py SdgContents has no is_empty; contents exist iff any of SD/SDF/nested SDG was read
        if !(contents.get_sd().is_empty()
            && contents.get_sdf().is_empty()
            && contents.get_sdg().is_empty())
        {
            let contents_id = document.sdg_contents.insert(contents);
            sdg.set_sdg_contents_type(contents_id);
        }

        Ok(document.sdgs.insert(sdg))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::m2::msr::documentation::text_model::language_data_model::XmlSpace;

    const ADMIN_DATA_SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
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
</AUTOSAR>"#;

    #[test]
    fn admin_data_round_trips_through_the_model() {
        let mut document = Document::new();
        ARXMLReader::new(default_options())
            .load_from_reader(Reader::from_str(ADMIN_DATA_SAMPLE), &mut document)
            .unwrap();

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
}
