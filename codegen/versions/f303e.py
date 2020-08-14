from codegen import proto


def make():
    gpio = proto.gpio_prototypes()
    return gpio
