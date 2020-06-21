import adblock
import pytest


def test_engine_arguments():
    # None of these should panic
    adblock.Engine()
    adblock.Engine([])
    adblock.Engine(network_filters=None)
    adblock.Engine(network_filters=[])
    adblock.Engine(load_network=False, load_cosmetic=True, debug=False)
    adblock.Engine(debug=True)


def test_serde_file(tmpdir):
    path = str(tmpdir / "cache.dat")

    engine0 = adblock.Engine()
    with pytest.raises(FileNotFoundError):
        # We haven't created the cache.dat file, so we should get an exception
        # when attempting to deserialize.
        engine0.deserialize_from_file(path)

    engine1 = adblock.Engine()
    serialization_result = engine1.serialize_to_file(path)
    assert serialization_result is None

    engine2 = adblock.Engine()
    deserialization_result = engine2.deserialize_from_file(path)
    assert deserialization_result is None


def test_serde():
    engine = adblock.Engine()
    serialization_result = engine.serialize()
    assert isinstance(serialization_result, bytes)

    engine2 = adblock.Engine()
    deserialization_result = engine2.deserialize(serialization_result)
    assert deserialization_result is None
