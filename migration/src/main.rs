use config::Config;
use database::DatabaseDriver;

mod config;
mod database;

#[tokio::main]
async fn main() {
    let config = Config::new();
    let database_driver = DatabaseDriver::init(&config)
        .await
        .expect(&format!("Unable to connect to DB!"));

    if let Err(err) = create_todo_table(&database_driver).await {
        println!("{}", err);
    }

    if let Err(err) = create_todo_index(&database_driver).await {
        println!("{}", err);
    }
}

async fn create_todo_table(database_driver: &DatabaseDriver) -> Result<(), String> {
    let todo_table = r#"
    DEFINE TABLE todo SCHEMAFULL;

    DEFINE FIELD id ON todo TYPE record;// DEFAULT rand::ulid();
    DEFINE FIELD subject ON todo TYPE string;
    DEFINE FIELD description ON todo TYPE string;
    DEFINE FIELD due_date ON todo TYPE datetime;
    DEFINE FIELD is_done ON todo TYPE bool DEFAULT false;
    DEFINE FIELD created_at ON todo TYPE datetime;
    DEFINE FIELD updated_at ON todo TYPE datetime VALUE (
        IF $value < time::now() THEN
            time::now()
        ELSE
            $value
        END
    );
    "#;

    database_driver
        .client
        .query(todo_table)
        .await
        .map_err(|e| e.to_string())?;

    println!("Created table 'todo'");
    Ok(())
}

async fn create_todo_index(database_driver: &DatabaseDriver) -> Result<(), String> {
    let todo_table = r#"
    // DEFINE INDEX todoSearchIndex ON TABLE todo COLUMNS subject SEARCH ANALYZER ascii BM25 HIGHLIGHTS;
    // DEFINE ANALYZER english TOKENIZERS class FILTERS snowball(english);
    // DEFINE ANALYZER todo_search TOKENIZERS class FILTERS ascii;
    
    DEFINE ANALYZER todo_search TOKENIZERS class FILTERS ascii, snowball(english);
    DEFINE INDEX todo_subject_index
        ON todo FIELDS subject
        SEARCH
        ANALYZER todo_search
        BM25(1.2, 0.75);

    DEFINE INDEX todo_description_index
        ON todo FIELDS description
        SEARCH
        ANALYZER todo_search
        BM25(1.2, 0.75);
    "#;

    database_driver
        .client
        .query(todo_table)
        .await
        .map_err(|e| e.to_string())?;

    println!("Created indexes on table 'todo'");
    Ok(())
}
