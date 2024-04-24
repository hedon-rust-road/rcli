use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    pub name: String,
    pub position: String,

    #[serde(rename = "DOB")]
    pub dob: String,
    pub nationality: String,

    #[serde(rename = "Kit Number")]
    pub kit_number: String,
}
