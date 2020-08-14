from enum import Enum
from dataclasses import dataclass, field
from typing import Dict


class Access(Enum):
    read_write = "read-write"
    read_only = "read-only"
    write_only = "write-only"


@dataclass
class Field:
    name: str
    description: str
    offset: int
    width: int = 1


@dataclass
class Register:
    name: str
    description: str
    address_offset: int
    access: Access = Access.read_write
    reset_value: int = 0x0000_0000
    fields: Dict[str, Field] = field(default_factory=dict)


@dataclass
class Peripheral:
    name: str
    description: str
    base_address: int
    registers: Dict[str, Register]
