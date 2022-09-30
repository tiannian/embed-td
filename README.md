# embeded-td
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
If you want to build from source, use `build_from_source`:

```toml
embedded-td = { version = "0.1", features = ["build_from_source"] }
```

This feature can work with version.

```toml
embedded-td = { version = "0.1", features = ["build_from_source", "td_ver_0_34"] }
```

Note: Build from source need go installed.

Building from source can also use non-goleveldb backends:

```toml
# Use cleveldb.
embedded-td = { version = "0.1", features = ["build_from_source", "backend_cleveldb"] }

# Use rocksdb, please install needed software.
embedded-td = { version = "0.1", features = ["build_from_source", "backend_rocksdb"] }
```

### Fork tendermint

If you fork tendermint, please use `custom-upstream`.

Please set environment `EMBEDDED_TD_UPSTREAM_URL` as fork version's url.

For example:

```shell
# precompile binary url, http or https.
EMBEDDED_TD_UPSTREAM_URL = "http://example.com/tendermint"

# precompile binary url, format is .tar.gz
EMBEDDED_TD_UPSTREAM_URL = "http://example.com/tendermint"
```


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
