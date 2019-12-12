# Lucene Query Builder Derive

A procmacro derive crate to generate lucene query builder for Rust structs :

## Example :


```rust
use lucene_query_builder::QueryBuilder;

#[derive(QueryBuilder)]
struct Person {
    name: String,
    age: i32,
}
```

### Simple query

```rust
    let query = Person::query_builder()
        .name("Bob")
        .or()
        .name("Alice")
        .build();

    assert_eq!(query, "query=name:Bob OR name:Alice".to_string());
```


### Nested query

```rust
    let query = Person::query_builder()
        .expr(Person::query_builder().name("Bob").or().name("Alice"))
        .and()
        .age("22")
        .build();

    assert_eq!(
        query,
        "query=(name:Bob OR name:Alice) AND age:22".to_string()
    );
```

### Range query

```rust
    let query = Person::query_builder().age_range("7", "77").build();

    assert_eq!(query, "query=age:[7 TO 77]".to_string());
```