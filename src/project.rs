use crate::configuration::CONFIGURATION;

/// Shows we can refer to CONFIGURATION in sub-modules.
#[allow(unused)]
pub fn foo() {
    let v = CONFIGURATION.verbose;
}
