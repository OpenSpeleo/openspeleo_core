# CI and releases

## Workflow responsibilities

[CI](../.github/workflows/workflow.yml) runs linting, Rust checks, and Python
3.11–3.14 tests on Linux, Windows, and macOS for pull requests, pushes to
`master`, and manual runs. After all checks pass on a push to `master`,
`trigger-release` fetches the tags on that exact commit. If it has a `v*` tag,
the job dispatches `release.yml` at that tag; otherwise it succeeds without
starting a release. Pull requests and manual CI runs do not dispatch releases.

[Release](../.github/workflows/release.yml) accepts `workflow_dispatch` and
builds manylinux, musllinux, Windows, and macOS wheels plus a source
distribution in independent jobs. The existing architecture and Python version
matrices are preserved. Once every build succeeds, `publish-release` downloads
the `wheels-*` artifacts from that release run, attests them, and publishes to
PyPI through the `pypi` environment. Publication requires a `v*` tag; manually
selecting a branch builds artifacts but skips publication. Runs for the same ref
are serialized, and already-published distributions are skipped on retries.

## Preparing a release

1. Set the package version in `Cargo.toml` and refresh the affected lockfiles.
2. Ensure `release.yml` exists on both the default branch and the release
   commit. GitHub requires dispatchable workflows to exist on the default
   branch, and `--ref` selects the tag's workflow definition.
3. Configure the PyPI project's GitHub Trusted Publisher for owner `OpenSpeleo`,
   repository `openspeleo_core`, workflow filename `release.yml`, and
   environment `pypi`. If the publisher currently references `workflow.yml`,
   update it for the new filename. Preserve any approval or tag restrictions on
   the GitHub `pypi` environment.
4. Push the release commit to `master` with one corresponding `v<version>` tag
   available before CI reaches `trigger-release`. CI validates the commit and
   then dispatches the release workflow.

A tag pushed after CI completes does not start a release automatically. After
checking that CI passed for the tagged commit, dispatch it explicitly:

```sh
gh workflow run release.yml --ref v<version>
```

Use the same command to retry a release. Review the Release run separately from
CI: a successful dispatch means GitHub accepted the run, not that the builds or
PyPI publication have completed. Manual release dispatch does not rerun the CI
checks.

See GitHub's
[workflow dispatch documentation](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows#workflow_dispatch)
and PyPI's
[Trusted Publisher setup](https://docs.pypi.org/trusted-publishers/adding-a-publisher/).
