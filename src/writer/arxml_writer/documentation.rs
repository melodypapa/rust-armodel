//! Multilanguage / documentation-block emitters. Part of the arxml_writer
//! domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

use crate::m2::msr::documentation::text_model::block_elements::DocumentationBlock;
use crate::m2::msr::documentation::text_model::multilanguage_data::{
    MultiLanguageOverviewParagraph, MultilanguageLongName,
};

impl ARXMLWriter {
    /// py `setMultiLongName` / `setLLongName` — L attr, text, then S/T
    /// (writeMixedContentForLongName runs after the text is set).
    pub(super) fn set_multi_long_name<W: Write>(
        &self,
        writer: &mut Writer<W>,
        long_name: &MultilanguageLongName,
        document: &Document,
    ) -> Result<(), WriteError> {
        let mut element = BytesStart::new("LONG-NAME");
        self.write_ar_object_attributes(&mut element, long_name.base());
        if long_name.get_l4().is_empty() {
            write_text_element(writer, "LONG-NAME", element, None)?;
            return Ok(());
        }
        writer.write_event(Event::Start(element))?;
        for l4_id in long_name.get_l4() {
            if let Some(l4) = document.l_long_names.get(*l4_id) {
                let mut l4_element = BytesStart::new("L-4");
                if let Some(l) = l4.get_l() {
                    l4_element.push_attribute(("L", l));
                }
                if let Some(sup) = l4.get_sup() {
                    l4_element.push_attribute(("SUP", sup));
                }
                if let Some(sub) = l4.get_sub() {
                    l4_element.push_attribute(("SUB", sub));
                }
                self.write_ar_object_attributes(&mut l4_element, l4.base().base());
                write_text_element(writer, "L-4", l4_element, l4.get_value())?;
            }
        }
        writer.write_event(Event::End(BytesEnd::new("LONG-NAME")))?;
        Ok(())
    }

    /// py `setMultiLanguageOverviewParagraph` / `setLOverviewParagraph` —
    /// S/T then L on each L-2 (setLanguageSpecific writes ARObject first).
    pub(super) fn set_multi_language_overview_paragraph<W: Write>(
        &self,
        writer: &mut Writer<W>,
        paragraph: &MultiLanguageOverviewParagraph,
        document: &Document,
    ) -> Result<(), WriteError> {
        let mut element = BytesStart::new("DESC");
        self.write_ar_object_attributes(&mut element, paragraph.base());
        // py/minidom renders a childless wrapper expanded-inline
        // (`<DESC></DESC>`) — which is exactly write_text_element's
        // empty-without-attributes form.
        if paragraph.get_l2().is_empty() {
            write_text_element(writer, "DESC", element, None)?;
            return Ok(());
        }
        writer.write_event(Event::Start(element))?;
        for l2_id in paragraph.get_l2() {
            if let Some(l2) = document.l_overview_paragraphs.get(*l2_id) {
                let mut l2_element = BytesStart::new("L-2");
                self.write_ar_object_attributes(&mut l2_element, l2.base().base());
                if let Some(l) = l2.get_l() {
                    l2_element.push_attribute(("L", l));
                }
                if let Some(blueprint) = l2.get_blueprint_value() {
                    l2_element.push_attribute(("BLUEPRINT-VALUE", blueprint));
                }
                write_text_element(writer, "L-2", l2_element, l2.get_value())?;
            }
        }
        writer.write_event(Event::End(BytesEnd::new("DESC")))?;
        Ok(())
    }

    /// py `writeDocumentationBlock` / `writeDocumentationBlockContent` —
    /// P paragraphs and LISTs; the other block kinds do not occur in the
    /// pinned fixtures (tracked on the port checklist).
    pub(super) fn write_documentation_block<W: Write>(
        &self,
        writer: &mut Writer<W>,
        key: &str,
        block: &DocumentationBlock,
        document: &Document,
    ) -> Result<(), WriteError> {
        writer.write_event(Event::Start(BytesStart::new(key)))?;
        self.write_documentation_block_content(writer, block, document)?;
        writer.write_event(Event::End(BytesEnd::new(key)))?;
        Ok(())
    }

    fn write_documentation_block_content<W: Write>(
        &self,
        writer: &mut Writer<W>,
        block: &DocumentationBlock,
        document: &Document,
    ) -> Result<(), WriteError> {
        // py setMultiLanguageParagraphs(element, "P", …)
        for paragraph_id in block.get_ps() {
            if let Some(paragraph) = document.multi_language_paragraphs.get(*paragraph_id) {
                let mut p_element = BytesStart::new("P");
                if let Some(help) = paragraph.get_help_entry() {
                    p_element.push_attribute(("HELP-ENTRY", help));
                }
                self.write_paginateable_attrs(&mut p_element, paragraph.base());
                writer.write_event(Event::Start(p_element))?;
                // py writeLParagraphs — S/T, text, then L (attributes in
                // py's dict-insertion order; the text is the element text).
                for l1_id in paragraph.get_l1() {
                    if let Some(l1) = document.l_paragraphs.get(*l1_id) {
                        let mut l1_element = BytesStart::new("L-1");
                        self.write_ar_object_attributes(&mut l1_element, l1.base().base());
                        if let Some(l) = l1.get_l() {
                            l1_element.push_attribute(("L", l));
                        }
                        write_text_element(writer, "L-1", l1_element, l1.get_value())?;
                    }
                }
                writer.write_event(Event::End(BytesEnd::new("P")))?;
            }
        }
        // py setListElement(element, "LIST", …)
        for list_id in block.get_lists() {
            if let Some(list) = document.ar_lists.get(*list_id) {
                let mut list_element = BytesStart::new("LIST");
                self.write_paginateable_attrs(&mut list_element, list.base());
                if let Some(list_type) = list.get_type() {
                    // py: type.getValue().upper()
                    let value = list_type.as_str().to_uppercase();
                    list_element.push_attribute(("TYPE", value.as_str()));
                }
                writer.write_event(Event::Start(list_element))?;
                for item_id in list.get_items() {
                    if let Some(item) = document.items.get(*item_id) {
                        let mut item_element = BytesStart::new("ITEM");
                        self.write_paginateable_attrs(&mut item_element, item.base());
                        writer.write_event(Event::Start(item_element))?;
                        if let Some(contents) = item
                            .get_item_contents()
                            .and_then(|id| document.documentation_blocks.get(id))
                        {
                            self.write_documentation_block_content(writer, contents, document)?;
                        }
                        writer.write_event(Event::End(BytesEnd::new("ITEM")))?;
                    }
                }
                writer.write_event(Event::End(BytesEnd::new("LIST")))?;
            }
        }
        Ok(())
    }

    /// py `writePaginateable` — BREAK / KEEP-WITH-PREVIOUS attributes.
    fn write_paginateable_attrs(
        &self,
        element: &mut BytesStart<'_>,
        paginateable: &crate::m2::msr::documentation::block_elements::pagination_and_view::Paginateable,
    ) {
        if let Some(chapter_break) = paginateable.get_chapter_break() {
            element.push_attribute(("BREAK", chapter_break.as_str()));
        }
        if let Some(keep) = paginateable.get_keep_with_previous() {
            element.push_attribute(("KEEP-WITH-PREVIOUS", keep.as_str()));
        }
    }
}
