# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## Breaking Changes

- `NodeIndex` is no longer just `Ix`, but a newtype wrapper around `Ix`. This is a breaking change for any code that
  relied on `NodeIndex` not being an opaque type.

## Added

- Moved `petgraph::csr` to `petgraph-csr`

[unreleased]: https://github.com/olivierlacan/keep-a-changelog/compare/petgraph@v0.6.3...HEAD