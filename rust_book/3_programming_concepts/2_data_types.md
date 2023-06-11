# Data Types

Rust is a statically typed language - so it must know all variables types at compile time. The compiler can usually infer the type we want based on the value. In cases we want to select one of many types, we must add type annotation. For example - we can declare a string of an integer and parse it. This could be an integer or a string - and we will need to use type annotation to specify the type.

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

An integer is a number without a fractional component. Integers are normally declared based on the bit length and if they're signed (i) or unsigned (u). Note that the "i" in the integer data types stands for "signed", while the "u" in the unsigned integer data types stands for "unsigned". The maximum and minimum values are calculated using two's complement representation. Signed and unsigned refer to whether it's possible for the number to be negative or not (signed/unsigned). Signed numbers are stored using [two's complements](https://en.wikipedia.org/wiki/Two%27s_complement) representation. 

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

There are many types of integers we can use. By default, integers use `i32`. Below, we list the different numerical ranges we can represent using varying integer types for Rust:

| Data Type | Size (bits) | Minimum Value (MIN)| Maximum Value (MAX) | Documentation |
| --- | --- | --- | --- | --- |
| `i8` | 8 | -128 | 127 | [i8 - Rust](https://doc.rust-lang.org/std/primitive.i8.html) |
| `u8` | 8 | 0 | 255 | [u8 - Rust](https://doc.rust-lang.org/std/primitive.u8.html) |
| `i16` | 16 | -32768 | 32767 | [i16 - Rust](https://doc.rust-lang.org/std/primitive.i16.html) |
| `u16` | 16 | 0 | 65535 | [u16 - Rust](https://doc.rust-lang.org/std/primitive.i16.html) |
| `i32` | 32 | -2147483648 | 2147483647 | [i32 - Rust](https://doc.rust-lang.org/std/primitive.i32.html) |
| `u32` | 32 | 0 | 4294967295 | [u32 - Rust](https://doc.rust-lang.org/std/primitive.u32.html) |
| `i64` | 64 | -9223372036854775808 | 9223372036854775807 | [i64 - Rust](https://doc.rust-lang.org/std/primitive.i64.html) |
| `u64` | 64 | 0 | 18446744073709551615 | [u64 - Rust](https://doc.rust-lang.org/std/primitive.u64.html) |
| `i128` | 128 | -170141183460469231731687303715884105728 | 170141183460469231731687303715884105727 | [i128 - Rust](https://doc.rust-lang.org/std/primitive.i128.html) |
| `u128` | 128 | 0 | 340282366920938463463374607431768211455 | [u128 - Rust](https://doc.rust-lang.org/std/primitive.u128.html) |
| `isize` | 32 or 64 (Architecture Based) | ^ | ^ | [isize - Rust](https://doc.rust-lang.org/std/primitive.isize.html) |
| `usize` | 32 or 64 (Architecture Based) | ^ | ^ | [usize - Rust](https://doc.rust-lang.org/std/primitive.usize.html) |

Integers can be declared as decimals, hexes, octals, binary or bytes. The symbol `_` can be used as a visual separator, and don't impact the numbers value, e.g `1_000=1000`. The type suffix can be used to declare numbers with their types explicitly, e.g `57u8`. See the table below for an example:

| Number literals | Example |
| --- | --- |
| Decimal | 98_222 | 
| Hex | 0xff | 
| Octal | 0o77 | 
| Binary | 0b1111_0000 | 
| Byte (u8 only) | b'A' |

#### Integer Overflows

A variable of type `u8` can store values between 0 and 255. If we try to set the variable as 256, which is above its value, it will overflow. This can result in one of two results:
1. debug mode: The program panics at runtime.
2. release mode: Program omits runtime checks. 

We can deal with overflows in many ways:
- `#[allow(arithmetic_overflow)]`: Disable compile time checks. In release mode, the operation will wrap around, and an error will happen in debug mode.
- Wrapping methods: Use the integer methods `.wrapping_*` to wrap around operations.
- Checking methods: Return `None` using the integer methods `checked_*`. 
- Overflowing methods: Return the value and a boolean indicating overflow with the integer methods `overflow_*`.
- Saturating methods: Return the maximum/minimum value and ignore the remainder with the integer methods `saturating_*`.

Observation: `*` can be any of the following values: `add`, `sub`, `mul`, `pow`, `neg`.

### Floating Point Types

Floating point numbers are also supported by Rust. Rust supports 32 (`f32`) and 64 bit (`f64`) bit floating point numbers. These are also called singles and doubles precision floats respectively. Rust defaults floats to 64 bits (Due to modern 64 bit architecture). Floating point numbers always have a sign. 



Floats can also be used to represent special values such as the following. We also list their constant keywords: 
- $-0$
- $\infty$ and $-\infty$ = `INFINITY` and `NEG_INFINITY`
- $NaN$ = `NAN`

See the docs for more information on [f32](https://doc.rust-lang.org/std/primitive.f32.html) and [f64](https://doc.rust-lang.org/std/primitive.f64.html) floats.

### Numerical Operations

Basic numeric operations supported by rust: 
- Addition: `+`
- Addition and attribution: `+=`
- Subtraction: `-`
- Subtraction and attribution: `-=`
- Multiplication: `*`
- Multiplication and attribution: `*=`
- Division: `/`
- Division and attribution: `/=`
- Remainder: `%`
- Remainder and attribution: `%=`
 
Obs: Integer division truncates down.

```rust
fn operations() {
    let mut x = 5;
    x = (5 + 1 - 2)*2/2
}
```

### Boolean Types

Rust supports booleans (defined by the keyword `bool`), which assume one of two values: `true` or `false`. The following operations are supported for boolean types:
- Bitwise complement (not): `!`
- Bitwise AND: `&`
- Short-circuit AND: `&&`
- Bitwise AND and attribution: `&=`
- Bitwise XOR: `^`
- Bitwise XOR and attribution: `^=`
- Bitwise OR: `|`
- Bitwise OR and attribution: `|=` 
- Short-circuit OR: `||`
- Comparisons: `==`, `!=`, ...


Obs: Short-circuiting operations only evaluate the second argument if the first argument doesn't already satisfy the condition.

```rust
fn boolean_ops(){
    let x:bool = true;
    let y:bool = false;

    println!("{}", x && y); // false
    println!("{}", x || y); // true
    println!("{}", x ^ y); // true
    println!("{}", !x); // false
}
```

### Character Types

Primitive alphabetic type. Char values can be declared between single quotes. String values use double quotes. `char` types can represent Unicode scalar values, so it supports accents, Chinese, emojis, etc.

```rust
fn char_dec(){
    let c = 'z';
    let emj = '😻';
}
```

## Compound types

Groups multiple values into one type. Rust supports two primitive compound types: tuples and arrays.

### Tuples

For grouping a variety of types into one compound type. Tuples have a fixed length. Once they're declared, their size can't change. Tuples are declared by writing a comma-separated list of values inside a parenthesis. 

```rust
fn main(){
    let tup = (500, 6.4, 1);
}
```

When using the variable `tup`, all of the elements of the tuple will be used. To extract individual elements, the tuple can be destructured with pattern matching as follows.

```rust
fn main(){
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("{}", x)
}
```

A tuples element can also be accessed by using the `.` operator followed by the index of the value.

```rust
fn main(){
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("{}", tup.0)
}
```

There is a special type of tuple, without any values, called the `unit` tuple. Unit values are implicitly returned by expressions that don't return any other value. It's defined by empty parenthesis, `()`.

### Array Type

Arrays are another way of storing multiple values. They differ to tuples since every element of an array must be the same type. They also have a fixed length.

Arrays differ to vectors which can grow or shrink in size. Arrays are useful for when we know the number of elements will not change.

Arrays can be declared by using square brackets with a comma separation. When declaring the type, we can provided the type of the array using the following notation: `[data_type; array_length]`.

```rust
fn main(){
    let x: [i32; 5] = [1,2,3,4,5];
}
```

An array can also be declared to contain the same constant for the entire length using a semicolon:

```rust
fn main(){
    let x = [0; 5];
}
```

The elements of an array can be accessed by using brackets followed by the index.

```rust
fn main(){
    let x: [i32; 5] = [1,2,3,4,5];
    println!("{}", x[0]);
}
```

Rusts memory safety will prevent us from accessing indexes larger than the array at compile time and run time by generating an error.

```rust
fn main(){
    let x: [i32; 5] = [1,2,3,4,5];
    println!("{}", x[6]);
}
```
