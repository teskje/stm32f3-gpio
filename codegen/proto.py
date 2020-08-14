from copy import deepcopy

from peripheral import Access, Field, Peripheral, Register


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


def gpio_fields(name, description, width=1):
    fields = {}
    for i in range(16):
        field_name = f"{name}{i}"
        field_key = field_name.lower()
        fields[field_key] = Field(
            name=field_name,
            description=f"Pin {i} {description}",
            offset=i * width,
            width=width,
        )
    return fields


def bsrr_fields():
    fields = {}
    for i in range(16):
        fields[f"bs{i}"] = Field(
            name=f"BS{i}",
            description=f"Pin {i} set bit",
            offset=i,
        )
        fields[f"br{i}"] = Field(
            name=f"BR{i}",
            description=f"Pin {i} reset bit",
            offset=i + 16,
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
        ),
    ),
    "ospeedr": Register(
        name="OSPEEDR",
        description="GPIO port output speed register",
        address_offset=0x08,
        fields=gpio_fields(
            name="OSPEEDR",
            description="I/O output speed configuration bits",
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
        ),
    ),
    "odr": Register(
        name="UDR",
        description="GPIO port output data register",
        address_offset=0x14,
        fields=gpio_fields(
            name="ODR",
            description="output data bit",
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
            **gpio_fields(name="LCK", description="lock bit"),
            "lckk": Field(name="LCKK", description="Lock key", offset=16),
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
        )
    ),
}
