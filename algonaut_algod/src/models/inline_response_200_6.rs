/*
 * Algod REST API.
 *
 * API endpoint for algod operations.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: contact@algorand.com
 * Generated by: https://openapi-generator.tech
 */

/// InlineResponse2006 : An catchpoint abort response.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InlineResponse2006 {
    /// Catchup abort response string
    #[serde(rename = "catchup-message")]
    pub catchup_message: String,
}

impl InlineResponse2006 {
    /// An catchpoint abort response.
    pub fn new(catchup_message: String) -> InlineResponse2006 {
        InlineResponse2006 {
            catchup_message,
        }
    }
}

