//! Stable, per-application identifier for the current machine, persisted in the
//! system keyring so it survives application restarts.

use thiserror::Error;
use uuid::Uuid;

use crate::keyring::{self, KeyringError};

const ID_ACCOUNT: &str = "machine-id";

/// Errors returned by [`get_or_create`].
#[derive(Debug, Error)]
pub enum MachineIdError {
    /// The application name was empty.
    #[error("machineid: application name required")]
    EmptyApplication,
    /// The system keyring could not be read or written.
    #[error("machineid: keyring: {0}")]
    Keyring(#[from] KeyringError),
}

/// get_or_create returns a stable identifier for the current machine scoped to
/// the application, generating and persisting one in the system keyring on
/// first use.
pub fn get_or_create(application: &str) -> Result<String, MachineIdError> {
    if application.is_empty() {
        return Err(MachineIdError::EmptyApplication);
    }
    if let Ok(existing) = keyring::get(application, ID_ACCOUNT) {
        if !existing.is_empty() {
            return Ok(existing);
        }
    }
    let id = Uuid::new_v4().simple().to_string();
    keyring::set(application, ID_ACCOUNT, &id)?;
    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requires_application() {
        assert!(matches!(
            get_or_create(""),
            Err(MachineIdError::EmptyApplication)
        ));
    }

    #[test]
    fn stable_across_calls() {
        const APP: &str = "onyx-machineid-test";
        let Ok(first) = get_or_create(APP) else {
            return;
        };
        if keyring::get(APP, ID_ACCOUNT).is_err() {
            let _ = keyring::delete(APP, ID_ACCOUNT);
            return;
        }
        let second = get_or_create(APP).expect("second get_or_create");
        let _ = keyring::delete(APP, ID_ACCOUNT);
        assert_eq!(first, second, "identity should be stable");
    }
}
