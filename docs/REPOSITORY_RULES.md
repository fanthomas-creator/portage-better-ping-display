# Repository rules

The maintainer is **@fanthomas-creator**. Public visibility lets people read and fork the code; it does not grant them write access. Do not grant write access to contributors just because they open a pull request.

The intended `main` ruleset requires pull requests, one code-owner approval, resolved review conversations and a successful `checks` CI job. Stale approvals are dismissed. Force pushes and branch deletion are blocked. The administrator may bypass through a pull request to handle their own work in a single-maintainer project; the maintainer must still inspect the diff and CI result.

Release tags matching `v*` should be protected against updates and deletion, and tag creation restricted to the maintainer. Releases are signed locally. No automated pull-request workflow receives a signing key or repository write token.

The machine-readable rulesets are in `.github/rulesets`. Keeping JSON in the repository **does not activate GitHub protection**. Import or apply them in Settings → Rules → Rulesets and verify that enforcement is Active. The final setup report must distinguish prepared rules from protections actually enabled on GitHub.

Recommended repository settings:

- Require approval before running workflows from outside collaborators.
- Use read-only default workflow permissions and disable Actions creating or approving pull requests.
- Allow squash merging and remove merged branches automatically.
- Enable private vulnerability reporting when available.
- Keep force pushes and deletion disabled for protected branches and tags.
- Never store a private minisign key in repository files, issues, artifacts or CI secrets for this manual-release project.

Branch rules reduce accidental or unreviewed changes. They cannot prevent an account owner from deliberately changing the repository's administration. Protect the GitHub account itself and review any future access grants.
