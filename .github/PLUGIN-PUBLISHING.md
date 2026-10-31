# Plugin distribution

The manual `plugin-artifacts.yaml` workflow verifies the selected checkout's generated bundles and requires its core version to match the existing release tag. GitHub marketplace indexes pin the checkout's immutable commit; archives attach to the selected release. No new product tag or release is created.

Three distribution jobs must succeed: GitHub archives, npm OpenCode/Pi, and a Hermes wheel on PyPI. Each registry job checks versions, public exports and native plugin freshness before invoking the idempotent publisher. Published versions are skipped by the shared publisher; they are never overwritten. The final report rejects failed, cancelled or skipped jobs.

| Distribution | Package | Required GitHub trusted-publisher claims |
| --- | --- | --- |
| npm | `@goldziher/opencode-spikard` | Owner `Goldziher`, repository `spikard`, workflow `plugin-artifacts.yaml`, environment unset |
| PyPI | `spikard-hermes-plugin` | Owner `Goldziher`, repository `spikard`, workflow `plugin-artifacts.yaml`, environment `pypi` |

Both registry jobs use GitHub-hosted runners and `id-token: write`; credentials are obtained through OIDC. Configure the `pypi` GitHub environment and the registry-side trust records before dispatching publication. The workflow verifies actual npm tarball resources and builds the wheel with the normalized PEP 440 version derived from the release tag.

The registry APIs returned HTTP 404 for both package names on 2026-10-14. This means the packages are not currently available from those public endpoints; it does not prove their names are reserved for this account. Use the verified local generated packages until registry enrollment and the first publication are complete.

For npm, an authorized maintainer must establish ownership of the package/scope and configure its package-settings trusted publisher with the exact claims above; confirm the initial package-creation/authentication procedure in the current account before the first dispatch. This audit does not publish an initial version or configure a registry account. [npm trusted-publishing documentation](https://docs.npmjs.com/trusted-publishers/) describes the claims and supported runners.

For the new PyPI project, register a pending trusted publisher under the account's Publishing page using the project name and claims above. It creates the project on first authorized publication and does not reserve the name beforehand. [PyPI pending-publisher documentation](https://docs.pypi.org/trusted-publishers/creating-a-project-through-oidc/) explains that setup.

Do not manually modify generated npm or wheel manifests. Change native `plugin/.ai-rulez` authoring, regenerate and export with `mise run plugin:generate`, then verify and review the resulting committed artifacts before dispatching.
