use crate::core::Skill;

pub struct SkillInstaller;

impl SkillInstaller {
    pub async fn install(_skill: &Skill) -> Result<(), String> {
        Err("Not implemented".to_string())
    }
}
