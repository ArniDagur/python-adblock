import adblock

def test_engine_arguments():
    # None of these should panic
    adblock.Engine()
    adblock.Engine([])
    adblock.Engine(network_filters=None)
    adblock.Engine(network_filters=[])
    adblock.Engine(load_network=False, load_cosmetic=True, debug=False)
    adblock.Engine(debug=True)
