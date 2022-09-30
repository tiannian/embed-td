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
If you want to build from source, use this feature:

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
