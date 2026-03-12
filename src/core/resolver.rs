use crate::core::Skill;

pub struct SkillResolver;

impl SkillResolver {
    pub fn resolve(_skill_name: &str) -> Result<Skill, String> {
        Err("Not implemented".to_string())
    }
}
