<h1 align="center">Lithra</h1>
<p align="center"><strong>Reproducible systems and images, described in a language you can read.</strong></p>

Lithra is an early experiment in keeping what makes Nix great, and fixing what makes it hard to use. What stays: a content-addressed store, atomic generations with rollback, and one declarative description of a machine or container image. What changes:

- **Typed Starlark for package definitions.** It's deterministic, evaluates in parallel, and loads only the packages you ask for. Overrides are plain function arguments.
- **TOML for users.** Describing a system or an image means listing what you want, without learning a build language.
- **Rust tooling** with errors that point at your code.
- **Minimal, reproducible OCI images** that come with an SBOM and provenance.

## Status

Nothing works yet. The first milestone is evaluating a Starlark package definition into a build plan and benchmarking it against `nix eval`.

## License

[MIT](LICENSE)
