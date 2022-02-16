import adblock
import pytest

def test_correct_baseclasses():
    assert issubclass(adblock.AdblockException, Exception)
    assert issubclass(adblock.BlockerException, adblock.AdblockException)
    assert issubclass(adblock.SerializationError, adblock.BlockerException)
    assert issubclass(adblock.DeserializationError, adblock.BlockerException)
    assert issubclass(adblock.OptimizedFilterExistence, adblock.BlockerException)
    assert issubclass(adblock.BadFilterAddUnsupported, adblock.BlockerException)
    assert issubclass(adblock.FilterExists, adblock.BlockerException)
    assert issubclass(adblock.AddResourceError, adblock.BlockerException)


def test_add_resource_error():
    filter_set = adblock.FilterSet()
    engine = adblock.Engine(filter_set=filter_set)

    with pytest.raises(adblock.AddResourceError) as exc:
        engine.add_resource(name='aa', content_type="image/jpeg", content="111")

    assert "invalid base64 content" in str(exc.value)
