# sample_deserialize_with_serde
Deserialization samples with [serde](https://serde.rs).

## Environment
- Rust  
  1.54.0
- serde  
  1.0
- serde_json  
  1.0
- serde_with  
  1.10.0

## Run
```sh
cargo run
```

## Test
```sh
cargo test
```

## Modules
In these samples, JSON like this will be deserialized into `Person` struct.

```json
{
    "name": "Alice",
    "age": 42,
    "is_active": true,
    "mails": [
        "alice@example.com",
        "wonderland@example.com"
    ],
    "company": "ABC technologies"
}
```

### person1
The simplest deserialization.

**All of `Person`'s fields are primitive types, not structs.**  
We don't need to write down any custom deserializer in this case.

### person2
Custom deserialization with implementation of `Visitor` and `Deserialize`.

**All of `Person`'s fields are structs and the same applies hereafter.**

### person3
Almost same as `person2` except `Option<Company>` is replaced by `MaybeCompany`.  
This deserialization does not work for `null` value now.

### person4
The fields of `Person` are all tuple structs.

### person5
Deserialization with serde's attribute `deserialize_with`.

Using this attribute, deserialization of `Vec<T>` or `Option<T>` would be difficult a bit.

### person6
A little improvement of `person5` with [serde_with](https://docs.rs/serde_with/).