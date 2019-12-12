# Lucene Query Builder Derive

WIP

A proc macro derive crate to generate lucene query builder for Rust structs :

## Example :


```rust
use lucene_query_builder::QueryBuilder;

#[derive(QueryBuilder)]
struct Person {
    name: String,
    age: i32,
}

fn main() {

    let query = Person::query_builder()
        .expr(Person::query_builder()
            .name("Bob")
            .or()
            .name("Alice")
        ).and()
        .age("22")
        .build();
    
        assert_eq!(
            query,
            "query=(name:Bob OR name:Alice) AND age:22".to_string()
         );
}

```