use connector_agent::data_sources::{
    postgres::{PostgresDataSource, PostgresDataSourceBuilder},
    DataSource, Produce,
};
use connector_agent::{SourceBuilder};
use connector_agent::{DataType, Dispatcher};
use ndarray::array;
use connector_agent::writers::mixed::MemoryWriter;

#[test]
#[should_panic]
fn wrong_connection() {
    let mut source_builder = PostgresDataSourceBuilder::new("connection_code");
    let mut source: PostgresDataSource = source_builder.build();
    source
        .run_query("select * from test_table_1")
        .expect("run query");
}

#[test]
#[should_panic]
fn wrong_table_name() {
    let mut source_builder = PostgresDataSourceBuilder::new("connection_code");
    let mut source: PostgresDataSource = source_builder.build();
    source
        .run_query("select * from test_table_wrong")
        .expect("run query");
}

#[test]
fn load_and_parse() {
    #[derive(Debug, PartialEq)]
    enum Value {
        Id(u64),
        Name(String),
        Email(String),
        Age(u64)
    }

    let mut source_builder = PostgresDataSourceBuilder::new("host=localhost user=postgres dbname=dataprep port=5432 password=postgres");
    let mut source: PostgresDataSource = source_builder.build();
    source
        .run_query("select * from person")
        .expect("run query");

    assert_eq!(3, source.nrows);
    assert_eq!(4, source.ncols);

    let mut results: Vec<Value> = Vec::new();
    for _i in 0..source.nrows {
        results.push(Value::Id(source.produce().expect("parse id")));
        results.push(Value::Name(source.produce().expect("parse name")));
        results.push(Value::Email(
            source.produce().expect("parse email"),
        ));
        results.push(Value::Age(source.produce().expect("parse age")));
    }

    assert_eq!(
        vec![
            Value::Id(1),
            Value::Name(String::from("Raj")),
            Value::Email(String::from("raj@gmail.com")),
            Value::Age(22),
            Value::Id(2),
            Value::Name(String::from("Abhishek")),
            Value::Email(String::from("ab@gmail.com")),
            Value::Age(32),
            Value::Id(3),
            Value::Name(String::from("Ashish")),
            Value::Email(String::from("ashish@gmail.com")),
            Value::Age(25),
        ],
        results
    );
}

// #[test]
// fn test_postgres() {
//     let schema = vec![DataType::U64; 2]; 
//     let queries = vec![
//         "select id, age from person where id < 2".to_string(),
//         "select id, age from person where id >= 2".to_string(),
//     ];
//     let dispatcher = Dispatcher::new(PostgresDataSourceBuilder::new("host=localhost user=postgres dbname=dataprep port=5432 password=postgres"), schema, queries);
//
//     let dw = dispatcher
//         .run_checked::<U64Writer>()
//         .expect("run dispatcher");
//
//     assert_eq!(
//         array![
//             [1, 22],
//             [2, 32],
//             [3, 25]
//         ],
//         dw.buffer()
//     );
// }

#[test]
fn test_postgres() {
    let schema = vec![DataType::String; 2];
    let queries = vec![
        "select name, email from person where id < 2".to_string(),
        "select name, email from person where id >= 2".to_string(),
    ];
    let builder = PostgresDataSourceBuilder::new("host=localhost user=postgres dbname=dataprep port=5432 password=postgres");
    let dispatcher = Dispatcher::new(builder, MemoryWriter::new(), schema, queries);

    let dw = dispatcher.run_checked().expect("run dispatcher");

    assert_eq!(
        array![
            ["Raj", "raj@gmail.com"],
            ["Abhishek", "ab@gmail.com"],
            ["Ashish", "ashish@gmail.com"]
        ],
        dw.buffer_view::<DataType>(0).unwrap()
    );
}
