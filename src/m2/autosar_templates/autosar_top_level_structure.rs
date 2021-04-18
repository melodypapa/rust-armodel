///
/// Root element of an AUTOSAR description, also the root element in correspondingXML documents.
///  Base ARObject
///
/// Atributes
/// ---------
/// admin_data: AdminData (0..1) This represents the administrative data of anAutosar file.
/// ar_packages: ARPackage (optional) This is the top level package in an AUTOSARmodel.
/// file_info_comment:
///
pub struct AUTOSAR {
    admin_data: str&;
    ar_package: Vec<ARPackage>
}
