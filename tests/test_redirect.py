import adblock


def test_redirect_worked_as_excepted_with_include_redirect_urls():
    # https://github.com/brave/adblock-rust/blob/b7f29af8c0a0d000201d8d769b6a0b25a9dd4e89/src/blocker.rs#L1242
    filter_set = adblock.FilterSet()
    filter_set.add_filter_list(
        "||foo.com$important,redirect-url=http://xyz.com", include_redirect_urls=True
    )

    engine = adblock.Engine(filter_set=filter_set)

    res = engine.check_network_urls("https://foo.com", "https://foo.com", "script")
    assert res.matched is True
    assert res.important is True
    assert res.redirect_type == "url"
    assert res.redirect == "http://xyz.com"


def test_redirect_url_is_not_recognized_without_include_redirect_urls():
    # https://github.com/brave/adblock-rust/blob/b7f29af8c0a0d000201d8d769b6a0b25a9dd4e89/src/blocker.rs#L1267
    filter_set2 = adblock.FilterSet()
    filter_set2.add_filter_list(
        "||foo.com$important,redirect-url=http://xyz.com", include_redirect_urls=False
    )

    engine2 = adblock.Engine(filter_set=filter_set2)

    res = engine2.check_network_urls("https://foo.com", "https://foo.com", "script")
    assert res.matched is False
    assert res.redirect is None
    assert res.redirect_type is None


def test_redirect_url_exception():
    # https://github.com/brave/adblock-rust/blob/b7f29af8c0a0d000201d8d769b6a0b25a9dd4e89/src/blocker.rs#L1314
    filter_set = adblock.FilterSet(debug=True)
    filter_set.add_filter_list(
        """
        ||imdb-video.media-imdb.com$media,redirect-url=http://xyz.com
        @@||imdb-video.media-imdb.com^$domain=imdb.com
        """,
        include_redirect_urls=True,
    )

    engine2 = adblock.Engine(filter_set=filter_set, optimize=False)

    res = engine2.check_network_urls(
        "https://imdb-video.media-imdb.com/kBOeI88k1o23eNAi",
        "https://www.imdb.com/video/13",
        "media",
    )
    assert res.matched is False
    assert res.redirect == "http://xyz.com"
    assert res.redirect_type == "url"
    assert res.exception == "@@||imdb-video.media-imdb.com^$domain=imdb.com"


def test_redirect_with_custom_resource():
    filters = adblock.FilterSet()
    filters.add_filter_list("-advertisement-$redirect=test\n")

    engine = adblock.Engine(filter_set=filters)
    engine.add_resource(
        name="test", content_type="application/javascript", content="YWxlcnQoMSk="
    )

    result = engine.check_network_urls(
        url="http://example.com/-advertisement-icon.",
        source_url="example.com",
        request_type="image",
    )

    assert result.matched
    assert not result.exception
    assert not result.important
    assert result.redirect == "data:application/javascript;base64,YWxlcnQoMSk="
