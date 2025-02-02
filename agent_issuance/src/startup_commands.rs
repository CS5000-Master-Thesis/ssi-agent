use crate::server_config::command::ServerConfigCommand;
use agent_shared::{config, issuance::ServerConfig, metadata::Metadata, url_utils::UrlAppendHelpers};
use oid4vci::credential_issuer::{
    authorization_server_metadata::AuthorizationServerMetadata, credential_issuer_metadata::CredentialIssuerMetadata,
};

/// Returns the startup commands for the application.
pub fn startup_commands(host: url::Url, metadata: &Metadata) -> Vec<ServerConfigCommand> {
    vec![load_server_metadata(host, metadata), create_credentials_supported()]
}

pub fn load_server_metadata(base_url: url::Url, metadata: &Metadata) -> ServerConfigCommand {
    let display = metadata.display.first().map(|display| {
        let display = serde_json::to_value(display).unwrap();
        vec![display]
    });

    ServerConfigCommand::InitializeServerMetadata {
        authorization_server_metadata: Box::new(AuthorizationServerMetadata {
            issuer: base_url.clone(),
            token_endpoint: Some(base_url.append_path_segment("auth/token")),
            ..Default::default()
        }),
        credential_issuer_metadata: CredentialIssuerMetadata {
            credential_issuer: base_url.clone(),
            credential_endpoint: base_url.append_path_segment("openid4vci/credential"),
            display,
            ..Default::default()
        },
    }
}

pub fn create_credentials_supported() -> ServerConfigCommand {
    let server_config = config!("server_config", ServerConfig)
        .expect("Failed due to missing `server_config` in `issuance-config.yml` file");

    let credential_configuration = server_config
        .credential_configurations
        .first()
        .expect("Failed due to empty `credential_configurations` array in `issuance-config.yml` file")
        .clone();

    ServerConfigCommand::AddCredentialConfiguration {
        credential_configuration,
    }
}
