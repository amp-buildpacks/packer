use eyre::Result;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
};
use tracing::trace;

#[derive(Clone, Copy, Debug)]
pub struct Git<'a> {
    pub root: &'a Path,
    pub quiet: bool,
    pub shallow: bool,
}

impl<'a> Git<'a> {
    #[inline]
    pub fn new(root: &'a Path) -> Self {
        Self {
            root,
            quiet: false,
            shallow: false,
        }
    }

    pub fn root_of(relative_to: &Path) -> Result<PathBuf> {
        let output = Self::cmd_no_root()
            .current_dir(relative_to)
            .args(["rev-parse", "--show-toplevel"])
            .get_stdout_lossy()?;
        Ok(PathBuf::from(output))
    }

    pub fn clone_with_branch(
        shallow: bool,
        from: impl AsRef<OsStr>,
        branch: impl AsRef<OsStr>,
        to: Option<impl AsRef<OsStr>>,
    ) -> Result<()> {
        Self::cmd_no_root()
            .stderr(Stdio::inherit())
            .args(["clone", "--recurse-submodules"])
            .args(shallow.then_some("--depth=1"))
            .args(shallow.then_some("--shallow-submodules"))
            .arg("-b")
            .arg(branch)
            .arg(from)
            .args(to)
            .exec()
            .map(drop)
    }

    pub fn clone(
        shallow: bool,
        from: impl AsRef<OsStr>,
        to: Option<impl AsRef<OsStr>>,
    ) -> Result<()> {
        Self::cmd_no_root()
            .stderr(Stdio::inherit())
            .args(["clone", "--recurse-submodules"])
            .args(shallow.then_some("--depth=1"))
            .args(shallow.then_some("--shallow-submodules"))
            .arg(from)
            .args(to)
            .exec()
            .map(drop)
    }

    pub fn fetch(
        self,
        shallow: bool,
        remote: impl AsRef<OsStr>,
        branch: Option<impl AsRef<OsStr>>,
    ) -> Result<()> {
        self.cmd()
            .stderr(Stdio::inherit())
            .arg("fetch")
            .args(shallow.then_some("--no-tags"))
            .args(shallow.then_some("--depth=1"))
            .arg(remote)
            .args(branch)
            .exec()
            .map(drop)
    }

    #[inline]
    pub fn root(self, root: &Path) -> Git<'_> {
        Git { root, ..self }
    }

    #[inline]
    pub fn quiet(self, quiet: bool) -> Self {
        Self { quiet, ..self }
    }

    /// True to perform shallow clones
    #[inline]
    pub fn shallow(self, shallow: bool) -> Self {
        Self { shallow, ..self }
    }

    pub fn checkout(self, recursive: bool, tag: impl AsRef<OsStr>) -> Result<()> {
        self.cmd()
            .arg("checkout")
            .args(recursive.then_some("--recurse-submodules"))
            .arg(tag)
            .exec()
            .map(drop)
    }

    pub fn init(self) -> Result<()> {
        self.cmd().arg("init").exec().map(drop)
    }

    #[allow(clippy::should_implement_trait)] // this is not std::ops::Add clippy
    pub fn add<I, S>(self, paths: I) -> Result<()>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.cmd().arg("add").args(paths).exec().map(drop)
    }

    pub fn reset(self, hard: bool, tree: impl AsRef<OsStr>) -> Result<()> {
        self.cmd()
            .arg("reset")
            .args(hard.then_some("--hard"))
            .arg(tree)
            .exec()
            .map(drop)
    }

    pub fn commit_tree(
        self,
        tree: impl AsRef<OsStr>,
        msg: Option<impl AsRef<OsStr>>,
    ) -> Result<String> {
        self.cmd()
            .arg("commit-tree")
            .arg(tree)
            .args(msg.as_ref().is_some().then_some("-m"))
            .args(msg)
            .get_stdout_lossy()
    }

    pub fn rm<I, S>(self, force: bool, paths: I) -> Result<()>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.cmd()
            .arg("rm")
            .args(force.then_some("--force"))
            .args(paths)
            .exec()
            .map(drop)
    }

    pub fn commit(self, msg: &str) -> Result<()> {
        let output = self
            .cmd()
            .args(["commit", "-m", msg])
            .args(cfg!(any(test, debug_assertions)).then_some("--no-gpg-sign"))
            .output()?;
        if !output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            // ignore "nothing to commit" error
            let msg = "nothing to commit, working tree clean";
            if !(stdout.contains(msg) || stderr.contains(msg)) {
                return Err(eyre::eyre!(
                    "failed to commit (code={:?}, stdout={:?}, stderr={:?})",
                    output.status.code(),
                    stdout.trim(),
                    stderr.trim()
                ));
            }
        }
        Ok(())
    }

    pub fn is_in_repo(self) -> std::io::Result<bool> {
        self.cmd()
            .args(["rev-parse", "--is-inside-work-tree"])
            .status()
            .map(|s| s.success())
    }

    pub fn is_clean(self) -> Result<bool> {
        self.cmd()
            .args(["status", "--porcelain"])
            .exec()
            .map(|out| out.stdout.is_empty())
    }

    pub fn has_branch(self, branch: impl AsRef<OsStr>) -> Result<bool> {
        self.cmd()
            .args(["branch", "--list", "--no-color"])
            .arg(branch)
            .get_stdout_lossy()
            .map(|stdout| !stdout.is_empty())
    }

    pub fn tag(self) -> Result<String> {
        self.cmd().arg("tag").get_stdout_lossy()
    }

    pub fn has_missing_dependencies<I, S>(self, paths: I) -> Result<bool>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.cmd()
            .args(["submodule", "status"])
            .args(paths)
            .get_stdout_lossy()
            .map(|stdout| stdout.lines().any(|line| line.starts_with('-')))
    }

    /// Returns true if the given path has no submodules by checking `git submodule status`
    pub fn has_submodules<I, S>(self, paths: I) -> Result<bool>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.cmd()
            .args(["submodule", "status"])
            .args(paths)
            .get_stdout_lossy()
            .map(|stdout| stdout.trim().lines().next().is_some())
    }

    pub fn submodule_add(
        self,
        force: bool,
        url: impl AsRef<OsStr>,
        path: impl AsRef<OsStr>,
    ) -> Result<()> {
        self.cmd()
            .stderr(self.stderr())
            .args(["submodule", "add"])
            .args(self.shallow.then_some("--depth=1"))
            .args(force.then_some("--force"))
            .arg(url)
            .arg(path)
            .exec()
            .map(drop)
    }

    pub fn submodule_update<I, S>(
        self,
        force: bool,
        remote: bool,
        no_fetch: bool,
        recursive: bool,
        paths: I,
    ) -> Result<()>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.cmd()
            .stderr(self.stderr())
            .args(["submodule", "update", "--progress", "--init"])
            .args(self.shallow.then_some("--depth=1"))
            .args(force.then_some("--force"))
            .args(remote.then_some("--remote"))
            .args(no_fetch.then_some("--no-fetch"))
            .args(recursive.then_some("--recursive"))
            .args(paths)
            .exec()
            .map(drop)
    }

    pub fn submodule_foreach(self, recursive: bool, cmd: impl AsRef<OsStr>) -> Result<()> {
        self.cmd()
            .stderr(self.stderr())
            .args(["submodule", "foreach"])
            .args(recursive.then_some("--recursive"))
            .arg(cmd)
            .exec()
            .map(drop)
    }

    pub fn submodule_init(self) -> Result<()> {
        self.cmd()
            .stderr(self.stderr())
            .args(["submodule", "init"])
            .exec()
            .map(drop)
    }

    pub fn cmd(self) -> Command {
        let mut cmd = Self::cmd_no_root();
        cmd.current_dir(self.root);
        cmd
    }

    pub fn cmd_no_root() -> Command {
        let mut cmd = Command::new("git");
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
        cmd
    }

    // don't set this in cmd() because it's not wanted for all commands
    fn stderr(self) -> Stdio {
        if self.quiet {
            Stdio::piped()
        } else {
            Stdio::inherit()
        }
    }
}

/// Useful extensions to [`std::process::Command`].
pub trait CommandUtils {
    /// Returns the command's output if execution is successful, otherwise, throws an error.
    fn exec(&mut self) -> Result<Output>;

    /// Returns the command's stdout if execution is successful, otherwise, throws an error.
    fn get_stdout_lossy(&mut self) -> Result<String>;
}

impl CommandUtils for Command {
    #[track_caller]
    fn exec(&mut self) -> Result<Output> {
        trace!(command=?self, "executing");

        let output = self.output()?;

        trace!(code=?output.status.code(), ?output);

        if output.status.success() {
            Ok(output)
        } else {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stdout = stdout.trim();
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stderr = stderr.trim();
            let msg = if stdout.is_empty() {
                stderr.to_string()
            } else if stderr.is_empty() {
                stdout.to_string()
            } else {
                format!("stdout:\n{stdout}\n\nstderr:\n{stderr}")
            };

            let mut name = self.get_program().to_string_lossy();
            if let Some(arg) = self.get_args().next() {
                let arg = arg.to_string_lossy();
                if !arg.starts_with('-') {
                    let name = name.to_mut();
                    name.push(' ');
                    name.push_str(&arg);
                }
            }

            let mut err = match output.status.code() {
                Some(code) => format!("{name} exited with code {code}"),
                None => format!("{name} terminated by a signal"),
            };
            if !msg.is_empty() {
                err.push(':');
                err.push(if msg.lines().count() == 0 { ' ' } else { '\n' });
                err.push_str(&msg);
            }
            Err(eyre::eyre!(err))
        }
    }

    #[track_caller]
    fn get_stdout_lossy(&mut self) -> Result<String> {
        let output = self.exec()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.trim().into())
    }
}
