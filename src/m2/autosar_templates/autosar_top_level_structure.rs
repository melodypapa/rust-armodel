//! spec: M2::AUTOSARTemplates::AutosarTopLevelStructure
//!
//! Home of the spec class `AUTOSAR`. The Rust root type is `Document`
//! (P0 design §4): it owns one arena per concrete class, and all
//! cross-type mutation goes through it (`docs/code_guide.md` §4).

use id_arena::{Arena, Id};

use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_object::{
    ARObject, ElementRef,
};
use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_package::{
    ARPackage, ReferenceBase,
};
use crate::m2::msr::asam_hdo::admin_data::{AdminData, DocRevision};
use crate::m2::msr::asam_hdo::special_data::{Sd, Sdf, Sdg, SdgCaption, SdgContents};
use crate::m2::msr::documentation::text_model::language_data_model::LPlainText;
use crate::m2::msr::documentation::text_model::multilanguage_data::{
    MultiLanguageOverviewParagraph, MultiLanguagePlainText,
};

/// Root type, in place of py-armodel's `AUTOSAR`.
///
/// The arena fields are `pub(crate)`: the parser and writer resolve ids
/// through them inside the crate; code outside the crate mutates only via the
/// factory methods and reads via the accessors/resolvers.
#[derive(Debug, Default)]
pub struct Document {
    // — arenas (one per concrete class) —
    pub(crate) ar_packages: Arena<ARPackage>,
    /// P0 placeholder arena: nothing allocates `ReferenceBase`s until P1.
    #[allow(dead_code)]
    pub(crate) reference_bases: Arena<ReferenceBase>,
    pub(crate) admin_datas: Arena<AdminData>,
    /// P0 placeholder arena: nothing allocates `DocRevision`s until P1.
    #[allow(dead_code)]
    pub(crate) doc_revisions: Arena<DocRevision>,
    pub(crate) multi_language_plain_texts: Arena<MultiLanguagePlainText>,
    /// P0 placeholder arena: nothing allocates overview paragraphs until P1.
    #[allow(dead_code)]
    pub(crate) multi_language_overview_paragraphs: Arena<MultiLanguageOverviewParagraph>,
    pub(crate) l_plain_texts: Arena<LPlainText>,
    pub(crate) sds: Arena<Sd>,
    pub(crate) sdfs: Arena<Sdf>,
    pub(crate) sdg_captions: Arena<SdgCaption>,
    pub(crate) sdg_contents: Arena<SdgContents>,
    pub(crate) sdgs: Arena<Sdg>,

    // — root fields (spec class `AUTOSAR`) —
    admin_data: Option<Id<AdminData>>,
    root_ar_packages: Vec<Id<ARPackage>>,
    schema_location: String,
    ar_release: String,
}

impl Document {
    pub fn new() -> Self {
        Self::default()
    }

    // — root field accessors (py getAdminData/setAdminData/getARPackages) —

    pub fn get_admin_data(&self) -> Option<&AdminData> {
        self.admin_data.and_then(|id| self.admin_datas.get(id))
    }

    pub fn set_admin_data(&mut self, admin_data: Id<AdminData>) -> &mut Self {
        self.admin_data = Some(admin_data);
        self
    }

    pub fn get_ar_packages(&self) -> &[Id<ARPackage>] {
        &self.root_ar_packages
    }

    pub fn get_schema_location(&self) -> &str {
        &self.schema_location
    }

    pub fn set_schema_location(&mut self, value: impl Into<String>) -> &mut Self {
        self.schema_location = value.into();
        self
    }

    pub fn get_ar_release(&self) -> &str {
        &self.ar_release
    }

    pub fn set_ar_release(&mut self, value: impl Into<String>) -> &mut Self {
        self.ar_release = value.into();
        self
    }

    // — factories (py createARPackage / addElement) —

    /// Creates an `ARPackage` in the arena and links it: when `parent` is
    /// `Some`, the child's `parent` handle is set and the id is pushed into
    /// the parent's package list; when `None`, it becomes a root package.
    pub fn add_ar_package(
        &mut self,
        parent: Option<Id<ARPackage>>,
        short_name: &str,
    ) -> Id<ARPackage> {
        let mut package = ARPackage::new();
        package.set_short_name(short_name);
        if let Some(parent_id) = parent {
            package.set_parent(Some(ElementRef::ARPackage(parent_id)));
        }
        let id = self.ar_packages.alloc(package);
        match parent {
            Some(parent_id) => {
                if let Some(parent) = self.ar_packages.get_mut(parent_id) {
                    parent.push_ar_package(id);
                }
            }
            None => self.root_ar_packages.push(id),
        }
        id
    }

    /// py `addElement` — appends a heterogeneous element to a package.
    pub fn add_element(&mut self, package: Id<ARPackage>, element: ElementRef) {
        if let Some(package) = self.ar_packages.get_mut(package) {
            package.push_element(element);
        }
    }

    // — id resolvers (read access from outside the crate) —

    pub fn get_ar_package(&self, id: Id<ARPackage>) -> Option<&ARPackage> {
        self.ar_packages.get(id)
    }

    pub fn get_sdg(&self, id: Id<Sdg>) -> Option<&Sdg> {
        self.sdgs.get(id)
    }

    pub fn get_sdg_contents(&self, id: Id<SdgContents>) -> Option<&SdgContents> {
        self.sdg_contents.get(id)
    }

    pub fn get_sd(&self, id: Id<Sd>) -> Option<&Sd> {
        self.sds.get(id)
    }

    pub fn get_multi_language_plain_text(
        &self,
        id: Id<MultiLanguagePlainText>,
    ) -> Option<&MultiLanguagePlainText> {
        self.multi_language_plain_texts.get(id)
    }

    pub fn get_l_plain_text(&self, id: Id<LPlainText>) -> Option<&LPlainText> {
        self.l_plain_texts.get(id)
    }

    // — structural equality (P0 design §7) —

    /// Structural equality: walk both models, resolving `Id<T>` links within
    /// each `Document`. Mirrors py's `assert_models_equal`:
    /// exact type + field-by-field comparison, ignoring `parent`.
    pub fn assert_structurally_equal(&self, other: &Document) -> Result<(), String> {
        if self.schema_location != other.schema_location {
            return Err(format!(
                "schema_location mismatch: `{}` vs `{}`",
                self.schema_location, other.schema_location
            ));
        }
        if self.ar_release != other.ar_release {
            return Err(format!(
                "ar_release mismatch: `{}` vs `{}`",
                self.ar_release, other.ar_release
            ));
        }

        match (self.get_admin_data(), other.get_admin_data()) {
            (None, None) => {}
            (Some(admin_a), Some(admin_b)) => {
                self.compare_admin_data(other, admin_a, admin_b, "ADMIN-DATA")?;
            }
            _ => {
                return Err(
                    "admin_data mismatch: ADMIN-DATA present in one document only".to_string(),
                )
            }
        }

        if self.root_ar_packages.len() != other.root_ar_packages.len() {
            return Err(format!(
                "root_ar_packages length mismatch: {} vs {}",
                self.root_ar_packages.len(),
                other.root_ar_packages.len()
            ));
        }
        for (index, (a, b)) in self
            .root_ar_packages
            .iter()
            .zip(other.root_ar_packages.iter())
            .enumerate()
        {
            let package_a = self
                .ar_packages
                .get(*a)
                .ok_or_else(|| format!("root_ar_packages[{index}]: id not found in own arena"))?;
            let package_b = other
                .ar_packages
                .get(*b)
                .ok_or_else(|| format!("root_ar_packages[{index}]: id not found in other arena"))?;
            self.compare_ar_package(other, package_a, package_b, &format!("AR-PACKAGE[{index}]"))?;
        }
        Ok(())
    }

    fn compare_ar_object(a: &ARObject, b: &ARObject, path: &str) -> Result<(), String> {
        if a.get_checksum() != b.get_checksum() {
            return Err(format!("{path}: checksum mismatch"));
        }
        if a.get_timestamp() != b.get_timestamp() {
            return Err(format!("{path}: timestamp mismatch"));
        }
        Ok(())
    }

    fn compare_ar_package(
        &self,
        other: &Document,
        a: &ARPackage,
        b: &ARPackage,
        path: &str,
    ) -> Result<(), String> {
        // Chain to ARObject: ARPackage → CollectableElement → Identifiable →
        // MultilanguageReferrable → Referrable → ARObject.
        Self::compare_ar_object(
            a.base().base().base().base().base(),
            b.base().base().base().base().base(),
            path,
        )?;
        if a.get_short_name() != b.get_short_name() {
            return Err(format!("{path}: SHORT-NAME mismatch"));
        }
        if a.get_uuid() != b.get_uuid() {
            return Err(format!("{path}: UUID mismatch"));
        }
        if a.get_category() != b.get_category() {
            return Err(format!("{path}: CATEGORY mismatch"));
        }
        // `parent` is ignored, mirroring py's ignored_attrs = ["parent"]

        match (a.get_admin_data(), b.get_admin_data()) {
            (None, None) => {}
            (Some(admin_a), Some(admin_b)) => {
                let admin_a = self
                    .admin_datas
                    .get(admin_a)
                    .ok_or_else(|| format!("{path}: admin_data id not found in own arena"))?;
                let admin_b = other
                    .admin_datas
                    .get(admin_b)
                    .ok_or_else(|| format!("{path}: admin_data id not found in other arena"))?;
                self.compare_admin_data(other, admin_a, admin_b, &format!("{path}.ADMIN-DATA"))?;
            }
            _ => return Err(format!("{path}: admin_data present in one package only")),
        }

        let sub_packages_a = a.get_ar_packages();
        let sub_packages_b = b.get_ar_packages();
        if sub_packages_a.len() != sub_packages_b.len() {
            return Err(format!("{path}: nested AR-PACKAGES length mismatch"));
        }
        for (index, (sub_a, sub_b)) in sub_packages_a.iter().zip(sub_packages_b.iter()).enumerate()
        {
            let package_a = self
                .ar_packages
                .get(*sub_a)
                .ok_or_else(|| format!("{path}.AR-PACKAGES[{index}]: id not found in own arena"))?;
            let package_b = other.ar_packages.get(*sub_b).ok_or_else(|| {
                format!("{path}.AR-PACKAGES[{index}]: id not found in other arena")
            })?;
            self.compare_ar_package(
                other,
                package_a,
                package_b,
                &format!("{path}.AR-PACKAGES[{index}]"),
            )?;
        }

        let elements_a = a.get_elements();
        let elements_b = b.get_elements();
        if elements_a.len() != elements_b.len() {
            return Err(format!("{path}: ELEMENTS length mismatch"));
        }
        for (index, (element_a, element_b)) in elements_a.iter().zip(elements_b.iter()).enumerate()
        {
            match (element_a, element_b) {
                (ElementRef::ARPackage(package_a), ElementRef::ARPackage(package_b)) => {
                    let package_a = self.ar_packages.get(*package_a).ok_or_else(|| {
                        format!("{path}.ELEMENTS[{index}]: id not found in own arena")
                    })?;
                    let package_b = other.ar_packages.get(*package_b).ok_or_else(|| {
                        format!("{path}.ELEMENTS[{index}]: id not found in other arena")
                    })?;
                    self.compare_ar_package(
                        other,
                        package_a,
                        package_b,
                        &format!("{path}.ELEMENTS[{index}]"),
                    )?;
                } // (P0 has the single ARPackage variant; the P1 converter's
                  // generated enum adds the kind-mismatch arm back.)
            }
        }

        // ReferenceBase is a P0 placeholder (no fields); list length is the
        // whole comparison until P1 fills it in.
        if a.get_reference_bases().len() != b.get_reference_bases().len() {
            return Err(format!("{path}: REFERENCE-BASES length mismatch"));
        }
        Ok(())
    }

    fn compare_admin_data(
        &self,
        other: &Document,
        a: &AdminData,
        b: &AdminData,
        path: &str,
    ) -> Result<(), String> {
        Self::compare_ar_object(a.base(), b.base(), path)?;
        if a.get_language() != b.get_language() {
            return Err(format!("{path}: LANGUAGE mismatch"));
        }

        match (a.get_used_languages(), b.get_used_languages()) {
            (None, None) => {}
            (Some(mlpt_a), Some(mlpt_b)) => {
                let mlpt_a = self
                    .multi_language_plain_texts
                    .get(mlpt_a)
                    .ok_or_else(|| format!("{path}: USED-LANGUAGES id not found in own arena"))?;
                let mlpt_b = other
                    .multi_language_plain_texts
                    .get(mlpt_b)
                    .ok_or_else(|| format!("{path}: USED-LANGUAGES id not found in other arena"))?;
                self.compare_multi_language_plain_text(
                    other,
                    mlpt_a,
                    mlpt_b,
                    &format!("{path}.USED-LANGUAGES"),
                )?;
            }
            _ => {
                return Err(format!(
                    "{path}: USED-LANGUAGES present in one document only"
                ))
            }
        }

        let sdgs_a = a.get_sdgs();
        let sdgs_b = b.get_sdgs();
        if sdgs_a.len() != sdgs_b.len() {
            return Err(format!("{path}: SDGS length mismatch"));
        }
        for (index, (sdg_a, sdg_b)) in sdgs_a.iter().zip(sdgs_b.iter()).enumerate() {
            let sdg_a = self
                .sdgs
                .get(*sdg_a)
                .ok_or_else(|| format!("{path}.SDGS[{index}]: id not found in own arena"))?;
            let sdg_b = other
                .sdgs
                .get(*sdg_b)
                .ok_or_else(|| format!("{path}.SDGS[{index}]: id not found in other arena"))?;
            self.compare_sdg(other, sdg_a, sdg_b, &format!("{path}.SDGS[{index}]"))?;
        }

        // DocRevision is a P0 placeholder (no fields); list length is the
        // whole comparison until P1 fills it in.
        if a.get_doc_revisions().len() != b.get_doc_revisions().len() {
            return Err(format!("{path}: DOC-REVISIONS length mismatch"));
        }
        Ok(())
    }

    fn compare_multi_language_plain_text(
        &self,
        other: &Document,
        a: &MultiLanguagePlainText,
        b: &MultiLanguagePlainText,
        path: &str,
    ) -> Result<(), String> {
        let l10s_a = a.get_l10s();
        let l10s_b = b.get_l10s();
        if l10s_a.len() != l10s_b.len() {
            return Err(format!("{path}: L-10 length mismatch"));
        }
        for (index, (l10_a, l10_b)) in l10s_a.iter().zip(l10s_b.iter()).enumerate() {
            let l10_a = self
                .l_plain_texts
                .get(*l10_a)
                .ok_or_else(|| format!("{path}.L-10[{index}]: id not found in own arena"))?;
            let l10_b = other
                .l_plain_texts
                .get(*l10_b)
                .ok_or_else(|| format!("{path}.L-10[{index}]: id not found in other arena"))?;
            let l10_path = format!("{path}.L-10[{index}]");
            if l10_a.get_l() != l10_b.get_l() {
                return Err(format!("{l10_path}: L attribute mismatch"));
            }
            if l10_a.get_xml_space() != l10_b.get_xml_space() {
                return Err(format!("{l10_path}: xml:space mismatch"));
            }
            if l10_a.get_value() != l10_b.get_value() {
                return Err(format!("{l10_path}: text mismatch"));
            }
        }
        Ok(())
    }

    fn compare_sdg(&self, other: &Document, a: &Sdg, b: &Sdg, path: &str) -> Result<(), String> {
        Self::compare_ar_object(a.base(), b.base(), path)?;
        if a.get_gid() != b.get_gid() {
            return Err(format!("{path}: GID mismatch"));
        }

        match (a.get_sdg_caption(), b.get_sdg_caption()) {
            (None, None) => {}
            (Some(caption_a), Some(caption_b)) => {
                let caption_a = self
                    .sdg_captions
                    .get(caption_a)
                    .ok_or_else(|| format!("{path}: SDG-CAPTION id not found in own arena"))?;
                let caption_b = other
                    .sdg_captions
                    .get(caption_b)
                    .ok_or_else(|| format!("{path}: SDG-CAPTION id not found in other arena"))?;
                let caption_path = format!("{path}.SDG-CAPTION");
                // SdgCaption base chain: Referrable → ARObject.
                Self::compare_ar_object(
                    caption_a.base().base(),
                    caption_b.base().base(),
                    &caption_path,
                )?;
                if caption_a.get_short_name() != caption_b.get_short_name() {
                    return Err(format!("{caption_path}: SHORT-NAME mismatch"));
                }
                // desc (MultiLanguageOverviewParagraph) is a P0 placeholder on
                // both sides; presence equality is the whole comparison.
                if caption_a.get_desc().is_some() != caption_b.get_desc().is_some() {
                    return Err(format!("{caption_path}: DESC present in one side only"));
                }
            }
            _ => return Err(format!("{path}: SDG-CAPTION present in one side only")),
        }

        match (a.get_sdg_contents_type(), b.get_sdg_contents_type()) {
            (None, None) => {}
            (Some(contents_a), Some(contents_b)) => {
                let contents_a = self
                    .sdg_contents
                    .get(contents_a)
                    .ok_or_else(|| format!("{path}: SDG contents id not found in own arena"))?;
                let contents_b = other
                    .sdg_contents
                    .get(contents_b)
                    .ok_or_else(|| format!("{path}: SDG contents id not found in other arena"))?;
                self.compare_sdg_contents(
                    other,
                    contents_a,
                    contents_b,
                    &format!("{path}.contents"),
                )?;
            }
            _ => return Err(format!("{path}: SDG contents present in one side only")),
        }
        Ok(())
    }

    fn compare_sdg_contents(
        &self,
        other: &Document,
        a: &SdgContents,
        b: &SdgContents,
        path: &str,
    ) -> Result<(), String> {
        let sds_a = a.get_sd();
        let sds_b = b.get_sd();
        if sds_a.len() != sds_b.len() {
            return Err(format!("{path}: SD length mismatch"));
        }
        for (index, (sd_a, sd_b)) in sds_a.iter().zip(sds_b.iter()).enumerate() {
            let sd_a = self
                .sds
                .get(*sd_a)
                .ok_or_else(|| format!("{path}.SD[{index}]: id not found in own arena"))?;
            let sd_b = other
                .sds
                .get(*sd_b)
                .ok_or_else(|| format!("{path}.SD[{index}]: id not found in other arena"))?;
            let sd_path = format!("{path}.SD[{index}]");
            Self::compare_ar_object(sd_a.base(), sd_b.base(), &sd_path)?;
            if sd_a.get_gid() != sd_b.get_gid() {
                return Err(format!("{sd_path}: GID mismatch"));
            }
            if sd_a.get_xml_space() != sd_b.get_xml_space() {
                return Err(format!("{sd_path}: xml:space mismatch"));
            }
            if sd_a.get_value() != sd_b.get_value() {
                return Err(format!("{sd_path}: text mismatch"));
            }
        }

        let sdfs_a = a.get_sdf();
        let sdfs_b = b.get_sdf();
        if sdfs_a.len() != sdfs_b.len() {
            return Err(format!("{path}: SDF length mismatch"));
        }
        for (index, (sdf_a, sdf_b)) in sdfs_a.iter().zip(sdfs_b.iter()).enumerate() {
            let sdf_a = self
                .sdfs
                .get(*sdf_a)
                .ok_or_else(|| format!("{path}.SDF[{index}]: id not found in own arena"))?;
            let sdf_b = other
                .sdfs
                .get(*sdf_b)
                .ok_or_else(|| format!("{path}.SDF[{index}]: id not found in other arena"))?;
            let sdf_path = format!("{path}.SDF[{index}]");
            Self::compare_ar_object(sdf_a.base(), sdf_b.base(), &sdf_path)?;
            if sdf_a.get_gid() != sdf_b.get_gid() {
                return Err(format!("{sdf_path}: GID mismatch"));
            }
            if sdf_a.get_value() != sdf_b.get_value() {
                return Err(format!("{sdf_path}: text mismatch"));
            }
        }

        let sdgs_a = a.get_sdg();
        let sdgs_b = b.get_sdg();
        if sdgs_a.len() != sdgs_b.len() {
            return Err(format!("{path}: nested SDG length mismatch"));
        }
        for (index, (sdg_a, sdg_b)) in sdgs_a.iter().zip(sdgs_b.iter()).enumerate() {
            let sdg_a = self
                .sdgs
                .get(*sdg_a)
                .ok_or_else(|| format!("{path}.SDG[{index}]: id not found in own arena"))?;
            let sdg_b = other
                .sdgs
                .get(*sdg_b)
                .ok_or_else(|| format!("{path}.SDG[{index}]: id not found in other arena"))?;
            self.compare_sdg(other, sdg_a, sdg_b, &format!("{path}.SDG[{index}]"))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_ar_package_links_parent_and_root() {
        let mut document = Document::new();
        let root = document.add_ar_package(None, "Root");
        let child = document.add_ar_package(Some(root), "Child");

        assert_eq!(document.get_ar_packages().len(), 1);

        let root_package = document.get_ar_package(root).unwrap();
        assert_eq!(root_package.get_short_name(), Some("Root"));
        assert_eq!(root_package.get_ar_packages(), &[child]);
        assert!(root_package.get_parent().is_none());

        let child_package = document.get_ar_package(child).unwrap();
        assert_eq!(child_package.get_short_name(), Some("Child"));
        assert!(
            matches!(child_package.get_parent(), Some(ElementRef::ARPackage(parent)) if parent == root)
        );
    }

    #[test]
    fn add_element_registers_heterogeneous_child() {
        let mut document = Document::new();
        let parent = document.add_ar_package(None, "Parent");
        let child = document.add_ar_package(None, "Child");

        document.add_element(parent, ElementRef::ARPackage(child));

        assert_eq!(
            document.get_ar_package(parent).unwrap().get_elements(),
            &[ElementRef::ARPackage(child)]
        );
    }

    #[test]
    fn equal_documents_compare_equal() {
        let mut document_a = Document::new();
        document_a.set_schema_location("http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd");
        document_a.set_ar_release("R21-11");
        document_a.add_ar_package(None, "WhitespaceDemo");

        let mut document_b = Document::new();
        document_b.set_schema_location("http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd");
        document_b.set_ar_release("R21-11");
        document_b.add_ar_package(None, "WhitespaceDemo");

        document_a.assert_structurally_equal(&document_b).unwrap();
    }

    #[test]
    fn mismatched_short_names_are_reported() {
        let mut document_a = Document::new();
        document_a.add_ar_package(None, "A");

        let mut document_b = Document::new();
        document_b.add_ar_package(None, "B");

        let error = document_a
            .assert_structurally_equal(&document_b)
            .unwrap_err();
        assert!(
            error.contains("SHORT-NAME mismatch"),
            "unexpected error: {error}"
        );
    }
}
