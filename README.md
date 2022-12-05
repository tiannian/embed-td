# embedded-td
Embed tendermint into rust to use tendermint as a crate.

## Usage

Add this line into `Cargo.toml`:

```toml
embedded-td = "0.1"
```

### Special tendermint version

Default tendermint version is `0.34`.

You can use feature to use different tendermint version.

```toml
embedded-td = { version = "0.1", features = ["td_ver_0_34"] }
```

### Build from source

By deafult, this crate use precompile version on github.
If you want to build from source, use `use_source_code`:

```toml
embedded-td = { version = "0.1", features = ["use_source_code"] }
```

This feature can work with version.

```toml
embedded-td = { version = "0.1", features = ["use_source_code", "td_ver_0_34"] }
```

Note: Build from source need `go` installed.

Building from source can also use non-goleveldb backends:
```toml
# Use cleveldb, please install `libleveldb`.
embedded-td = { version = "0.1", features = ["use_source_code", "backend_cleveldb"] }

# Use rocksdb, please install `librocksdb`.
embedded-td = { version = "0.1", features = ["use_source_code", "backend_rocksdb"] }
```

### Fork tendermint

If you fork tendermint, please use `custom-upstream` feature.

Please set environment `EMBEDDED_TD_UPSTREAM_URL` as fork version's url.

For example:

```shell
# source code url, format is .tar.gz
EMBEDDED_TD_UPSTREAM_URL = "http://example.com/tendermint"
```

## Features

You can use these features:

- Version of tendermint
    - `td_ver_0_34`(default) tendermint 0.34
    - `td_ver_0_37` tendermint 0.37
- Runtime of async
    - `smol-backend`(default).
    - `tokio-backend`
- How to get tendermint?
    - `use_precompile_binary`(default)
    - `use_source_code`
- Use fork version of tendermint, `custom-upstream`.

## Supported platfrom

1. linux, amd64
2. linux, arm64
3. linux, armv6
4. macos, amd64
5. macos, arm64
6. windows, amd64
7. windows, arm64
8. windows, armv6
9. other target (only build from source)

1 ~ 8 can work with precompile binary. Beacuse origin tendermin repo only provide these release.
If you want to running on other platform, please configure go cross compile.

### Test platform

1. linux, x86_64
2. windows, x86_64
3. windows, aarch64
4. android(termux), aarch64

