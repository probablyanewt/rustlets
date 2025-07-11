# std_ex

A crate I use to extend the std library.

To alias this crate as std, add the following to the `Cargo.toml`

```
[dependencies]
std = { package = "std_ex", git = "https://github.com/probablyanewt/rustlets" }
```

## Extensions

- `env`
  - `fn var_exists` tests if env var exists
- `fmt`
  - `fn pretty` formats any type which implements `Debug` as a pretty string
- `time`
  - Chrono crate
