# booking-ms

# CLIs

## Buf

https://buf.build/docs/installation

```sh
cargo install protoc-gen-prost-crate
```

### Generation

```sh
make protos
```
or
```sh
buf generate
```

## TODO

- [ ] Prevent double event insertion
- [x] Prevent double booking insertion
- [ ] Prevent double closing exception insertion
- [ ] Add CRUD operations
- [ ] Add tests
- [ ] Fix event recursion when the current date is far from the event date
- [ ] Return the datetime converted with the timezone and add a filter params to manually set the timezone for conversion
