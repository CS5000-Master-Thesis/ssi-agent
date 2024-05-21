use crate::subject::Subject;
use agent_shared::config;
use anyhow::Result;
use did_manager::SecretManager;

pub struct SecretManagerServices {
    pub subject: Option<Subject>,
    pub default_did_method: String,
}

impl SecretManagerServices {
    pub fn new(subject: Option<Subject>) -> Self {
        let default_did_method = config!("default_did_method").unwrap_or("did:key".to_string());
        Self {
            subject,
            default_did_method,
        }
    }

    pub async fn init(&mut self) -> Result<(), std::io::Error> {
        let snapshot_path = config!("stronghold_path").unwrap();
        let password = config!("stronghold_password").unwrap();
        let key_id = config!("issuer_key_id").unwrap();
        let issuer_did = config!("issuer_did");
        let issuer_fragment = config!("issuer_fragment");

        let secret_manager = SecretManager::load(
            snapshot_path,
            password,
            Some(key_id),
            None,
            issuer_did.ok(),
            issuer_fragment.ok(),
        )
        .await
        .unwrap();

        self.subject.replace(Subject { secret_manager });

        Ok(())
    }
}
