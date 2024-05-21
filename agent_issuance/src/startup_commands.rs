use crate::server_config::command::ServerConfigCommand;
use agent_shared::{config, url_utils::UrlAppendHelpers};
use jsonwebtoken::Algorithm;
use oid4vci::{
    credential_format_profiles::{
        w3c_verifiable_credentials::jwt_vc_json::CredentialDefinition, CredentialFormats, Parameters,
    },
    credential_issuer::{
        authorization_server_metadata::AuthorizationServerMetadata,
        credential_configurations_supported::CredentialConfigurationsSupportedObject,
        credential_issuer_metadata::CredentialIssuerMetadata,
    },
    proof::KeyProofMetadata,
    ProofType,
};
use serde_json::json;

/// Returns the startup commands for the application.
pub fn startup_commands(host: url::Url) -> Vec<ServerConfigCommand> {
    vec![load_server_metadata(host.clone()), create_credentials_supported()]
}

pub fn load_server_metadata(base_url: url::Url) -> ServerConfigCommand {
    let issuer_display = match (config!("issuer_name"), config!("issuer_logo_url")) {
        (Ok(name), Ok(logo_url)) => Some(vec![json!({
            "name": name,
            "logo": {
                "url": logo_url
            }
        })]),
        _ => None,
    };

    ServerConfigCommand::InitializeServerMetadata {
        authorization_server_metadata: Box::new(AuthorizationServerMetadata {
            issuer: base_url.clone(),
            token_endpoint: Some(base_url.append_path_segment("auth/token")),
            ..Default::default()
        }),
        credential_issuer_metadata: CredentialIssuerMetadata {
            credential_issuer: base_url.clone(),
            credential_endpoint: base_url.append_path_segment("openid4vci/credential"),
            display: issuer_display,
            ..Default::default()
        },
    }
}

// TODO: Should not be a static startup command. Should be dynamic based on the configuration and/or updatable.
pub fn create_credentials_supported() -> ServerConfigCommand {
    ServerConfigCommand::CreateCredentialsSupported {
        credential_configurations_supported: vec![(
            "badge".to_string(),
            CredentialConfigurationsSupportedObject {
                credential_format: CredentialFormats::JwtVcJson(Parameters {
                    parameters: (
                        CredentialDefinition {
                            type_: vec!["VerifiableCredential".to_string(), "OpenBadgeCredential".to_string()],
                            credential_subject: Default::default(),
                        },
                        None,
                    )
                        .into(),
                }),
                cryptographic_binding_methods_supported: vec![
                    config!("default_did_method").unwrap_or("did:key".to_string())
                ],
                credential_signing_alg_values_supported: vec!["EdDSA".to_string()],
                proof_types_supported: vec![(
                    ProofType::Jwt,
                    KeyProofMetadata {
                        proof_signing_alg_values_supported: vec![Algorithm::EdDSA, Algorithm::ES256],
                    },
                )]
                .into_iter()
                .collect(),
                display: match (config!("credential_name"), config!("credential_logo_url")) {
                    (Ok(name), Ok(logo_url)) => vec![json!({
                        "name": name,
                        "logo": {
                            "url": logo_url
                        }
                    })],
                    _ => vec![],
                },
                ..Default::default()
            },
        )]
        .into_iter()
        .collect(),
    }
}
