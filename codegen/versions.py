from codegen import proto


def make_definitions():
    yield "f302", make_f302()
    yield "f303", make_f303()
    yield "f303e", make_f303e()
    yield "f333", make_f333()
    yield "f373", make_f373()


def make_f302():
    gpio = proto.gpio_prototypes()

    del gpio["E"]
    del gpio["G"]
    del gpio["H"]

    return gpio


def make_f303():
    gpio = proto.gpio_prototypes()

    del gpio["C"].registers["lckr"]
    del gpio["E"].registers["lckr"]
    del gpio["F"].registers["lckr"]
    del gpio["F"].registers["moder"].fields["moder5"]

    del gpio["G"]
    del gpio["H"]

    return gpio


def make_f303e():
    gpio = proto.gpio_prototypes()
    return gpio


def make_f333():
    gpio = proto.gpio_prototypes()

    del gpio["E"]
    del gpio["G"]
    del gpio["H"]

    return gpio


def make_f373():
    gpio = proto.gpio_prototypes()

    del gpio["C"].registers["lckr"]
    del gpio["E"].registers["lckr"]
    del gpio["F"].registers["lckr"]

    del gpio["G"]
    del gpio["H"]

    return gpio
