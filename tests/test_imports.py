import re
import adblock


def get_added_classes():
    """
    Try to get the names of all classes that we added to the Python module
    from Rust. As always, we unfortunately don't have access to the Rust AST
    so we have to make do with regular expressions.
    """
    classes = []
    with open("src/lib.rs", "r", encoding="utf-8") as rs_f:
        for line in rs_f:
            match = re.match(r"m\.add_class::<(.+)>\(\)\?;", line.strip())
            if match is not None:
                classes.append(match.group(1))
                continue
    return classes


def test_added_classes():
    """
    Make sure that there's no class that we added in Rust but didn't import in
    `__init__.py`.
    """
    added_classes = get_added_classes()
    for c in added_classes:
        assert c in adblock.__all__


def test_dunder_all_classes_imported():
    """
    Make sure that there's no class in `__all__` that we haven't imported.
    """
    for c in adblock.__all__:
        assert hasattr(adblock, c)
