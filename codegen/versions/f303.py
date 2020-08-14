from codegen import proto


def make():
    gpio = proto.gpio_prototypes()

    del gpio["C"].registers["lckr"]
    del gpio["E"].registers["lckr"]
    del gpio["F"].registers["lckr"]
    del gpio["F"].registers["moder"].fields["moder5"]

    del gpio["G"]
    del gpio["H"]

    return gpio
