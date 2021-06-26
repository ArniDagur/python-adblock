import adblock

def test_correct_baseclasses():
    assert issubclass(adblock.AdblockException, Exception)
    assert issubclass(adblock.BlockerException, adblock.AdblockException)
    assert issubclass(adblock.SerializationError, adblock.BlockerException)
    assert issubclass(adblock.DeserializationError, adblock.BlockerException)
    assert issubclass(adblock.OptimizedFilterExistence, adblock.BlockerException)
    assert issubclass(adblock.BadFilterAddUnsupported, adblock.BlockerException)
    assert issubclass(adblock.FilterExists, adblock.BlockerException)