# edge cases of rust

casting i32 to usize?

In Rust, the code snippet let yo = (-1 as i32) as usize; involves two type casts:

(-1 as i32) – This converts the literal -1 to a 32-bit signed integer (i32). The value of -1 remains the same.
(-1 as i32) as usize – This cast converts the signed integer -1 to an unsigned integer of type usize.
The key issue is that usize is an unsigned type, which means it cannot represent negative values. When a negative value like -1 is cast to an unsigned type, Rust doesn't raise an error; instead, it performs two's complement interpretation. This means that the bit pattern representing -1 in i32 is reinterpreted as a positive number in usize.

For example:

In a 32-bit system, usize is 32 bits, and the bit representation of -1 in i32 is 0xFFFFFFFF. When this is reinterpreted as a usize, it becomes the unsigned integer 4294967295 (the maximum value for a 32-bit unsigned integer).
In a 64-bit system, usize is 64 bits, and -1 (as i32) will be first extended to 0xFFFFFFFFFFFFFFFF in 64 bits, which is 18446744073709551615 (the maximum value for a 64-bit unsigned integer).
This explains why the value is not -1, but instead the maximum value for an unsigned integer depending on the platform.

If you're on a 64-bit platform, casting -1 as usize would give you 18446744073709551615. On a 32-bit platform, it would be 4294967295.
