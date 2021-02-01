python-adblock
==========
Python wrapper for Brave's adblocking library, which is written in Rust.

### Building

```
maturin build --release
```

#### Build dependencies

| Build Dependency | Versions | Arch Linux | Url |
|------------------|----------|------------|-----|
| Python           | `>=3.6`  | `python3`  | -   |
| Rust             | `>=1.45` | `rust`     | -   |
| Maturin          | `*`      | `maturin`  | https://github.com/PyO3/maturin |

### Developing

I use Poetry for development. To create and enter a virtual environment, do
```
poetry install
poetry shell
```
then, to install the `adblock` module into the virtual environment, do
```
maturin develop
```

### Documentation

Rust documentation for the latest `master` branch can be found at https://arnidagur.github.io/python-adblock/docs/adblock/index.html.

### License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.
