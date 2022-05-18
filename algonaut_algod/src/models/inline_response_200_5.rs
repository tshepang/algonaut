/*
 * Algod REST API.
 *
 * API endpoint for algod operations.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: contact@algorand.com
 * Generated by: https://openapi-generator.tech
 */

/// InlineResponse2005 : An catchpoint start response.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InlineResponse2005 {
    /// Catchup start response string
    #[serde(rename = "catchup-message")]
    pub catchup_message: String,
}

impl InlineResponse2005 {
    /// An catchpoint start response.
    pub fn new(catchup_message: String) -> InlineResponse2005 {
        InlineResponse2005 {
            catchup_message,
        }
    }
}

