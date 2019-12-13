use lucene_query_builder::QueryBuilder;

#[derive(QueryBuilder)]
struct Person {
    #[query_builder_rename = "fullname"]
    name: String,
    age: i32,
    #[query_builder_ignore]
    ignored: String,
}

#[test]
fn should_serialize_simple_query() {
    let query = Person::query_builder()
        .name("Bob")
        .or()
        .name("Alice")
        .build();

    assert_eq!(query, "query=name:Bob OR name:Alice".to_string());
}

#[test]
fn should_serialize_nested_query() {
    let query = Person::query_builder()
        .expr(Person::query_builder().name("Bob").or().name("Alice"))
        .and()
        .age("22")
        .build();

    assert_eq!(
        query,
        "query=(name:Bob OR name:Alice) AND age:22".to_string()
    );
}

#[test]
fn should_serialize_proximity() {
    let query = Person::query_builder()
        .name("Bob")
        .and()
        .name("Alice")
        .proximity(4)
        .build();

    assert_eq!(query, "query=name:Bob AND name:Alice~4".to_string());
}

#[test]
fn should_serialize_range() {
    let query = Person::query_builder().name_range("Bob", "Baz").build();

    assert_eq!(query, "query=name:[Bob TO Baz]".to_string());

    let query = Person::query_builder().age_range("7", "77").build();

    assert_eq!(query, "query=age:[7 TO 77]".to_string());
}
