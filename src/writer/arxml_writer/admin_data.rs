//! ADMIN-DATA / SDG emitters. Part of the arxml_writer domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

use crate::m2::msr::asam_hdo::admin_data::AdminData;
use crate::m2::msr::asam_hdo::special_data::{Sd, Sdg};

impl ARXMLWriter {
    /// py `setAdminData`
    pub(super) fn write_admin_data<W: Write>(
        &self,
        writer: &mut Writer<W>,
        admin_data: &AdminData,
        document: &Document,
    ) -> Result<(), WriteError> {
        let mut element = BytesStart::new("ADMIN-DATA");
        self.write_ar_object_attributes(&mut element, admin_data.base());
        writer.write_event(Event::Start(element))?;

        if let Some(language) = admin_data.get_language() {
            let language_element = BytesStart::new("LANGUAGE");
            write_text_element(writer, "LANGUAGE", language_element, Some(language))?;
        }

        // py setMultiLanguagePlainText
        if let Some(paragraph) = admin_data
            .get_used_languages()
            .and_then(|id| document.multi_language_plain_texts.get(id))
        {
            writer.write_event(Event::Start(BytesStart::new("USED-LANGUAGES")))?;
            for l10_id in paragraph.get_l10s() {
                if let Some(l10) = document.l_plain_texts.get(*l10_id) {
                    let mut l10_element = BytesStart::new("L-10");
                    if let Some(l) = l10.get_l() {
                        l10_element.push_attribute(("L", l));
                    }
                    if let Some(xml_space) = l10.get_xml_space() {
                        let xml_space = xml_space.to_string();
                        l10_element.push_attribute(("xml:space", xml_space.as_str()));
                    }
                    write_text_element(writer, "L-10", l10_element, l10.get_value())?;
                }
            }
            writer.write_event(Event::End(BytesEnd::new("USED-LANGUAGES")))?;
        }

        // py writeAdminDataSdgs
        let sdgs = admin_data.get_sdgs();
        if !sdgs.is_empty() {
            writer.write_event(Event::Start(BytesStart::new("SDGS")))?;
            for sdg_id in sdgs {
                if let Some(sdg) = document.sdgs.get(*sdg_id) {
                    self.write_sdg(writer, sdg, document)?;
                }
            }
            writer.write_event(Event::End(BytesEnd::new("SDGS")))?;
        }

        // py writeAdminDataDocRevisions — DocRevision is a P0 placeholder and
        // `doc_revisions` is always empty here; emitted from P1.

        writer.write_event(Event::End(BytesEnd::new("ADMIN-DATA")))?;
        Ok(())
    }

    /// py `setSdg`
    fn write_sdg<W: Write>(
        &self,
        writer: &mut Writer<W>,
        sdg: &Sdg,
        document: &Document,
    ) -> Result<(), WriteError> {
        let mut element = BytesStart::new("SDG");
        self.write_ar_object_attributes(&mut element, sdg.base());
        if let Some(gid) = sdg.get_gid() {
            element.push_attribute(("GID", gid));
        }
        writer.write_event(Event::Start(element))?;

        // py writeSdgCaption
        if let Some(caption) = sdg
            .get_sdg_caption()
            .and_then(|id| document.sdg_captions.get(id))
        {
            writer.write_event(Event::Start(BytesStart::new("SDG-CAPTION")))?;
            if let Some(short_name) = caption.get_short_name() {
                let short_name_element = BytesStart::new("SHORT-NAME");
                write_text_element(writer, "SHORT-NAME", short_name_element, Some(short_name))?;
            }
            // caption.get_desc() → MultiLanguageOverviewParagraph is a P0
            // placeholder; nothing to emit yet.
            writer.write_event(Event::End(BytesEnd::new("SDG-CAPTION")))?;
        }

        // py writeSds / writeSdfs / nested setSdg
        if let Some(contents) = sdg
            .get_sdg_contents_type()
            .and_then(|id| document.sdg_contents.get(id))
        {
            for sd_id in contents.get_sd() {
                if let Some(sd) = document.sds.get(*sd_id) {
                    self.write_sd(writer, sd)?;
                }
            }
            for sdf_id in contents.get_sdf() {
                if let Some(sdf) = document.sdfs.get(*sdf_id) {
                    let mut sdf_element = BytesStart::new("SDF");
                    self.write_ar_object_attributes(&mut sdf_element, sdf.base());
                    if let Some(gid) = sdf.get_gid() {
                        sdf_element.push_attribute(("GID", gid));
                    }
                    write_text_element(writer, "SDF", sdf_element, sdf.get_value())?;
                }
            }
            for nested_id in contents.get_sdg() {
                if let Some(nested) = document.sdgs.get(*nested_id) {
                    self.write_sdg(writer, nested, document)?;
                }
            }
        }

        writer.write_event(Event::End(BytesEnd::new("SDG")))?;
        Ok(())
    }

    /// py `writeSds` per-SD body
    fn write_sd<W: Write>(&self, writer: &mut Writer<W>, sd: &Sd) -> Result<(), WriteError> {
        let mut element = BytesStart::new("SD");
        self.write_ar_object_attributes(&mut element, sd.base());
        if let Some(gid) = sd.get_gid() {
            element.push_attribute(("GID", gid));
        }
        if let Some(xml_space) = sd.get_xml_space() {
            let xml_space = xml_space.to_string();
            element.push_attribute(("xml:space", xml_space.as_str()));
        }
        write_text_element(writer, "SD", element, sd.get_value())?;
        Ok(())
    }
}
