#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DomainIdMappingPostRequest {
    #[serde(rename = "external_id")]
    pub external_id: String,
    #[serde(rename = "external_key")]
    pub external_key: String,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl DomainIdMappingPostRequest {
    pub fn new(external_id: String, external_key: String) -> DomainIdMappingPostRequest {
        DomainIdMappingPostRequest {
            external_id,
            external_key,
            name: None,
        }
    }
}