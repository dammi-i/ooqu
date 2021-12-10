# ooqu
a custom vm for a custom assembly-like language

# Registers
- `ra`
- `rb`
- `rc`
- `rd`
- `re`
- `rf`
- `rj`
- `rk`

# Instructions
- `sto dest (register), src0 (register/imm/mem)` stores the value from src0 to dest
- `lod dest (register), src0 (register)` load the address of src0 and store it in dest
- `add dest (register), src0 (register/imm/mem), src1 (register/imm/mem)`
- `sub dest (register), src0 (register/imm/mem), src1 (register/imm/mem)`
- `mul dest (register), src0 (register/imm/mem), src1 (register/imm/mem)`
- `div dest (register), src0 (register/imm/mem), src1 (register/imm/mem)`
- `nop`
- `exec` interrupts
- `(register/mem)*` fetches the value from the address

# Rust example
```rs
use ooqu::ooqu;
fn main() {
    let a = 1u8;
    let b = 1u8;
    let out: u8;
    ooqu!(
        "add {out}, {a}, {b}"
    );
    println!("{} + {} = {}", a, b, o);
}
```

# Exec table
`ra` should hold the value of the target interrupt code to call and the rest of the registers are passed as arguments to that call.

ID   |   Name   | ra   | rb  |
-----|----------|------|-----|
0    | exit     | 0    | code|
1    | write    | 1    | fd  |
2    | input    | 2    | n/a |

# Example
```
static
    str: "hello, world!"
    len: 15
end

sto ra, 1 // ID for write interrupt 
sto rb, 1 // stdout
lod rc, str // address of the string
lod rd, len // len of bytes to write
exec // execute the interrupt
```
