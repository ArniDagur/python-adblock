import ast
import re


def read_stubfile():
    with open("adblock/adblock.pyi", encoding="utf-8") as file:
        node = ast.parse(file.read())
    return node


def get_functions_and_methods(node):
    functions = [n for n in node.body if isinstance(n, ast.FunctionDef)]
    classes = [n for n in node.body if isinstance(n, ast.ClassDef)]

    methods = {}
    for c in classes:
        methods[c.name] = [n for n in c.body if isinstance(n, ast.FunctionDef)]

    return functions, methods


def pattern_exists_in_file(filename, regex):
    """
    Checks if the given regex is present in the given file
    """
    with open(filename, "r", encoding="utf-8") as f:
        for line in f:
            if re.search(regex, line):
                return True
    return False


def test_functions_and_methods_exist_in_rust():
    """
    Check that for each of the functions and methods present in the Python
    typestub file, there is a line in `src/lib.rs` containing a matching
    definition. Since we're doing a naive grep search, without access to the
    Rust AST, there may be false negatives.
    """
    stubfile_node = read_stubfile()
    functions, methods = get_functions_and_methods(stubfile_node)

    methods_flattened = []
    for class_methods in methods.values():
        methods_flattened += class_methods

    for f in functions + methods_flattened:
        if f.name.startswith("__"):
            # Skip dunder methods since their names are the same for every
            # class, making the test not particularly useful. They are also not
            # marked `pub` in Rust.
            continue
        assert pattern_exists_in_file("src/lib.rs", r"pub fn {}".format(f.name))
