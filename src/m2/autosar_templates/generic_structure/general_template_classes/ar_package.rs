#[derive(Debug)]
pub struct PackageableElement {}

#[derive(Debug)]
pub struct ARObject {}

#[derive(Debug)]
pub struct ReferenceBase{}

///
/// AUTOSAR package, allowing to create top level packages to structure the contained ARElements.
/// ARPackages are open sets. This means that in a file based description system multiple files can be used
/// to partially describe the contents of a package.
/// This is an extended version of MSR’s SW-SYSTEM
///
/// Attributes
/// ----------
///   
#[derive(Debug)]
pub struct ARPackage {
    ar_packages: Vec<ARPackage>,
    elements: Vec<PackageableElement>,
    reference_bases: Vec<ReferenceBase>,
}

trait ARObjectTraits: {
    fn init(&self);

}

trait ARPackageTraits: ARObjectTraits {
    fn create_package(&self);
}

impl ARPackage {
    /// construct ARPackage Object
    pub fn new() -> Self {
        Self {
            ar_packages: Vec::new(),
            elements: Vec::new(),
            reference_bases: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ARPackage;

    #[test]
    fn create_arpackage_test() {
        let pkg = ARPackage::new();
    }
}
