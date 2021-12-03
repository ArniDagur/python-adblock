import re
import sys

import toml
import adblock


def parse_version(version):
    parts = version.split(".")
    return tuple(map(int, parts))


def get_version_value_cargo():
    with open("Cargo.toml", encoding="utf-8") as f:
        cargo_toml = toml.loads(f.read())
    return parse_version(cargo_toml["package"]["version"])


def get_version_value_changelog():
    """
    Try to get the names of all classes that we added to the Python module
    from Rust. As always, we unfortunately don't have access to the Rust AST
    so we have to make do with regular expressions.
    """
    versions = []
    with open("CHANGELOG.md", "r", encoding="utf-8") as f:
        for line in f:
            match = re.match(
                r"## ([0-9]+\.[0-9]+\.[0-9]+) - \(20[0-9]+-[0-1][0-9]-[0-3][0-9]\)",
                line.strip(),
            )
            if match is not None:
                versions.append(parse_version(match.group(1)))
    assert versions == sorted(versions, reverse=True)
    return versions[0]


def test_version_numbers_all_same():
    """
    Makes sure that `Cargo.toml` and `CHANGELOG.md` contain the same version
    number as the one attached to the `adblock` module.
    """
    cargo_version = get_version_value_cargo()
    changelog_version = get_version_value_changelog()
    module_version = parse_version(adblock.__version__)

    assert cargo_version == module_version
    assert module_version == changelog_version


def get_current_python_version():
    return (sys.version_info.major, sys.version_info.minor, sys.version_info.micro)


def test_required_python_version():
    """
    Make sure that the Python interpreter we're running this test suite on
    falls into the required Python range.
    """
    with open("Cargo.toml", encoding="utf-8") as f:
        cargo_toml = toml.loads(f.read())

    required_python = cargo_toml["package"]["metadata"]["maturin"]["requires-python"]
    assert required_python.startswith(">=")
    required_python = required_python[2:]
    assert get_current_python_version() >= parse_version(required_python)
