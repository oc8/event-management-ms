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
- [ ] Prevent double slot insertion
- [ ] Prevent double booking insertion
- [ ] Prevent double closing exception insertion
- [ ] Add CRUD operations
- [ ] Add tests
- [ ] Fix event recursion when the current date is far from the event date
