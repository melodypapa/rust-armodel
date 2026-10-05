//! Multilanguage long-name / overview-paragraph / documentation-block readers
//! (py readMultilanguageReferrable chain). Part of the arxml_reader domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

impl ARXMLReader {
    /// py `getMultilanguageLongName` / `readLLongName` / `readMixedContentForLongName`.
    pub(super) fn get_multilanguage_long_name(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<Option<MultilanguageLongNameId>, ParseError> {
        let Some(long_name_node) = find(element, "LONG-NAME") else {
            return Ok(None);
        };
        let mut long_name = MultilanguageLongName::new();
        self.read_ar_object(long_name_node, long_name.base_mut());
        for l4_node in find_all(long_name_node, "L-4") {
            let mut l4 = LLongName::new();
            // py readMixedContentForLongName reads the L-4's ARObject (S/T);
            // LLongName forwards checksum/timestamp from its mixed-content base.
            if let Some(checksum) = l4_node.attrs.get("S") {
                l4.set_checksum(checksum.as_str());
            }
            if let Some(timestamp) = l4_node.attrs.get("T") {
                l4.set_timestamp(timestamp.as_str());
            }
            if let Some(text) = &l4_node.text {
                l4.set_value(text.as_str());
            }
            if let Some(l) = l4_node.attrs.get("L") {
                l4.set_l(l.as_str());
            }
            if let Some(sup) = l4_node.attrs.get("SUP") {
                l4.set_sup(sup.as_str());
            }
            if let Some(sub) = l4_node.attrs.get("SUB") {
                l4.set_sub(sub.as_str());
            }
            // py iterates inline children and only acts on E/IE/TT; neither
            // occurs in any pinned fixture, so they stay on the warning path.
            for inline in &l4_node.children {
                if matches!(inline.name.as_str(), "E" | "IE" | "TT") {
                    let message = format!("Unsupported inline element <{}> in L-4", inline.name);
                    self.not_implemented(message)?;
                }
            }
            let l4_id = document.l_long_names.insert(l4);
            long_name.push_l4(l4_id);
        }
        Ok(Some(document.multilanguage_long_names.insert(long_name)))
    }

    /// py `getMultiLanguageOverviewParagraph` / `readLOverviewParagraph`.
    pub(super) fn get_multi_language_overview_paragraph(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<Option<MultiLanguageOverviewParagraphId>, ParseError> {
        let Some(desc_node) = find(element, "DESC") else {
            return Ok(None);
        };
        let mut paragraph = MultiLanguageOverviewParagraph::new();
        self.read_ar_object(desc_node, paragraph.base_mut());
        for l2_node in find_all(desc_node, "L-2") {
            let mut l2 = LOverviewParagraph::new();
            if let Some(checksum) = l2_node.attrs.get("S") {
                l2.set_checksum(checksum.as_str());
            }
            if let Some(timestamp) = l2_node.attrs.get("T") {
                l2.set_timestamp(timestamp.as_str());
            }
            if let Some(text) = &l2_node.text {
                l2.set_value(text.as_str());
            }
            if let Some(l) = l2_node.attrs.get("L") {
                l2.set_l(l.as_str());
            }
            if let Some(blueprint) = l2_node.attrs.get("BLUEPRINT-VALUE") {
                l2.set_blueprint_value(blueprint.as_str());
            }
            let l2_id = document.l_overview_paragraphs.insert(l2);
            paragraph.push_l2(l2_id);
        }
        Ok(Some(
            document
                .multi_language_overview_paragraphs
                .insert(paragraph),
        ))
    }

    /// py `getDocumentationBlock` — the INTRODUCTION wrapper.
    pub(super) fn get_documentation_block(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<Option<DocumentationBlockId>, ParseError> {
        let Some(block_node) = find(element, "INTRODUCTION") else {
            return Ok(None);
        };
        let block = self.read_documentation_block(block_node, document)?;
        Ok(Some(block))
    }

    /// py `readDocumentationBlock` — P paragraphs and LIST items (+ ITEM
    /// recursion). The other block kinds (DEF-LIST, FORMULA, FIGURE, …) do
    /// not occur in any pinned fixture; py only reads them when present, so
    /// their absence here is behavior-identical (tracked on the checklist).
    pub(super) fn read_documentation_block(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<DocumentationBlockId, ParseError> {
        let mut block = DocumentationBlock::new();
        self.read_ar_object(element, block.base_mut());
        // py getMultiLanguageParagraphs(element, "P")
        for p_node in find_all(element, "P") {
            let mut paragraph = MultiLanguageParagraph::new();
            self.read_paginateable(p_node, paragraph.base_mut())?;
            if let Some(help) = p_node.attrs.get("HELP-ENTRY") {
                paragraph.set_help_entry(help.as_str());
            }
            // py getLParagraphs(child_element, "L-1") — py reads inline
            // mixed content via readMixedContentForParagraph; no pinned
            // fixture carries it, so only the text/L form is read here.
            for l1_node in find_all(p_node, "L-1") {
                let mut l1 = LParagraph::new();
                if let Some(checksum) = l1_node.attrs.get("S") {
                    l1.set_checksum(checksum.as_str());
                }
                if let Some(timestamp) = l1_node.attrs.get("T") {
                    l1.set_timestamp(timestamp.as_str());
                }
                if let Some(text) = &l1_node.text {
                    l1.set_value(text.as_str());
                }
                if let Some(l) = l1_node.attrs.get("L") {
                    l1.set_l(l.as_str());
                }
                let l1_id = document.l_paragraphs.insert(l1);
                paragraph.push_l1(l1_id);
            }
            let paragraph_id = document.multi_language_paragraphs.insert(paragraph);
            block.push_p(paragraph_id);
        }
        // py getListElements(element, "LIST")
        for list_node in find_all(element, "LIST") {
            let mut list = ARList::new();
            self.read_paginateable(list_node, list.base_mut())?;
            if let Some(type_attr) = list_node.attrs.get("TYPE") {
                // py: ListEnum().setValue(value.lower()); the writer uppercases
                match ListEnum::try_from(type_attr.to_lowercase().as_str()) {
                    Ok(value) => {
                        list.set_type(value);
                    }
                    Err(_) => {
                        let message = format!("Unsupported LIST TYPE <{type_attr}>");
                        self.not_implemented(message)?;
                    }
                }
            }
            for item_node in find_all(list_node, "ITEM") {
                let mut item = Item::new();
                self.read_paginateable(item_node, item.base_mut())?;
                let contents = self.read_documentation_block(item_node, document)?;
                item.set_item_contents(contents);
                let item_id = document.items.insert(item);
                list.push_item(item_id);
            }
            let list_id = document.ar_lists.insert(list);
            block.push_list(list_id);
        }
        Ok(document.documentation_blocks.insert(block))
    }

    /// py `readPaginateable` — BREAK / KEEP-WITH-PREVIOUS attributes.
    fn read_paginateable(
        &mut self,
        element: &Node,
        paginateable: &mut Paginateable,
    ) -> Result<(), ParseError> {
        if let Some(break_attr) = element.attrs.get("BREAK") {
            match ChapterEnumBreak::try_from(break_attr.as_str()) {
                Ok(value) => {
                    paginateable.set_chapter_break(value);
                }
                Err(_) => {
                    let message = format!("Unsupported BREAK <{break_attr}>");
                    self.not_implemented(message)?;
                }
            }
        }
        if let Some(keep) = element.attrs.get("KEEP-WITH-PREVIOUS") {
            match KeepWithPreviousEnum::try_from(keep.as_str()) {
                Ok(value) => {
                    paginateable.set_keep_with_previous(value);
                }
                Err(_) => {
                    let message = format!("Unsupported KEEP-WITH-PREVIOUS <{keep}>");
                    self.not_implemented(message)?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const IDENTIFIABLE_SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<AUTOSAR xmlns="http://autosar.org/schema/r4.0" xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00052.xsd">
  <AR-PACKAGES>
    <AR-PACKAGE>
      <SHORT-NAME>Pkg</SHORT-NAME>
      <ELEMENTS>
        <SW-BASE-TYPE>
          <SHORT-NAME>T</SHORT-NAME>
          <LONG-NAME>
            <L-4 L="DE">Ein Typ</L-4>
          </LONG-NAME>
          <DESC>
            <L-2 L="EN" xml:space="preserve">A  type</L-2>
          </DESC>
          <INTRODUCTION>
            <P>
              <L-1 L="EN">Intro text</L-1>
            </P>
            <LIST TYPE="LIST">
              <ITEM>
                <P><L-1>Nested</L-1></P>
              </ITEM>
            </LIST>
          </INTRODUCTION>
        </SW-BASE-TYPE>
      </ELEMENTS>
    </AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#;

    #[test]
    fn identifiable_payload_round_trips_text_nodes() {
        let mut document = Document::new();
        ARXMLReader::new(default_options())
            .load_from_reader(Reader::from_str(IDENTIFIABLE_SAMPLE), &mut document)
            .unwrap();

        let pkg = document
            .get_ar_package(document.get_ar_packages()[0])
            .unwrap();
        let ElementRef::SwBaseType(id) = pkg.get_elements()[0] else {
            panic!("expected SwBaseType element");
        };
        let base_type = document.get_sw_base_type(id).unwrap();
        let long_name = document
            .get_multilanguage_long_name(base_type.get_long_name().unwrap())
            .unwrap();
        let l4 = document.get_l_long_name(long_name.get_l4()[0]).unwrap();
        assert_eq!(l4.get_l(), Some("DE"));
        assert_eq!(l4.get_value(), Some("Ein Typ"));

        let desc = document
            .get_multi_language_overview_paragraph(base_type.get_desc().unwrap())
            .unwrap();
        let l2 = document.get_l_overview_paragraph(desc.get_l2()[0]).unwrap();
        assert_eq!(l2.get_value(), Some("A  type"));

        let introduction = document
            .get_documentation_block(base_type.get_introduction().unwrap())
            .unwrap();
        assert_eq!(introduction.get_ps().len(), 1);
        assert_eq!(introduction.get_lists().len(), 1);
    }
}
