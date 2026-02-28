
- gh workflow run windows-harness.yml failed with 404 because workflow file not yet pushed to remote.
- Workflow dispatch requires committing and pushing the new workflow and docs to GitHub.
- gh workflow run still returns 404 after pushing to feature branch; likely Actions API access issue or workflow not registered yet.
- PR opened for merge to develop: https://github.com/chozandrias76/sc-interdiction/pull/73
- Cannot approve or merge own PR; merge blocked by branch policy requiring external approval.
- Windows Harness run failed after merge: dtolnay/rust-toolchain step uses bash and cannot access Windows temp script path (C:Users...); add Git Bash to PATH before toolchain step.
