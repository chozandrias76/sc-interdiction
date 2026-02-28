
- Added `.github/workflows/windows-harness.yml` with workflow_dispatch mode input, self-hosted Windows runner labels, matrix commands including llvm-cov, LLVM install via Chocolatey, and artifact uploads.
- Documented Windows Harness prerequisites and dispatch command in docs/RELEASE_WORKFLOW.md.
- Windows runners need Git Bash available for dtolnay/rust-toolchain; prepend Git Bash to PATH before toolchain install.
- Ensure Cargo bin ($USERPROFILE\.cargo\bin) is added to PATH before running cargo commands in pwsh.
