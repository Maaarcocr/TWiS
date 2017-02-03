#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
    pub name: String,
    pub clone_url: String
}