use lucene_query_builder::QueryBuilder;

#[derive(QueryBuilder)]
struct Person {
    #[query_builder_field = "patronymic"]
    #[query_builder_rename = "last_name"]
    name: String,
    age: i32,
    #[query_builder_ignore]
    ignored: String,
    #[query_builder_rename = "fullname"]
    complete_name: String,
}

#[test]
fn should_serialize_simple_query() {
    let query = Person::query_builder()
        .last_name("Bob")
        .or()
        .last_name("Alice")
        .build();

    assert_eq!(
        query,
        "query=patronymic:Bob OR patronymic:Alice".to_string()
    );
}

#[test]
fn should_quote_values_with_space() {
    let query = Person::query_builder().last_name("Uncle Bob").build();

    assert_eq!(query, "query=patronymic:\"Uncle Bob\"".to_string());
}

#[test]
fn should_serialize_nested_query() {
    let query = Person::query_builder()
        .expr(
            Person::query_builder()
                .last_name("Bob")
                .or()
                .last_name("Alice"),
        )
        .and()
        .age("22")
        .build();

    assert_eq!(
        query,
        "query=(patronymic:Bob OR patronymic:Alice) AND age:22".to_string()
    );
}

#[test]
fn should_serialize_proximity() {
    let query = Person::query_builder()
        .last_name("Bob")
        .and()
        .last_name("Alice")
        .proximity(4)
        .build();

    assert_eq!(
        query,
        "query=patronymic:Bob AND patronymic:Alice~4".to_string()
    );
}

#[test]
fn should_gen_renamed_method() {
    let query = Person::query_builder()
        .last_name("Bob")
        .and()
        .fullname("Bob Marley")
        .build();

    assert_eq!(
        query,
        "query=patronymic:Bob AND complete_name:\"Bob Marley\"".to_string()
    );
}

#[test]
fn should_serialize_range() {
    let query = Person::query_builder()
        .last_name_range("Bob", "Baz")
        .build();

    assert_eq!(query, "query=patronymic:[Bob TO Baz]".to_string());

    let query = Person::query_builder().age_range("7", "77").build();

    assert_eq!(query, "query=age:[7 TO 77]".to_string());
}

#[derive(QueryBuilder)]
struct PersonWithoutAnnotation {
    name: String,
    age: i32,
}

#[test]
fn should_serialize_simple_query_on_struct_with_no_annotations() {
    let query = PersonWithoutAnnotation::query_builder()
        .name("Bob")
        .or()
        .name("Alice")
        .build();

    assert_eq!(query, "query=name:Bob OR name:Alice".to_string());
}
