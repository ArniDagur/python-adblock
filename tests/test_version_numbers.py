import subprocess

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


def get_version_value_changelog():
    try:
        proc = subprocess.Popen(["changelog", "current"], stdout=subprocess.PIPE)
    except FileNotFoundError:
        return None
    assert proc.wait() == 0
    return proc.stdout.read().decode("utf-8").strip()


def test_version_numbers_all_same():
    """
    Makes sure that `pyproject.toml`, `Cargo.toml`, and `CHANGELOG.md` contain
    the same version number as the one attached to the `adblock` module.
    """
    cargo_version = get_version_value_cargo()
    poetry_version = get_version_value_poetry()
    changelog_version = get_version_value_changelog()
    module_version = adblock.__version__

    assert cargo_version == poetry_version
    assert poetry_version == module_version
    assert changelog_version is None or module_version == changelog_version
