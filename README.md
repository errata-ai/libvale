# libvale

`libvale` is an IoC-like library for Vale: the goal is to make it easy to
manage consistent configurations in any environment: locally, CI, etc. Some key
features include:

- **Version management**: `libvale` will automatically install the latest
  version, or a specific version, of Vale for you.

  ```console
  # Install the latest version of Vale to `/bin`.
  $ libvale install /bin
  ...
  ```

- **Input control**: `libvale` will give to granular control over which files
  are linted and how, including respecting `.gitignore` files and linting
  only files that have changed since a given commit.

  ```console
  # Lint all Markdown files in the current directory.
  $ libvale lint --glob "*.md"
  ...
  ```

- **Named invocations**: `libvale` allows you to define named invocations of
  Vale, which can be used to lint different sets of files with different
  configurations.

  ```console
  # Run only `spelling` rules.
  $ libvale run spelling
  ...
  ```

## Example

```yaml
---
# An invocation of Vale consisting of multiple parts:
#
# - `version`: the version of Vale to use.
# - `flags`: the flags to pass to Vale.
# - `input`: the files to lint.
version: v2.20.0

inputs:
    - name: markdown
        path: .
        glob: "*.md"
        ignore: ["README.md"]
    - name: code
        path: .
        glob: "*.py"
        ignore: ["setup.py"]

invocations:
    - name: spelling
        flags: ["--filter='.Extends==spelling'", "--minAlertLevel=error"]
        inputs: ["markdown", "code"]
```
