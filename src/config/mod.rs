pub mod io;
pub mod settings;

pub use settings::{Config, ConfigSkill, RegistryEntry, SkillSource};

#[allow(unused_imports)]
pub type _RegistryEntryExportKeepalive = RegistryEntry;
