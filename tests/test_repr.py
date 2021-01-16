import adblock
import re


def assert_acceptable_repr(obj):
    # Default repr is r"<[A-Za-z]+ object at 0x[0-9a-f]+>"
    assert "object at" not in repr(obj)
    assert re.match(repr(obj), "[A-Z][a-zA-Z]+\(.*\)") or re.match(
        repr(obj), "([A-Z][a-zA-Z]+)?<.*>"
    )


def test_has_nondefault_repr():
    for b in (True, False):
        fs = adblock.FilterSet(debug=b)
        assert_acceptable_repr(fs)
        assert repr(b) in repr(fs)

    fs.add_filters(["||example.com^"])

    e = adblock.Engine(fs)
    assert_acceptable_repr(e)

    result = e.check_network_urls(
        "https://example.com/picture.png", "https://example.net", "image"
    )
    assert_acceptable_repr(result)
    assert repr(result) == (
        "BlockerResult(matched={}, important={}, redirect={}, exception={}, filter={}, error={})".format(
            repr(result.matched),
            repr(result.important),
            repr(result.redirect),
            repr(result.exception),
            repr(result.filter),
            repr(result.error),
        )
    )
