use app::{docs::v1::todos::TodoResponse, util::Clock};
use axum::http::StatusCode;
use axum_test_helper::TestClient;
use serde_json::json;
use ulid::Ulid;

use crate::fixtures::{
    app::{get_app, Dependencies, DATETIME_STRING},
    clock::MockClock,
    id_generator::MockUlidGenerator,
};

mod fixtures;

mod create_todo {
    use super::*;

    #[tokio::test]
    async fn fails_for_bad_input() {
        let id = Ulid::new();
        let clock = MockClock::with_frozen_time(DATETIME_STRING.into());
        let id_generator = MockUlidGenerator::with_fixed_value(&id.to_string());

        let dependencies = Dependencies::new(clock.clone(), id_generator);

        let app = get_app(dependencies).await;

        let payload = json!({
          "description": "Buy groceries from the supermarket for the weekend.",
          "dueDate": "ALIEN DATE",
          "subject": "Buy groceries"
        });

        let res = app.post("/v1/todos").json(&payload).send().await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn successfully_create_todos() {
        let id = Ulid::new().to_string();
        let clock = MockClock::with_frozen_time(DATETIME_STRING.into());
        let id_generator = MockUlidGenerator::with_fixed_value(&id);

        let dependencies = Dependencies::new(clock.clone(), id_generator);
        let app = get_app(dependencies).await;

        let payload = json!({
          "description": "Buy groceries from the supermarket for the weekend.",
          "dueDate": "2023-11-04T15:32:34.205052Z",
          "subject": "Buy groceries"
        });

        let res = app.post("/v1/todos").json(&payload).send().await;
        let response_status = res.status();
        let response_body: TodoResponse = res.json().await;

        assert_eq!(response_status, StatusCode::OK);

        assert_eq!(response_body.id, id.to_string());
        assert_eq!(response_body.subject, "Buy groceries");
        assert_eq!(
            response_body.description,
            "Buy groceries from the supermarket for the weekend."
        );
        assert_eq!(response_body.is_done, false);
        assert_eq!(response_body.due_date, clock.now());
        assert_eq!(response_body.created_at, clock.now());
    }
}

mod get_todo_by_id {
    use super::*;

    #[tokio::test]
    async fn returns_not_found_for_bad_input() {
        let id = Ulid::new();
        let clock = MockClock::with_frozen_time(DATETIME_STRING.into());
        let id_generator = MockUlidGenerator::with_fixed_value(&id.to_string());

        let dependencies = Dependencies::new(clock.clone(), id_generator);

        let app = get_app(dependencies).await;

        let res = app.get(&format!("/v1/todos/{}", "BAD_ID")).send().await;

        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn returns_not_found_when_not_found_by_id() {
        let id = Ulid::new();
        let clock = MockClock::with_frozen_time(DATETIME_STRING.into());
        let id_generator = MockUlidGenerator::with_fixed_value(&id.to_string());

        let dependencies = Dependencies::new(clock.clone(), id_generator);

        let app = get_app(dependencies).await;

        let res = app.get(&format!("/v1/todos/{}", Ulid::new())).send().await;

        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn successfully_gets_todo_by_id() {
        let id = Ulid::new().to_string();
        let clock = MockClock::with_frozen_time(DATETIME_STRING.into());
        let id_generator = MockUlidGenerator::with_fixed_value(&id);

        let dependencies = Dependencies::new(clock.clone(), id_generator);
        let app = get_app(dependencies).await;

        create_one_todo(&app).await;

        let res = app.get(&format!("/v1/todos/{}", id)).send().await;
        let response_status = res.status();
        let response_body: TodoResponse = res.json().await;

        assert_eq!(response_status, StatusCode::OK);

        assert_eq!(response_body.id, id.to_string());
        assert_eq!(response_body.subject, "Dummy subject");
        assert_eq!(response_body.description, "Dummy description");
        assert_eq!(response_body.is_done, false);
        assert_eq!(response_body.due_date, clock.now());
        assert_eq!(response_body.created_at, clock.now());
    }
}

mod get_todos {
    use super::*;

    #[tokio::test]
    async fn successfully_gets_todos() {
        let id = Ulid::new().to_string();
        let clock = MockClock::with_frozen_time(DATETIME_STRING.into());
        let id_generator = MockUlidGenerator::with_fixed_value(&id);

        let dependencies = Dependencies::new(clock.clone(), id_generator);
        let app = get_app(dependencies).await;

        create_one_todo(&app).await;

        let res = app.get("/v1/todos").send().await;
        let response_status = res.status();
        let response_body: Vec<TodoResponse> = res.json().await;

        assert_eq!(response_status, StatusCode::OK);
        assert!(response_body.len() > 0);
    }
}

mod update_todo_by_id {
    use super::*;

    #[tokio::test]
    async fn returns_bad_request_for_bad_input() {
        let id = Ulid::new();
        let clock = MockClock::with_frozen_time(DATETIME_STRING.into());
        let id_generator = MockUlidGenerator::with_fixed_value(&id.to_string());

        let dependencies = Dependencies::new(clock.clone(), id_generator);

        let app = get_app(dependencies).await;
        create_one_todo(&app).await;

        let update = json!({
          "description": "Updated description",
          "subject": "Update subject",
          "isDone": "SHOULD BE BOOLEAN",
        });

        let res = app
            .patch(&format!("/v1/todos/{}", id))
            .json(&update)
            .send()
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn returns_not_found_when_not_found_by_id() {
        let id = Ulid::new();
        let clock = MockClock::with_frozen_time(DATETIME_STRING.into());
        let id_generator = MockUlidGenerator::with_fixed_value(&id.to_string());

        let dependencies = Dependencies::new(clock.clone(), id_generator);

        let app = get_app(dependencies).await;
        create_one_todo(&app).await;

        let update = json!({
          "description": "Updated description",
          "subject": "Update subject",
          "isDone": false,
        });

        let res = app
            .patch(&format!("/v1/todos/{}", Ulid::new()))
            .json(&update)
            .send()
            .await;

        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn successfully_updates_todo() {
        let id = Ulid::new();
        let clock = MockClock::with_frozen_time(DATETIME_STRING.into());
        let id_generator = MockUlidGenerator::with_fixed_value(&id.to_string());

        let dependencies = Dependencies::new(clock.clone(), id_generator);

        let app = get_app(dependencies).await;
        create_one_todo(&app).await;

        let update = json!({
          "description": "Updated description",
          "subject": "Updated subject",
          "isDone": true,
        });

        let res = app
            .patch(&format!("/v1/todos/{}", id.clone()))
            .json(&update)
            .send()
            .await;

        let response_status = res.status();
        let response_body: TodoResponse = res.json().await;

        assert_eq!(response_status, StatusCode::OK);

        assert_eq!(response_body.id, id.to_string());
        assert_eq!(response_body.subject, "Updated subject".to_string());
        assert_eq!(response_body.description, "Updated description".to_string());
        assert_eq!(response_body.is_done, true);
        assert_eq!(response_body.due_date, clock.now());
        assert_eq!(response_body.created_at, clock.now());
    }
}

mod delete_todo_by_id {
    use super::*;

    #[tokio::test]
    async fn returns_not_found_when_not_found_by_id() {
        let id = Ulid::new();
        let clock = MockClock::with_frozen_time(DATETIME_STRING.into());
        let id_generator = MockUlidGenerator::with_fixed_value(&id.to_string());

        let dependencies = Dependencies::new(clock.clone(), id_generator);

        let app = get_app(dependencies).await;
        create_one_todo(&app).await;

        let res = app
            .delete(&format!("/v1/todos/{}", Ulid::new()))
            .send()
            .await;

        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn successfully_deletes_todo() {
        let id = Ulid::new();
        let clock = MockClock::with_frozen_time(DATETIME_STRING.into());
        let id_generator = MockUlidGenerator::with_fixed_value(&id.to_string());

        let dependencies = Dependencies::new(clock.clone(), id_generator);

        let app = get_app(dependencies).await;
        create_one_todo(&app).await;

        let res = app
            .delete(&format!("/v1/todos/{}", id.clone()))
            .send()
            .await;

        let response_status = res.status();
        let response_body: TodoResponse = res.json().await;

        assert_eq!(response_status, StatusCode::OK);

        assert_eq!(response_body.id, id.to_string());
        assert_eq!(response_body.subject, "Dummy subject".to_string());
        assert_eq!(response_body.description, "Dummy description".to_string());
        assert_eq!(response_body.is_done, false);
        assert_eq!(response_body.due_date, clock.now());
        assert_eq!(response_body.created_at, clock.now());
    }
}

mod search_todos {
    use std::vec;

    use super::*;

    #[tokio::test]
    async fn returns_empty_array_if_none_found() {
        let id = Ulid::new();
        let search_term = "qwerty";
        let clock = MockClock::with_frozen_time(DATETIME_STRING.into());
        let id_generator = MockUlidGenerator::with_fixed_value(&id.to_string());

        let dependencies = Dependencies::new(clock.clone(), id_generator);

        let app = get_app(dependencies).await;
        create_one_todo(&app).await;

        let res = app
            .get(&format!("/v1/todos/search?q={}", search_term))
            .send()
            .await;

        let expected: Vec<TodoResponse> = vec![];

        let response_status = res.status();
        let response_body: Vec<TodoResponse> = res.json().await;

        assert_eq!(response_status, StatusCode::OK);
        assert_eq!(response_body, expected);
    }

    #[tokio::test]
    async fn returns_array_of_todos_if_found() {
        let id = Ulid::new();
        let search_term = "Dummy";
        let clock = MockClock::with_frozen_time(DATETIME_STRING.into());
        let id_generator = MockUlidGenerator::with_fixed_value(&id.to_string());

        let dependencies = Dependencies::new(clock.clone(), id_generator);

        let app = get_app(dependencies).await;
        create_one_todo(&app).await;

        let res = app
            .get(&format!("/v1/todos/search?q={}", search_term))
            .send()
            .await;

        let response_status = res.status();
        let response_body: Vec<TodoResponse> = res.json().await;

        assert_eq!(response_status, StatusCode::OK);
        assert!(response_body.len() > 0);
    }
}

async fn create_one_todo(app: &TestClient) {
    let payload = json!({
      "description": "Dummy description",
      "dueDate": DATETIME_STRING,
      "subject": "Dummy subject"
    });

    let res = app.post("/v1/todos").json(&payload).send().await;

    assert_eq!(res.status(), StatusCode::OK, "Unable to create Todo");
}
