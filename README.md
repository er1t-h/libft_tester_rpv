# libft_rpv

A quick and efficient tester for your libft using Cargo.

## Usage:

To launch all mandatory tests:
```sh
cargo test
```

To launch all tests:
```sh
cargo test --features bonus
```

To launch test in verbose mode:
```sh
cargo test --features verbose
```

To launch only the test of functions containing `pattern`:
```sh
cargo test `pattern`
```

## Memory leaks check:
When running a `cargo test`, a file will be generated in `target/debug/deps/libft_rpv-` followed by a random hexadecimal string. Locate it, and use:
```sh
valgrind --show-leak-kinds=all --leak-check=full target/debug/deps/<filename>
```
The executable generated will have a 16 bytes still reachable. Every cargo test, as simple as it may be, will have it. So just ignore it. But if there's any more byte leaked, then your program is leaking, which should result in instant `Leaks` flag.

## What is not tested by this program
- Norm (As a correcter, be wary, some people found ways of bypass the norminette's check)
- Malloc protection (if a malloc is not protected, it's a potential crash, so it should be a 0)
- Leaks in case of malloc fail. Be sure to check it yourself, particularly in functions that require lots of allocation, like `ft_split`

## Side note
You already heard it thousands of time, but make sure to create your own test suite. This test suite should only be used for correction purpose. I know that creating tests can get boring, but you'll need it at some point, so learning it now will only benefit you.
