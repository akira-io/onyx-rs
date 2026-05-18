//! Single source of truth for platform identity.
//!
//! Every other module in `onyx` asks `osinfo` instead of switching on
//! `std::env::consts::OS` directly. Keeps cross-cutting platform
//! knowledge in one place.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Platform {
    identifier: &'static str,
}

impl Platform {
    pub fn current() -> Self {
        Self {
            identifier: std::env::consts::OS,
        }
    }

    pub fn is_darwin(self) -> bool {
        self.identifier == "macos"
    }

    pub fn is_linux(self) -> bool {
        self.identifier == "linux"
    }

    pub fn is_windows(self) -> bool {
        self.identifier == "windows"
    }

    pub fn as_str(self) -> &'static str {
        self.identifier
    }
}

pub fn executable_extension() -> &'static str {
    if Platform::current().is_windows() {
        ".exe"
    } else {
        ""
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_matches_os_constant() {
        assert_eq!(Platform::current().as_str(), std::env::consts::OS);
    }

    #[test]
    fn at_most_one_predicate_holds() {
        let p = Platform::current();
        let count = [p.is_darwin(), p.is_linux(), p.is_windows()]
            .into_iter()
            .filter(|x| *x)
            .count();
        assert!(
            count <= 1,
            "expected at most one predicate true, got {}",
            count
        );
    }

    #[test]
    fn executable_extension_matches_platform() {
        let got = executable_extension();
        if Platform::current().is_windows() {
            assert_eq!(got, ".exe");
        } else {
            assert_eq!(got, "");
        }
    }
}
