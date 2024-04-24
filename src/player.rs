use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    name: String,
    position: String,

    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,

    #[serde(rename = "Kit Number")]
    kit_number: String,
}
