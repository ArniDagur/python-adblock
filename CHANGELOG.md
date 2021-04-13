# CHANGELOG

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/) and [Keep a Changelog](http://keepachangelog.com/).

## Unreleased
---
### Changes
* PyO3 is now configured to use [`abi3`](https://pyo3.rs/v0.13.2/building_and_distribution.html#py_limited_apiabi3).

## 0.4.3 - (2021-03-20)
---
### Changes
* Update `adblock` dependency to `0.3.10`

## 0.4.2 - (2021-02-01)
---
### Fixes
* Remove relative import which caused problems in [#17](https://github.com/ArniDagur/python-adblock/issues/17).


## 0.4.1 - (2021-01-27)
---

### New
* Windows 32-bit prebuilt wheels.

### Changes
* Updated PyO3 to version `0.13`.
* Changed `__repr__` methods of classes to be more idiomatic.

### Breaks
* Dropped Python `3.5` support.


## 0.4.0 - (2020-12-16)
---

### New
* Maintain a `CHANGELOG.md` file.
* Include `generichide` field in `UrlSpecificResources`.

### Fixes
* Include `Cargo.lock` in source control, fixing incorrect dependency resolution [#15](https://github.com/ArniDagur/python-adblock/issues/15).

### Breaks
* Remove `explicit_cancel` field from `BlockerResult`, as it has been removed upstream.


## 0.3.2 - (2020-09-22)
---

### New
* Build Python 3.9 wheels.

### Changes
* Updated PyO3 to version `0.12`.

### Fixes
* Don't use star imports in `__init__.py` to give linters and type checkers more information.
