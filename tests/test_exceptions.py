import adblock
import pytest


def test_correct_baseclasses():
    assert issubclass(adblock.AdblockException, Exception)
    assert issubclass(adblock.BlockerException, adblock.AdblockException)
    assert issubclass(adblock.AddResourceException, adblock.AdblockException)
    assert issubclass(adblock.InvalidUtf8ContentError, adblock.AddResourceException)
    assert issubclass(adblock.InvalidBase64ContentError, adblock.AddResourceException)
    assert issubclass(adblock.SerializationError, adblock.BlockerException)
    assert issubclass(adblock.DeserializationError, adblock.BlockerException)
    assert issubclass(adblock.OptimizedFilterExistence, adblock.BlockerException)
    assert issubclass(adblock.BadFilterAddUnsupported, adblock.BlockerException)
    assert issubclass(adblock.FilterExists, adblock.BlockerException)


def test_add_resource_error():
    filter_set = adblock.FilterSet()
    engine = adblock.Engine(filter_set=filter_set)

    with pytest.raises(adblock.InvalidBase64ContentError) as exc:
        engine.add_resource(name="aa", content_type="image/jpeg", content="111")
    assert "invalid base64 content" in str(exc.value)

    with pytest.raises(adblock.InvalidUtf8ContentError) as exc:
        # // Ensure any text contents are also valid utf8
        # MimeType::ApplicationJavascript | MimeType::TextPlain | MimeType::TextHtml => {
        #     let _ = String::from_utf8(decoded)?;
        # }
        # xOO6ww== => base64.b64encode('你好'.encode('gbk'))
        engine.add_resource(
            name="aa", content_type="application/javascript", content="xOO6ww=="
        )
    assert "invalid utf content" in str(exc.value)
