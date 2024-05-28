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

- [x] Prevent double event insertion
- [x] Prevent double booking insertion
- [x] Add CRUD operations
- [ ] Add tests
- [x] Fix event recursion when the current date is far from the event date
- [x] Exclude slots/events when from filter is after the event date
- [ ] Add a validation error struct to improve errors messages
- [x] Add overlapping event boolean field to the event struct
- [x] Validate strings max size
- [x] Add caching