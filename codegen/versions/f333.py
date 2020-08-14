from codegen import proto


def make():
    gpio = proto.gpio_prototypes()

    del gpio["E"]
    del gpio["G"]
    del gpio["H"]

    return gpio
