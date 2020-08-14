import xml.etree.ElementTree as etree
from xml.etree.ElementTree import Element, SubElement


def gen_svd(name, peripherals):
    device = Element("device")

    SubElement(device, "name").text = name

    peripherals_elem = SubElement(device, "peripherals")
    gen_peripherals(peripherals, peripherals_elem)

    return etree.tostring(device, encoding="unicode")


def gen_peripherals(peripherals, root):
    for peripheral in peripherals.values():
        peripheral_elem = SubElement(root, "peripheral")
        gen_peripheral(peripheral, peripheral_elem)


def gen_peripheral(peripheral, root):
    SubElement(root, "name").text = peripheral.name

    SubElement(root, "description").text = peripheral.description
    SubElement(root, "baseAddress").text = hex(peripheral.base_address)

    address_block_elem = SubElement(root, "addressBlock")
    gen_address_block(address_block_elem)

    registers_elem = SubElement(root, "registers")
    gen_registers(peripheral.registers, registers_elem)


def gen_address_block(root):
    SubElement(root, "offset").text = "0x0"
    SubElement(root, "size").text = "0x400"
    SubElement(root, "usage").text = "registers"


def gen_registers(registers, root):
    for register in registers.values():
        register_elem = SubElement(root, "register")
        gen_register(register, register_elem)


def gen_register(register, root):
    SubElement(root, "name").text = register.name
    SubElement(root, "description").text = register.description
    SubElement(root, "addressOffset").text = hex(register.address_offset)
    SubElement(root, "size").text = "0x20"
    SubElement(root, "access").text = register.access.value
    SubElement(root, "resetValue").text = hex(register.reset_value)

    fields_elem = SubElement(root, "fields")
    gen_fields(register.fields, fields_elem)


def gen_fields(fields, root):
    for field in fields.values():
        field_elem = SubElement(root, "field")
        gen_field(field, field_elem)


def gen_field(field, root):
    SubElement(root, "name").text = field.name
    SubElement(root, "description").text = field.description
    SubElement(root, "bitOffset").text = hex(field.offset)
    SubElement(root, "bitWidth").text = hex(field.width)
