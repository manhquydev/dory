//! `--current` / omit-target law (contract §5, phase 3).
//!
//! `--current` reads injected `DORY_PANE_ID`. Outside env is a runtime
//! error. Omitting a target is usage. Never invent a focused pane.

use std::env;
use std::ffi::OsStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetError {
    OutsideEnv,
    OmitTarget,
}

pub fn require_env() -> Result<(), TargetError> {
    if env::var_os("DORY_ENV").as_deref() == Some(OsStr::new("1")) {
        Ok(())
    } else {
        Err(TargetError::OutsideEnv)
    }
}

pub fn pane_from_current_flag(args: &[String]) -> Result<String, TargetError> {
    if args.iter().any(|a| a == "--current") {
        require_env()?;
        match env::var("DORY_PANE_ID") {
            Ok(id) if !id.is_empty() => Ok(id),
            _ => Err(TargetError::OutsideEnv),
        }
    } else if let Some(id) = flag_value(args, "--pane").filter(|id| !id.is_empty()) {
        Ok(id.to_string())
    } else {
        Err(TargetError::OmitTarget)
    }
}

pub fn exit_code(err: TargetError) -> i32 {
    match err {
        TargetError::OutsideEnv => 1,
        TargetError::OmitTarget => 2,
    }
}

fn flag_value<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    let prefix = format!("{flag}=");
    let mut i = 0;
    while i < args.len() {
        if args[i] == flag {
            return args.get(i + 1).map(String::as_str);
        }
        if let Some(rest) = args[i].strip_prefix(&prefix) {
            return Some(rest);
        }
        i += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;
    use std::sync::{Mutex, MutexGuard};

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    struct EnvGuard {
        prev_env: Option<OsString>,
        prev_pane: Option<OsString>,
        _lock: MutexGuard<'static, ()>,
    }

    impl EnvGuard {
        fn apply(dory_env: Option<&str>, pane_id: Option<&str>) -> Self {
            let lock = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
            let prev_env = env::var_os("DORY_ENV");
            let prev_pane = env::var_os("DORY_PANE_ID");
            match dory_env {
                Some(v) => unsafe { env::set_var("DORY_ENV", v) },
                None => unsafe { env::remove_var("DORY_ENV") },
            }
            match pane_id {
                Some(v) => unsafe { env::set_var("DORY_PANE_ID", v) },
                None => unsafe { env::remove_var("DORY_PANE_ID") },
            }
            Self {
                prev_env,
                prev_pane,
                _lock: lock,
            }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match &self.prev_env {
                Some(v) => unsafe { env::set_var("DORY_ENV", v) },
                None => unsafe { env::remove_var("DORY_ENV") },
            }
            match &self.prev_pane {
                Some(v) => unsafe { env::set_var("DORY_PANE_ID", v) },
                None => unsafe { env::remove_var("DORY_PANE_ID") },
            }
        }
    }

    fn args(parts: &[&str]) -> Vec<String> {
        parts.iter().map(|s| (*s).to_string()).collect()
    }

    #[test]
    fn current_without_env_is_outside() {
        let _g = EnvGuard::apply(None, Some("w2:p9"));
        let err = pane_from_current_flag(&args(&["--current"])).unwrap_err();
        assert_eq!(err, TargetError::OutsideEnv);
        assert_eq!(exit_code(err), 1);
        assert_eq!(require_env(), Err(TargetError::OutsideEnv));
    }

    #[test]
    fn current_reads_injected_pane_id() {
        let _g = EnvGuard::apply(Some("1"), Some("w2:p9"));
        assert_eq!(
            pane_from_current_flag(&args(&["--current"])).unwrap(),
            "w2:p9"
        );
    }

    #[test]
    fn omit_target_is_not_focused_pane() {
        let err = pane_from_current_flag(&args(&["pane", "split"])).unwrap_err();
        assert_eq!(err, TargetError::OmitTarget);
        assert_eq!(exit_code(err), 2);
    }

    #[test]
    fn explicit_pane_flag_returns_id() {
        let _g = EnvGuard::apply(None, None);
        assert_eq!(
            pane_from_current_flag(&args(&["--pane", "w3:p1"])).unwrap(),
            "w3:p1"
        );
    }
}
