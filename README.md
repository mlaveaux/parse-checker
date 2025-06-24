
# Tool

In the mCRL2 202507.0 release of the mCRL2 toolset, see the
[repository](https://github.com/mCRL2org/mCRL2) or [website](https://mcrl2.org),
we have adapted the parser to conform with the priorities written in the
corresponding book. This is a tool that can be used to find parsing differences, especially for modal formulas and mCRL2 specifications.

# Building

This repository uses git [submodules](https://git-scm.com/book/en/v2/Git-Tools-Submodules) to manage third party dependencies, which means that we must use

```
git submodule update --init
```

to acquire the submodules the first time. This command must be repeated after any git command (git checkout, pull, etc) whenever the third party dependencies have changed in that HEAD. Cargo does not yet allow binary dependencies, so after that `cargo build (--release)` must be used to build both binaries.

# Usage

The command line interface is provided by `parse-checker`.

# Details

This tool contains two different versions of the mCRL2 toolset before and after
the changes to the parser. Linking these versions together has some nasty
consequences due to how linking works for C and C++ projects. In particular, the
`dparser` C library will have a lot of name clashes of the two different parsers
that will silently be resolved. Therefore, `parse-checker` is the main binary that only links the new
parser directly, and uses a binary called `mcrl2-2024` that only links the old parser and has a simple internal command line interface for `parse-checker`. So the latter tool does not have to called directly, but must be present in PATH or next to the `parse-checker`. 