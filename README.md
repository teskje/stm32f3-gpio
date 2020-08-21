# stm32f3-gpio

Proof-of-concept for a GPIO peripheral access crate (PAC, a crate that allows
low-level access to device registers) for the
[STM32F3 family of MCUs](https://www.st.com/en/microcontrollers-microprocessors/stm32f3-series.html).

## GPIO IP Versions

Each STM32F3 device includes one of five different GPIO internal peripheral
(IP) versions. To my knowledge the only way to extract these is by inspecting
the database included in the
[STM32CubeMX](https://www.st.com/en/development-tools/stm32cubemx.html) tool.
For more information on this see
[the cube-parse project](https://github.com/dbrgn/cube-parse).

This is a mapping from STM32F3 sub-families to GPIO IP versions:

| MCU families    | GPIO IP version |
| --------------- | --------------- |
| STM32F301       | F302            |
| STM32F302x{6,8} | F302            |
| STM32F302x{B,C} | F303            |
| STM32F302x{D,E} | F303e           |
| STM32F303x{6,8} | F333            |
| STM32F303x{B,C} | F303            |
| STM32F303x{D,E} | F303e           |
| STM32F318       | F302            |
| STM32F328       | F333            |
| STM32F334       | F333            |
| STM32F358       | F303            |
| STM32F373       | F373            |
| STM32F378       | F373            |
| STM32F398       | F303e           |

Note that the GPIO IP versions don't always correlate with the MCU families.
For example, STM32F303x6 devices include the GPIO version F333.

Also note that the GPIO IP versions vary even between similar MCUs. For example,
between the STM32F303 MCUs, 3 different GPIO IPs are used. Unfortunately, ST
only provides a single SVD for all STM32F303 devices, which means PACs based
on this SVD cannot be accurate for all sub-families. The same is true for
other STM32 families.

## Code Generation

This PoC tries to solve the mentioned problem with SVD-based GPIO PACs by making
use of hand-written definitions of the relevant registers. While the STM32CubeMX
database supplies us with information about available ports and pins for each of
the GPIO IPs, it doesn't include information about low-level register access.
This information is manually pulled from the respective reference manuals
instead.

The register definitions are defined in Python code as part of the `codegen`
package. Using Python code to describe the GPIO registers makes it easy to
get rid of almost all duplication, which is essential to make the manual work
required bearable. The general approach is to provide a prototypal definition
of all GPIO registers and ports (see [codegen/proto.py](codegen/proto.py)).
Then for the actual GPIO IP definitions we simply copy that prototype and
modify it as necessary, usually be removing superfluous GPIO ports (see
[codegen/versions.py](codegen/versions.py)). This approach makes writing the
register definitions quite manageable.

The rest of the `codegen` package is responsible for translating the register
definitions into SVD files and passing those to
[`svd2rust`](https://github.com/rust-embedded/svd2rust) to generate the PAC
code.

To re-generate the PAC code, simply execute the `codegen` package:

```
$ python3 ./codegen
```

## Usage

You can use this crate mostly like any other PAC generated with svd2rust. The
only limitation is that the `Peripherals` type does not provide a `take`
method, so you will have to use the unsafe `steal` method instead to get an
instance of the peripherals:

```rust
// The 'f303' feature must be enabled.
use stm32f3_gpio::f303 as gpio_pac;

let gp = unsafe { gpio_pac::Peripherals::steal() };
```

As an example, check out this branch, where the `stm32f3xx-hal` was ported to
use `stm32f3-gpio` for low-level GPIO register access:
https://github.com/ra-kete/stm32f3xx-hal/tree/gpio-pac.
