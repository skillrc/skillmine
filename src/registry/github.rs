pub struct GitHubRegistry;

impl GitHubRegistry {
    pub fn fetch_skill(_owner: &str, _repo: &str) -> Result<String, String> {
        Err("Not implemented".to_string())
    }
}
