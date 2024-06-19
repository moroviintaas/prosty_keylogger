
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize )]
pub enum Gender{
    Male,
    Female,
    MaleActingAsFemale,
    FemaleActingAsMale,
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PersonalData{
    //pub id: u64,
    pub name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<Gender>
}