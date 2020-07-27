import toml
import adblock


def get_version_value_poetry():
    with open("pyproject.toml", encoding="utf-8") as f:
        pyproject_toml = toml.loads(f.read())
    return pyproject_toml["tool"]["poetry"]["version"]


def get_version_value_cargo():
    with open("Cargo.toml", encoding="utf-8") as f:
        cargo_toml = toml.loads(f.read())
    return cargo_toml["package"]["version"]


def test_version_numbers_all_same():
    """
    Makes sure that `pyproject.toml` and `Cargo.toml` contain the same version
    number as the one attached to the `adblock` module.
    """
    cargo_version = get_version_value_cargo()
    poetry_version = get_version_value_poetry()
    module_version = adblock.__version__

    assert cargo_version == poetry_version
    assert poetry_version == module_version
    assert cargo_version == module_version
