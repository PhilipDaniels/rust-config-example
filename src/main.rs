mod configuration;
mod project;

use configuration::CONFIGURATION;

fn main() {
    println!("The configuration is {:#?}", *CONFIGURATION);
}


mod submod {
    use crate::configuration::CONFIGURATION;

    /// Shows we can refer to CONFIGURATION in sub-modules.
    /// See also project.rs.
    #[allow(unused)]
    fn foo() {
        let v = CONFIGURATION.verbose;
    }
}
