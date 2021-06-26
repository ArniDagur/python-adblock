import adblock
import pytest

SMALL_FILTER_LIST = """
||wikipedia.org^
||old.reddit.com^
||lobste.rs^
"""


def empty_engine():
    return adblock.Engine(adblock.FilterSet())


def test_engine_creation_and_blocking():
    filter_set = adblock.FilterSet(debug=True)
    filter_set.add_filter_list(SMALL_FILTER_LIST)
    engine = adblock.Engine(filter_set=filter_set)

    blocker_result_wikipedia = engine.check_network_urls(
        url="https://wikipedia.org/img.png",
        source_url="https://google.com/",
        request_type="image",
    )
    assert isinstance(blocker_result_wikipedia, adblock.BlockerResult)
    assert blocker_result_wikipedia.matched

    blocker_result_facebook = engine.check_network_urls(
        "https://facebook.com/directory/img.png",
        "https://old.reddit.com/r/all",
        "image",
    )
    assert isinstance(blocker_result_facebook, adblock.BlockerResult)
    assert not blocker_result_facebook.matched


def test_serde_file(tmpdir):
    path = str(tmpdir / "cache.dat")

    engine0 = empty_engine()
    with pytest.raises(FileNotFoundError):
        # We haven't created the cache.dat file, so we should get an exception
        # when attempting to deserialize.
        engine0.deserialize_from_file(path)

    engine1 = empty_engine()
    serialization_result = engine1.serialize_to_file(path)
    assert serialization_result is None

    engine2 = empty_engine()
    deserialization_result = engine2.deserialize_from_file(path)
    assert deserialization_result is None


def test_deserialize_corrupt(tmpdir):
    path = str(tmpdir / "corrupt_cache.dat")
    with open(path, "w", encoding="utf-8") as f:
        f.write("abc")

    engine = empty_engine()
    with pytest.raises(adblock.DeserializationError):
        engine.deserialize_from_file(path)
    with pytest.raises(adblock.DeserializationError):
        engine.deserialize(b"abc")


def test_serde():
    engine = empty_engine()
    serialization_result = engine.serialize()
    assert isinstance(serialization_result, bytes)

    engine2 = empty_engine()
    deserialization_result = engine2.deserialize(serialization_result)
    assert deserialization_result is None
