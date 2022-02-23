# python-adblock

Python wrapper for Brave's adblocking library, which is written in Rust.

### Building from source

#### Build dependencies

| Build Dependency | Versions | Arch Linux | Url                             |
| ---------------- | -------- | ---------- | ------------------------------- |
| Python           | `>=3.6`  | `python`   | -                               |
| Rust             | `>=1.49` | `rust`     | -                               |
| Maturin          | `>=0.10` | `maturin`  | https://github.com/PyO3/maturin |

#### PEP 517

The `python-adblock` library is [PEP 517](https://www.python.org/dev/peps/pep-0517/) compatible, so you can build and install it from source, simply by running

```
pip install .
```

from the root of this directory.

#### Wheels

To create a wheel for this library, run the following command

```
maturin build --release --no-sdist --out dist/
```

the result can be found in the `dist/` directory.

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

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.
