//! py `AdminDataTransformer` (`armodel/transformer/admin_data.py`):
//! removes ADMIN-DATA from the document root, every AR-PACKAGE
//! (recursively) and every package element. py's debug logging is not
//! ported; the removal itself is the 1:1 behaviour.

use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_package::ARPackageId;
use crate::m2::element_registry::element_remove_admin_data;
use crate::Document;

/// py `AdminDataTransformer`
#[derive(Debug, Default)]
pub struct AdminDataTransformer;

impl AdminDataTransformer {
    pub fn new() -> Self {
        Self
    }

    /// py `remove`
    pub fn remove(&self, document: &mut Document) {
        document.remove_admin_data();
        let root_packages = document.get_ar_packages().to_vec();
        for package_id in root_packages {
            Self::process_package(document, package_id);
        }
    }

    /// py `process_pkg`
    fn process_package(document: &mut Document, package_id: ARPackageId) {
        let (sub_packages, elements) = match document.ar_packages.get(package_id) {
            Some(package) => (
                package.get_ar_packages().to_vec(),
                package.get_elements().to_vec(),
            ),
            None => return,
        };

        for sub_package_id in sub_packages {
            Self::process_package(document, sub_package_id);
        }

        if let Some(package) = document.ar_packages.get_mut(package_id) {
            package.remove_admin_data();
        }
        for element in &elements {
            element_remove_admin_data(document, element);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_object::ElementRef;
    use crate::m2::msr::asam_hdo::admin_data::AdminData;

    #[test]
    fn remove_strips_root_packages_and_elements() {
        let mut document = Document::new();

        let root_admin_id = document.admin_datas.insert(AdminData::new());
        document.set_admin_data(root_admin_id);

        let package_id = document.add_ar_package(None, "Outer");
        let nested_id = document.add_ar_package(Some(package_id), "Inner");

        let package_admin_id = document.admin_datas.insert(AdminData::new());
        document
            .ar_packages
            .get_mut(package_id)
            .unwrap()
            .set_admin_data(package_admin_id);

        let nested_admin_id = document.admin_datas.insert(AdminData::new());
        document
            .ar_packages
            .get_mut(nested_id)
            .unwrap()
            .set_admin_data(nested_admin_id);

        // one package element: allocated through the registry and given
        // ADMIN-DATA like the parser would for a full model
        let factory = crate::m2::element_registry::element_factory_for_tag("UNIT")
            .expect("UNIT is a registry element");
        let element_ref = factory(&mut document, Some(package_id), "U1");
        let element_admin_id = document.admin_datas.insert(AdminData::new());
        let unit_id = match element_ref {
            ElementRef::Unit(unit_id) => unit_id,
            _ => panic!("UNIT factory returns a Unit"),
        };
        document
            .units
            .get_mut(unit_id)
            .unwrap()
            .set_admin_data(element_admin_id);

        AdminDataTransformer::new().remove(&mut document);

        assert!(document.get_admin_data().is_none());
        assert!(document
            .ar_packages
            .get(package_id)
            .unwrap()
            .get_admin_data()
            .is_none());
        assert!(document
            .ar_packages
            .get(nested_id)
            .unwrap()
            .get_admin_data()
            .is_none());
        assert!(document
            .units
            .get(unit_id)
            .unwrap()
            .get_admin_data()
            .is_none());
    }
}
