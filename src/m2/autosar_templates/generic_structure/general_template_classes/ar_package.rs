
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
    //element: Vec<PackageableElement>,
    //referenceBase: Vec<ReferenceBase>,
}

trait ARPackageInterface {
    fn create_package(&self);
}

impl ARPackageInterface for ARPackage {
    /// construct ARPackage Object
    pub fn new() -> ARPackage {
        println!("Create ARPackage");
        ARPackage {
            ar_packages: Vec::new(),
            //element: Vec::new(),
        }
    }

    pub create_package(short_name: str&){
        
    }
fn create_package(&self) { todo!() }
}

#[cfg(test)]
mod tests {
    use super::ARPackage;

    #[test]
    fn create_arpackage_test() {
        let _pkg = ARPackage::new();
    }
}
