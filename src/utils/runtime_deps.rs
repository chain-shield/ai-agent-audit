use anyhow::{Result, bail};
use std::{
    collections::HashSet,
    env,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuntimeDependency {
    Git,
    Slither,
    Forge,
    Node,
    Npm,
    Npx,
    Yarn,
    Pnpm,
    Shell,
}

impl RuntimeDependency {
    pub fn command(self) -> &'static str {
        match self {
            Self::Git => "git",
            Self::Slither => "slither",
            Self::Forge => "forge",
            Self::Node => "node",
            Self::Npm => "npm",
            Self::Npx => "npx",
            Self::Yarn => "yarn",
            Self::Pnpm => "pnpm",
            Self::Shell => "sh",
        }
    }

    fn install_note(self) -> &'static str {
        match self {
            Self::Git => "Install Git and make sure `git` is available on PATH.",
            Self::Slither => {
                "Install Slither (`pipx install slither-analyzer` is recommended) and make sure `slither` is available on PATH."
            }
            Self::Forge => {
                "Install Foundry (`curl -L https://foundry.paradigm.xyz | bash`, then `foundryup`) and make sure `forge` is available on PATH."
            }
            Self::Node | Self::Npm | Self::Npx => {
                "Install Node.js LTS from https://nodejs.org/ so `node`, `npm`, and `npx` are available on PATH."
            }
            Self::Yarn => {
                "Install Yarn (`corepack enable` is recommended, or `npm install -g yarn`) and make sure `yarn` is available on PATH."
            }
            Self::Pnpm => {
                "Install pnpm (`corepack enable` is recommended, or `npm install -g pnpm`) and make sure `pnpm` is available on PATH."
            }
            Self::Shell => {
                "Install a POSIX-compatible shell and make sure `sh` is available on PATH."
            }
        }
    }
}

pub fn ensure_runtime_dependencies(context: &str, deps: &[RuntimeDependency]) -> Result<()> {
    let mut seen = HashSet::new();
    let mut missing = Vec::new();

    for dep in deps {
        if seen.insert(dep.command()) && !command_exists(dep.command()) {
            missing.push(*dep);
        }
    }

    if missing.is_empty() {
        return Ok(());
    }

    let mut message = format!(
        "Missing required runtime dependencies for {context}.\n\nThe Docker execution path has been removed, so these tools must be installed on the machine running ai-agent-audit:\n"
    );

    for dep in missing {
        message.push_str(&format!("\n- `{}`: {}", dep.command(), dep.install_note()));
    }

    message.push_str("\n\nAfter installing, open a new shell or update PATH and rerun the audit.");
    bail!(message);
}

fn command_exists(command: &str) -> bool {
    if command.contains(std::path::MAIN_SEPARATOR) {
        return is_executable(Path::new(command));
    }

    let Some(paths) = env::var_os("PATH") else {
        return false;
    };

    for dir in env::split_paths(&paths) {
        for candidate in command_candidates(&dir, command) {
            if is_executable(&candidate) {
                return true;
            }
        }
    }

    false
}

#[cfg(windows)]
fn command_candidates(dir: &Path, command: &str) -> Vec<PathBuf> {
    let mut candidates = vec![dir.join(command)];
    if Path::new(command).extension().is_none() {
        let pathext = env::var_os("PATHEXT")
            .and_then(|v| v.into_string().ok())
            .unwrap_or_else(|| ".COM;.EXE;.BAT;.CMD".to_string());
        for ext in pathext.split(';') {
            candidates.push(dir.join(format!("{command}{ext}")));
        }
    }
    candidates
}

#[cfg(not(windows))]
fn command_candidates(dir: &Path, command: &str) -> Vec<PathBuf> {
    vec![dir.join(command)]
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;

    path.is_file()
        && path
            .metadata()
            .map(|meta| meta.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
}

#[cfg(not(unix))]
fn is_executable(path: &Path) -> bool {
    path.is_file()
}
