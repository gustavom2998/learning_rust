# Data Types

Rust is statically typed language - so it must know all variables types at compile time. The compiler can usually infer th etype we want based on the value. In cases we want to select one of many types, we must add type annotation. For example - we can declare a string of an integer and parse it. This could be an integer or a string - and we will need to use type annotation to specify the type.

```rust
fn main(){

    // Success - unsigned int containing the value 42
    let val: u32 = "42".parse().expect("Not a number!");

    // error: type annotations needed
    let val = "42".parse().expect("Not a number!");
}

```

Every value in Rust has a data type. This tells Rust what kind of data is being specified. Data types can be divided into two sets: scalar and compound. 

## Scalar Types

Scalar types represent a single value. Rust has four primary scalar types:
- Integers
- Floating-point numbers
- Booleans
- Characters

### Integer types

An integer is a number without a fractional component. Integers are normally declared based on the bit length and if theyare signed (i) or unsigned (u). Note that the "i" in the integer data types stands for "signed", while the "u" in the unsigned integer data types stands for "unsigned". The maximum and minimum values are calculated using two's complement representation. Signed and unsigned refer to whether it's possible for the number to be negative or not (signed/unsigned). Signed numbers are stored using [two's complements](https://en.wikipedia.org/wiki/Two%27s_complement) representation. 

Signed numbers can represent numbers based on the following formula for the number of bits:
```
min(b) = -2^(b-1)
max(b) = 2^(b-1)-1
```

Unsigned numbers can represent numbers based on the following formula for the number of bits:
```
min(b) = 0
max(b) = 2^(b)-1
```

| Data Type | Size (bits) | Minimum Value | Maximum Value |
| --- | --- | --- | --- |
| `i8` | 8 | -128 | 127 |
| `u8` | 8 | 0 | 255 |
| `i16` | 16 | -32768 | 32767 |
| `u16` | 16 | 0 | 65535 |
| `i32` | 32 | -2147483648 | 2147483647 |
| `u32` | 32 | 0 | 4294967295 |
| `i64` | 64 | -9223372036854775808 | 9223372036854775807 |
| `u64` | 64 | 0 | 18446744073709551615 |
| `i128` | 128 | -170141183460469231731687303715884105728 | 170141183460469231731687303715884105727 |
| `u128` | 128 | 0 | 340282366920938463463374607431768211455 |
| `isize` | 32 or 64 (Architecture Based) | ^ | ^ |
| `usize` | 32 or 64 (Architecture Based) | ^ | ^ |

Integers can be declared as decimals, hexes, octals, binary or bytes. The symbol `_` can be used as visual separator, and don't impact the numbers value, e.g `1_000=1000`. The type suffix  can be used to declare numbers with their types explicitly, for example as `57u8`. See the table below for an example:

| Number literals | Example |
| --- | --- |
| Decimal | 98_222 | 
| Hex | 0xff | 
| Octal | 0o77 | 
| Binary | 0b1111_0000 | 
| Byte (u8 only) | b'A' | 

There are many types of integers we can use. By default, integers use i32.

#### Integer Overflows

A variable of type `u8` can stored values between 0 and 255. If we try to set the variable as 256, which is above it's value, it will overflow. This can result in one of two results:
1. debug mode: The program panics at runtime.
2. release mode: Program omits runtime checks. 

We can deal with overflows in many ways:
- `#[allow(arithmetic_overflow)]`: Disable compile time checks. In release mode, the operation will wrap around, and an error will happen in debug mode.
- Wrapping methods: Use the integer methods `.wrapping_*` to wrap around operations.
- Cheking methods: Return `None` using the integer methods `checked_*`. 
- Overflowing methods: Return the value and a boolean indicating overflow with the integer methods `overflow_*`.
- Saturating methods: Return the maximum/minimum value and ignore the remainder with the integer methods `saturating_*`.

Observation: `*` can be any of the following values: `add`, `sub`, `mul`, `pow`, `neg`.

### Floating Point Tyoes