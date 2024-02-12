use serde::Deserialize;

use super::value_object::Subject;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum CredentialCommand {
    LoadCredentialFormatTemplate {
        credential_format_template: serde_json::Value,
    },
    CreateUnsignedCredential {
        // subject: Subject,
        credential: serde_json::Value,
    },
}
