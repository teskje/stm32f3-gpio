from copy import deepcopy

from peripheral import Access, Field, Peripheral, Register, Usage, Values


def gpio_prototypes():
    peripherals = {}
    base_address = 0x4800_0000
    for port in "ABCDEFGH":
        peripherals[port] = Peripheral(
            name=f"GPIO{port}",
            description=f"General-purpose I/Os, port {port}",
            base_address=base_address,
            registers=deepcopy(REGISTERS),
        )
        base_address += 0x400

    gpioa = peripherals["A"]
    gpioa.registers["moder"].reset_value = 0xA800_0000
    gpioa.registers["ospeedr"].reset_value = 0x0C00_0000
    gpioa.registers["pupdr"].reset_value = 0x6400_0000

    gpiob = peripherals["B"]
    gpiob.registers["moder"].reset_value = 0x0000_0280
    gpiob.registers["ospeedr"].reset_value = 0x0000_00C0
    gpiob.registers["pupdr"].reset_value = 0x0000_0100

    return peripherals


def gpio_fields(name, description, values, width=1):
    fields = {}
    for i in range(16):
        field_name = f"{name}{i}"
        field_key = field_name.lower()
        fields[field_key] = Field(
            name=field_name,
            description=f"Pin {i} {description}",
            offset=i * width,
            width=width,
            values=values,
        )
    return fields


def bsrr_fields():
    fields = {}
    for i in range(16):
        fields[f"bs{i}"] = Field(
            name=f"BS{i}",
            description=f"Pin {i} set bit",
            offset=i,
            values=Values(
                usage=Usage.write,
                values={
                    0b0: ("NoAction", "No action on the OD bit"),
                    0b1: ("Set", "Set the OD bit"),
                },
            ),
        )
        fields[f"br{i}"] = Field(
            name=f"BR{i}",
            description=f"Pin {i} reset bit",
            offset=i + 16,
            values=Values(
                usage=Usage.write,
                values={
                    0b0: ("NoAction", "No action on the OD bit"),
                    0b1: ("Reset", "Reset the OD bit"),
                },
            ),
        )
    return fields


def afr_fields(start):
    fields = {}
    for i in range(start, start + 8):
        fields[f"afr{i}"] = Field(
            name=f"AFR{i}",
            description=f"Pin {i} alternate function selection bits",
            offset=i * 4,
            width=4,
            values=Values(
                values={
                    0b0000: ("AF0", "Alternate function 0"),
                    0b0001: ("AF1", "Alternate function 1"),
                    0b0010: ("AF2", "Alternate function 2"),
                    0b0011: ("AF3", "Alternate function 3"),
                    0b0100: ("AF4", "Alternate function 4"),
                    0b0101: ("AF5", "Alternate function 5"),
                    0b0110: ("AF6", "Alternate function 6"),
                    0b0111: ("AF7", "Alternate function 7"),
                    0b1000: ("AF8", "Alternate function 8"),
                    0b1001: ("AF9", "Alternate function 9"),
                    0b1010: ("AF10", "Alternate function 10"),
                    0b1011: ("AF11", "Alternate function 11"),
                    0b1100: ("AF12", "Alternate function 12"),
                    0b1101: ("AF13", "Alternate function 13"),
                    0b1110: ("AF14", "Alternate function 14"),
                    0b1111: ("AF15", "Alternate function 15"),
                },
            ),
        )
    return fields


REGISTERS = {
    "moder": Register(
        name="MODER",
        description="GPIO port mode register",
        address_offset=0x00,
        fields=gpio_fields(
            name="MODER",
            description="I/O mode configuration bits",
            values=Values(
                values={
                    0b00: ("Input", "Input mode"),
                    0b01: ("Output", "General-purpose output mode"),
                    0b10: ("Alternate", "Alternate function mode"),
                    0b11: ("Analog", "Analog mode"),
                },
            ),
            width=2,
        ),
    ),
    "otyper": Register(
        name="OTYPER",
        description="GPIO port output type register",
        address_offset=0x04,
        fields=gpio_fields(
            name="OT",
            description="I/O output type configuration bit",
            values=Values(
                values={
                    0b0: ("PushPull", "Output push-pull"),
                    0b1: ("OpenDrain", "Output open-drain"),
                },
            ),
        ),
    ),
    "ospeedr": Register(
        name="OSPEEDR",
        description="GPIO port output speed register",
        address_offset=0x08,
        fields=gpio_fields(
            name="OSPEEDR",
            description="I/O output speed configuration bits",
            values=Values(
                values={
                    0b00: ("LowSpeed", "Low speed"),
                    0b01: ("MediumSpeed", "Medium speed"),
                    0b11: ("HighSpeed", "High speed"),
                },
            ),
            width=2,
        ),
    ),
    "pupdr": Register(
        name="PUPDR",
        description="GPIO port pull-up/pull-down register",
        address_offset=0x0C,
        fields=gpio_fields(
            name="PUPDR",
            description="I/O pull-up or pull-down configuration bits",
            values=Values(
                values={
                    0b00: ("Floating", "No pull-up, pull-down"),
                    0b01: ("PullUp", "Pull-up"),
                    0b10: ("PullDown", "Pull-down"),
                },
            ),
            width=2,
        ),
    ),
    "idr": Register(
        name="IDR",
        description="GPIO port input data register",
        address_offset=0x10,
        access=Access.read_only,
        fields=gpio_fields(
            name="IDR",
            description="input data bit",
            values=Values(
                usage=Usage.read,
                values={
                    0b0: ("Low", "Input is logic low"),
                    0b1: ("High", "Input is logic high"),
                },
            ),
        ),
    ),
    "odr": Register(
        name="UDR",
        description="GPIO port output data register",
        address_offset=0x14,
        fields=gpio_fields(
            name="ODR",
            description="output data bit",
            values=Values(
                values={
                    0b0: ("Low", "Set output to logic low"),
                    0b1: ("High", "Set output to logic high"),
                },
            ),
        ),
    ),
    "bsrr": Register(
        name="BSRR",
        description="GPIO port bit set/reset register",
        address_offset=0x18,
        access=Access.write_only,
        fields=bsrr_fields(),
    ),
    "lckr": Register(
        name="LCKR",
        description="GPIO port configuration lock register",
        address_offset=0x1C,
        fields={
            **gpio_fields(
                name="LCK",
                description="lock bit",
                values=Values(
                    values={
                        0b0: ("Unlocked", "Port configuration is not locked"),
                        0b1: ("Locked", "Port configuration is locked"),
                    },
                ),
            ),
            "lckk": Field(
                name="LCKK",
                description="Lock key",
                offset=16,
                values=Values(
                    values={
                        0b0: ("NotActive", "Port configuration lock is not active"),
                        0b1: ("Active", "Port configuration lock is active"),
                    },
                ),
            ),
        },
    ),
    "afrl": Register(
        name="AFRL",
        description="GPIO alternate function low register",
        address_offset=0x20,
        fields=afr_fields(start=0),
    ),
    "afrh": Register(
        name="AFRH",
        description="GPIO alternate function high register",
        address_offset=0x24,
        fields=afr_fields(start=8),
    ),
    "brr": Register(
        name="BRR",
        description="GPIO port bit reset register",
        address_offset=0x28,
        access=Access.write_only,
        fields=gpio_fields(
            name="BR",
            description="reset bit",
            values=Values(
                usage=Usage.write,
                values={
                    0b0: ("NoAction", "No action on the OD bit"),
                    0b1: ("Reset", "Reset the OD bit"),
                },
            ),
        )
    ),
}
