use crate::common::Server;
use assert_json_diff::assert_json_include;
use serde_json::json;

#[actix_rt::test]
async fn add_valid_api_key() {
    let mut server = Server::new_auth().await;
    server.use_api_key("MASTER_KEY");

    let content = json!({
        "description": "Indexing API key",
        "indexes": ["products"],
        "actions": [
            "search",
            "documents.add",
            "documents.get",
            "documents.delete",
            "indexes.add",
            "indexes.get",
            "indexes.update",
            "indexes.delete",
            "tasks.get",
            "settings.get",
            "settings.update",
            "settings.reset",
            "stats",
            "dumps"
        ],
        "expiresAt": "2050-11-13T00:00:00Z"
    });

    let (response, code) = server.add_api_key(content).await;

    assert!(response["key"].is_string());
    assert!(response["expiresAt"].is_string());
    assert!(response["createdAt"].is_string());
    assert!(response["updatedAt"].is_string());

    let expected_response = json!({
        "description": "Indexing API key",
        "indexes": ["products"],
        "actions": [
            "search",
            "documents.add",
            "documents.get",
            "documents.delete",
            "indexes.add",
            "indexes.get",
            "indexes.update",
            "indexes.delete",
            "tasks.get",
            "settings.get",
            "settings.update",
            "settings.reset",
            "stats",
            "dumps"
        ],
        "expiresAt": "2050-11-13T00:00:00Z"
    });

    assert_json_include!(actual: response, expected: expected_response);
    assert_eq!(code, 201);
}

#[actix_rt::test]
async fn add_valid_api_key_no_description() {
    let mut server = Server::new_auth().await;
    server.use_api_key("MASTER_KEY");

    let content = json!({
        "indexes": ["products"],
        "actions": [
            "documents.add"
        ],
        "expiresAt": "2050-11-13T00:00:00Z"
    });

    let (response, code) = server.add_api_key(content).await;

    assert!(response["key"].is_string());
    assert!(response["expiresAt"].is_string());
    assert!(response["createdAt"].is_string());
    assert!(response["updatedAt"].is_string());

    let expected_response = json!({
        "actions": [
            "documents.add"
        ],
        "indexes": [
            "products"
        ],
        "expiresAt": "2050-11-13T00:00:00Z"
    });

    assert_json_include!(actual: response, expected: expected_response);
    assert_eq!(code, 201);
}

#[actix_rt::test]
async fn error_add_api_key_no_header() {
    let server = Server::new_auth().await;
    let content = json!({
        "description": "Indexing API key",
        "indexes": ["products"],
        "actions": [
            "documents.add"
        ],
        "expiresAt": "2050-11-13T00:00:00Z"
    });
    let (response, code) = server.add_api_key(content).await;

    let expected_response = json!({
        "message": "The X-MEILI-API-KEY header is missing.",
        "code": "missing_authorization_header",
        "type": "auth",
        "link": "https://docs.meilisearch.com/errors#missing_authorization_header"
    });

    assert_eq!(response, expected_response);
    assert_eq!(code, 401);
}

#[actix_rt::test]
async fn error_add_api_key_bad_key() {
    let mut server = Server::new_auth().await;
    server.use_api_key("d4000bd7225f77d1eb22cc706ed36772bbc36767c016a27f76def7537b68600d");

    let content = json!({
        "description": "Indexing API key",
        "indexes": ["products"],
        "actions": [
            "documents.add"
        ],
        "expiresAt": "2050-11-13T00:00:00Z"
    });
    let (response, code) = server.add_api_key(content).await;

    let expected_response = json!({
        "message": "The provided API key is invalid.",
        "code": "invalid_api_key",
        "type": "auth",
        "link": "https://docs.meilisearch.com/errors#invalid_api_key"
    });

    assert_eq!(response, expected_response);
    assert_eq!(code, 403);
}

#[actix_rt::test]
async fn error_add_api_key_missing_parameter() {
    let mut server = Server::new_auth().await;
    server.use_api_key("MASTER_KEY");

    // missing indexes
    let content = json!({
        "description": "Indexing API key",
        "actions": [
            "documents.add"
        ],
        "expiresAt": "2050-11-13T00:00:00Z"
    });
    let (response, code) = server.add_api_key(content).await;

    let expected_response = json!({
        "message": "`indexes` field is mandatory.",
        "code": "missing_parameter",
        "type": "invalid_request",
        "link":"https://docs.meilisearch.com/errors#missing_parameter"
    });

    assert_eq!(response, expected_response);
    assert_eq!(code, 400);

    // missing actions
    let content = json!({
        "description": "Indexing API key",
        "indexes": ["products"],
        "expiresAt": "2050-11-13T00:00:00Z"
    });
    let (response, code) = server.add_api_key(content).await;

    let expected_response = json!({
        "message": "`actions` field is mandatory.",
        "code": "missing_parameter",
        "type": "invalid_request",
        "link":"https://docs.meilisearch.com/errors#missing_parameter"
    });

    assert_eq!(response, expected_response);
    assert_eq!(code, 400);

    // missing expiresAt
    let content = json!({
        "description": "Indexing API key",
        "indexes": ["products"],
        "actions": [
            "documents.add"
        ],
    });
    let (response, code) = server.add_api_key(content).await;

    let expected_response = json!({
        "message": "`expiresAt` field is mandatory.",
        "code": "missing_parameter",
        "type": "invalid_request",
        "link":"https://docs.meilisearch.com/errors#missing_parameter"
    });

    assert_eq!(response, expected_response);
    assert_eq!(code, 400);
}

#[actix_rt::test]
async fn error_add_api_key_invalid_parameters_description() {
    let mut server = Server::new_auth().await;
    server.use_api_key("MASTER_KEY");

    let content = json!({
        "description": {"name": "products"},
        "indexes": ["products"],
        "actions": [
            "documents.add"
        ],
        "expiresAt": "2050-11-13T00:00:00Z"
    });
    let (response, code) = server.add_api_key(content).await;

    let expected_response = json!({
        "message": r#"description field value `{"name": "products"}` is invalid. It should be a string or specified as a null value."#,
        "code": "invalid_api_key_description",
        "type": "invalid_request",
        "link": "https://docs.meilisearch.com/errors#invalid_api_key_description"
    });

    assert_eq!(response, expected_response);
    assert_eq!(code, 400);
}

#[actix_rt::test]
async fn error_add_api_key_invalid_parameters_indexes() {
    let mut server = Server::new_auth().await;
    server.use_api_key("MASTER_KEY");

    let content = json!({
        "description": "Indexing API key",
        "indexes": {"name": "products"},
        "actions": [
            "documents.add"
        ],
        "expiresAt": "2050-11-13T00:00:00Z"
    });
    let (response, code) = server.add_api_key(content).await;

    let expected_response = json!({
        "message": r#"indexes field value `{"name": "products"}` is invalid. It should be an array of string representing index names."#,
        "code": "invalid_api_key_indexes",
        "type": "invalid_request",
        "link": "https://docs.meilisearch.com/errors#invalid_api_key_indexes"
    });

    assert_eq!(response, expected_response);
    assert_eq!(code, 400);
}

#[actix_rt::test]
async fn error_add_api_key_invalid_parameters_actions() {
    let mut server = Server::new_auth().await;
    server.use_api_key("MASTER_KEY");

    let content = json!({
        "description": "Indexing API key",
        "indexes": ["products"],
        "actions": {"name": "products"},
        "expiresAt": "2050-11-13T00:00:00Z"
    });
    let (response, code) = server.add_api_key(content).await;

    let expected_response = json!({
        "message": r#"actions field value `{"name": "products"}` is invalid. It should be an array of string representing action names."#,
        "code": "invalid_api_key_actions",
        "type": "invalid_request",
        "link": "https://docs.meilisearch.com/errors#invalid_api_key_actions"
    });

    assert_eq!(response, expected_response);
    assert_eq!(code, 400);

    let content = json!({
        "description": "Indexing API key",
        "indexes": ["products"],
        "actions": [
            "doc.add"
        ],
        "expiresAt": "2050-11-13T00:00:00Z"
    });
    let (response, code) = server.add_api_key(content).await;

    let expected_response = json!({
        "message": r#"actions field value `["doc.add"]` is invalid. It should be an array of string representing action names."#,
        "code": "invalid_api_key_actions",
        "type": "invalid_request",
        "link": "https://docs.meilisearch.com/errors#invalid_api_key_actions"
    });

    assert_eq!(response, expected_response);
    assert_eq!(code, 400);
}

#[actix_rt::test]
async fn error_add_api_key_invalid_parameters_expires_at() {
    let mut server = Server::new_auth().await;
    server.use_api_key("MASTER_KEY");

    let content = json!({
        "description": "Indexing API key",
        "indexes": ["products"],
        "actions": [
            "documents.add"
        ],
        "expiresAt": {"name": "products"}
    });
    let (response, code) = server.add_api_key(content).await;

    let expected_response = json!({
        "message": r#"expiresAt field value `{"name": "products"}` is invalid. It should be in ISO-8601 format to represents a date or datetime in the future or specified as a null value. e.g. 'YYYY-MM-DD' or 'YYYY-MM-DDTHH:MM:SS'."#,
        "code": "invalid_api_key_expires_at",
        "type": "invalid_request",
        "link": "https://docs.meilisearch.com/errors#invalid_api_key_expires_at"
    });

    assert_eq!(response, expected_response);
    assert_eq!(code, 400);
}

#[actix_rt::test]
async fn get_api_key() {
    let mut server = Server::new_auth().await;
    server.use_api_key("MASTER_KEY");

    let content = json!({
        "description": "Indexing API key",
        "indexes": ["products"],
        "actions": [
            "search",
            "documents.add",
            "documents.get",
            "documents.delete",
            "indexes.add",
            "indexes.get",
            "indexes.update",
            "indexes.delete",
            "tasks.get",
            "settings.get",
            "settings.update",
            "settings.reset",
            "stats",
            "dumps"
        ],
        "expiresAt": "2050-11-13T00:00:00Z"
    });

    let (response, code) = server.add_api_key(content).await;
    // must pass if add_valid_api_key test passes.
    assert_eq!(code, 201);
    assert!(response["key"].is_string());

    let key = response["key"].as_str().unwrap();

    let (response, code) = server.get_api_key(&key).await;
    assert!(response["key"].is_string());
    assert!(response["expiresAt"].is_string());
    assert!(response["createdAt"].is_string());
    assert!(response["updatedAt"].is_string());

    let expected_response = json!({
        "description": "Indexing API key",
        "indexes": ["products"],
        "actions": [
            "search",
            "documents.add",
            "documents.get",
            "documents.delete",
            "indexes.add",
            "indexes.get",
            "indexes.update",
            "indexes.delete",
            "tasks.get",
            "settings.get",
            "settings.update",
            "settings.reset",
            "stats",
            "dumps"
        ],
        "expiresAt": "2050-11-13T00:00:00Z"
    });

    assert_json_include!(actual: response, expected: expected_response);
    assert_eq!(code, 201);
}

#[actix_rt::test]
async fn error_get_api_key_no_header() {
    let server = Server::new_auth().await;

    let (response, code) = server
        .get_api_key("d0552b41536279a0ad88bd595327b96f01176a60c2243e906c52ac02375f9bc4")
        .await;

    let expected_response = json!({
        "message": "The X-MEILI-API-KEY header is missing.",
        "code": "missing_authorization_header",
        "type": "auth",
        "link": "https://docs.meilisearch.com/errors#missing_authorization_header"
    });

    assert_eq!(response, expected_response);
    assert_eq!(code, 401);
}

#[actix_rt::test]
async fn error_get_api_key_bad_key() {
    let mut server = Server::new_auth().await;
    server.use_api_key("d4000bd7225f77d1eb22cc706ed36772bbc36767c016a27f76def7537b68600d");

    let (response, code) = server
        .get_api_key("d0552b41536279a0ad88bd595327b96f01176a60c2243e906c52ac02375f9bc4")
        .await;

    let expected_response = json!({
        "message": "The provided API key is invalid.",
        "code": "invalid_api_key",
        "type": "auth",
        "link": "https://docs.meilisearch.com/errors#invalid_api_key"
    });

    assert_eq!(response, expected_response);
    assert_eq!(code, 403);
}

#[actix_rt::test]
async fn error_get_api_key_not_found() {
    let mut server = Server::new_auth().await;
    server.use_api_key("MASTER_KEY");

    let (response, code) = server
        .get_api_key("d0552b41536279a0ad88bd595327b96f01176a60c2243e906c52ac02375f9bc4")
        .await;

    let expected_response = json!({
        "message": "Api key `d0552b41536279a0ad88bd595327b96f01176a60c2243e906c52ac02375f9bc4` not found.",
        "code": "api_key_not_found",
        "type": "invalid_request",
        "link": "https://docs.meilisearch.com/errors#api_key_not_found"
    });

    assert_eq!(response, expected_response);
    assert_eq!(code, 404);
}

#[actix_rt::test]
async fn put_api_key_description() {
    let mut server = Server::new_auth().await;
    server.use_api_key("MASTER_KEY");

    let content = json!({
        "indexes": ["products"],
        "actions": [
            "search",
            "documents.add",
            "documents.get",
            "documents.delete",
            "indexes.add",
            "indexes.get",
            "indexes.update",
            "indexes.delete",
            "stats",
            "dumps"
        ],
        "expiresAt": "2050-11-13T00:00:00Z"
    });

    let (response, code) = server.add_api_key(content).await;
    // must pass if add_valid_api_key test passes.
    assert_eq!(code, 201);
    assert!(response["key"].is_string());
    assert!(response["createdAt"].is_string());
    assert!(response["updatedAt"].is_string());

    let key = response["key"].as_str().unwrap();
    let created_at = response["createdAt"].as_str().unwrap();
    let updated_at = response["updatedAt"].as_str().unwrap();

    // Add a description
    let content = json!({
        "description": "Indexing API key",
        "indexes": ["products"],
        "actions": [
            "search",
            "documents.add",
            "documents.get",
            "documents.delete",
            "indexes.add",
            "indexes.get",
            "indexes.update",
            "indexes.delete",
            "stats",
            "dumps"
        ],
        "expiresAt": "2050-11-13T00:00:00Z"
    });

    let (response, code) = server.put_api_key(&key, content.clone()).await;
    assert!(response["key"].is_string());
    assert!(response["expiresAt"].is_string());
    assert!(response["createdAt"].is_string());
    assert_ne!(response["updatedAt"].as_str().unwrap(), updated_at);
    assert_eq!(response["createdAt"].as_str().unwrap(), created_at);

    assert_json_include!(actual: response, expected: content);
    assert_eq!(code, 201);

    // Change the description
    let content = json!({
        "description": "Porduct API key",
        "indexes": ["products"],
        "actions": [
            "search",
            "documents.add",
            "documents.get",
            "documents.delete",
            "indexes.add",
            "indexes.get",
            "indexes.update",
            "indexes.delete",
            "stats",
            "dumps"
        ],
        "expiresAt": "2050-11-13T00:00:00Z"
    });

    let (response, code) = server.put_api_key(&key, content.clone()).await;
    assert!(response["key"].is_string());
    assert!(response["expiresAt"].is_string());
    assert!(response["createdAt"].is_string());

    assert_json_include!(actual: response, expected: content);
    assert_eq!(code, 201);

    // Remove the description
    let content = json!({
        "description": serde_json::Value::Null,
        "indexes": ["products"],
        "actions": [
            "search",
            "documents.add",
            "documents.get",
            "documents.delete",
            "indexes.add",
            "indexes.get",
            "indexes.update",
            "indexes.delete",
            "stats",
            "dumps"
        ],
        "expiresAt": "2050-11-13T00:00:00Z"
    });

    let (response, code) = server.put_api_key(&key, content.clone()).await;
    assert!(response["key"].is_string());
    assert!(response["expiresAt"].is_string());
    assert!(response["createdAt"].is_string());

    assert_json_include!(actual: response, expected: content);
    assert_eq!(code, 201);
}

#[actix_rt::test]
async fn put_api_key_indexes() {
    let mut server = Server::new_auth().await;
    server.use_api_key("MASTER_KEY");

    let content = json!({
        "description": "Indexing API key",
        "indexes": ["products"],
        "actions": [
            "search",
            "documents.add",
            "documents.get",
            "documents.delete",
            "indexes.add",
            "indexes.get",
            "indexes.update",
            "indexes.delete",
            "stats",
            "dumps"
        ],
        "expiresAt": "2050-11-13T00:00:00Z"
    });

    let (response, code) = server.add_api_key(content).await;
    // must pass if add_valid_api_key test passes.
    assert_eq!(code, 201);
    assert!(response["key"].is_string());
    assert!(response["createdAt"].is_string());
    assert!(response["updatedAt"].is_string());

    let key = response["key"].as_str().unwrap();
    let created_at = response["createdAt"].as_str().unwrap();
    let updated_at = response["updatedAt"].as_str().unwrap();

    let content = json!({
        "description": "Indexing API key",
        "indexes": ["products", "prices"],
        "actions": [
            "search",
            "documents.add",
            "documents.get",
            "documents.delete",
            "indexes.add",
            "indexes.get",
            "indexes.update",
            "indexes.delete",
            "stats",
            "dumps"
        ],
        "expiresAt": "2050-11-13T00:00:00Z"
    });

    let (response, code) = server.put_api_key(&key, content.clone()).await;
    assert!(response["key"].is_string());
    assert!(response["expiresAt"].is_string());
    assert!(response["createdAt"].is_string());
    assert_ne!(response["updatedAt"].as_str().unwrap(), updated_at);
    assert_eq!(response["createdAt"].as_str().unwrap(), created_at);

    assert_json_include!(actual: response, expected: content);
    assert_eq!(code, 201);
}

#[actix_rt::test]
async fn put_api_key_actions() {
    let mut server = Server::new_auth().await;
    server.use_api_key("MASTER_KEY");

    let content = json!({
        "description": "Indexing API key",
        "indexes": ["products"],
        "actions": [
            "search",
            "documents.add",
            "documents.get",
            "documents.delete",
            "indexes.add",
            "indexes.get",
            "indexes.update",
            "indexes.delete",
            "stats",
            "dumps"
        ],
        "expiresAt": "2050-11-13T00:00:00Z"
    });

    let (response, code) = server.add_api_key(content).await;
    // must pass if add_valid_api_key test passes.
    assert_eq!(code, 201);
    assert!(response["key"].is_string());
    assert!(response["createdAt"].is_string());
    assert!(response["updatedAt"].is_string());

    let key = response["key"].as_str().unwrap();
    let created_at = response["createdAt"].as_str().unwrap();
    let updated_at = response["updatedAt"].as_str().unwrap();

    let content = json!({
        "description": "Indexing API key",
        "indexes": ["products"],
        "actions": [
            "search",
            "documents.get",
            "indexes.get",
            "tasks.get",
            "settings.get",
        ],
        "expiresAt": "2050-11-13T00:00:00Z"
    });

    let (response, code) = server.put_api_key(&key, content.clone()).await;
    assert!(response["key"].is_string());
    assert!(response["expiresAt"].is_string());
    assert!(response["createdAt"].is_string());
    assert_ne!(response["updatedAt"].as_str().unwrap(), updated_at);
    assert_eq!(response["createdAt"].as_str().unwrap(), created_at);

    assert_json_include!(actual: response, expected: content);
    assert_eq!(code, 201);
}

#[actix_rt::test]
async fn put_api_key_expiration_date() {
    let mut server = Server::new_auth().await;
    server.use_api_key("MASTER_KEY");

    let content = json!({
        "description": "Indexing API key",
        "indexes": ["products"],
        "actions": [
            "search",
            "documents.add",
            "documents.get",
            "documents.delete",
            "indexes.add",
            "indexes.get",
            "indexes.update",
            "indexes.delete",
            "stats",
            "dumps"
        ],
        "expiresAt": "205-11-13T00:00:00Z"
    });

    let (response, code) = server.add_api_key(content).await;
    // must pass if add_valid_api_key test passes.
    assert_eq!(code, 201);
    assert!(response["key"].is_string());
    assert!(response["createdAt"].is_string());
    assert!(response["updatedAt"].is_string());

    let key = response["key"].as_str().unwrap();
    let created_at = response["createdAt"].as_str().unwrap();
    let updated_at = response["updatedAt"].as_str().unwrap();

    let content = json!({
        "description": "Indexing API key",
        "indexes": ["products"],
        "actions": [
            "search",
            "documents.add",
            "documents.get",
            "documents.delete",
            "indexes.add",
            "indexes.get",
            "indexes.update",
            "indexes.delete",
            "stats",
            "dumps"
        ],
        "expiresAt": "2055-11-13T00:00:00Z"
    });

    let (response, code) = server.put_api_key(&key, content.clone()).await;
    assert!(response["key"].is_string());
    assert!(response["expiresAt"].is_string());
    assert!(response["createdAt"].is_string());
    assert_ne!(response["updatedAt"].as_str().unwrap(), updated_at);
    assert_eq!(response["createdAt"].as_str().unwrap(), created_at);

    assert_json_include!(actual: response, expected: content);
    assert_eq!(code, 201);
}

#[actix_rt::test]
async fn error_put_api_key_no_header() {
    let server = Server::new_auth().await;

    let (response, code) = server
        .put_api_key(
            "d0552b41536279a0ad88bd595327b96f01176a60c2243e906c52ac02375f9bc4",
            json!({}),
        )
        .await;

    let expected_response = json!({
        "message": "The X-MEILI-API-KEY header is missing.",
        "code": "missing_authorization_header",
        "type": "auth",
        "link": "https://docs.meilisearch.com/errors#missing_authorization_header"
    });

    assert_eq!(response, expected_response);
    assert_eq!(code, 401);
}

#[actix_rt::test]
async fn error_put_api_key_bad_key() {
    let mut server = Server::new_auth().await;
    server.use_api_key("d4000bd7225f77d1eb22cc706ed36772bbc36767c016a27f76def7537b68600d");

    let (response, code) = server
        .put_api_key(
            "d0552b41536279a0ad88bd595327b96f01176a60c2243e906c52ac02375f9bc4",
            json!({}),
        )
        .await;

    let expected_response = json!({
        "message": "The provided API key is invalid.",
        "code": "invalid_api_key",
        "type": "auth",
        "link": "https://docs.meilisearch.com/errors#invalid_api_key"
    });

    assert_eq!(response, expected_response);
    assert_eq!(code, 403);
}

#[actix_rt::test]
async fn error_put_api_key_not_found() {
    let mut server = Server::new_auth().await;
    server.use_api_key("MASTER_KEY");

    let (response, code) = server
        .put_api_key(
            "d0552b41536279a0ad88bd595327b96f01176a60c2243e906c52ac02375f9bc4",
            json!({}),
        )
        .await;

    let expected_response = json!({
        "message": "Api key `d0552b41536279a0ad88bd595327b96f01176a60c2243e906c52ac02375f9bc4` not found.",
        "code": "api_key_not_found",
        "type": "invalid_request",
        "link": "https://docs.meilisearch.com/errors#api_key_not_found"
    });

    assert_eq!(response, expected_response);
    assert_eq!(code, 404);
}

#[actix_rt::test]
async fn error_put_api_key_indexes_missing_parameters() {
    let mut server = Server::new_auth().await;
    server.use_api_key("MASTER_KEY");

    let content = json!({
        "description": "Indexing API key",
        "indexes": ["products"],
        "actions": [
            "search",
        ],
        "expiresAt": "2050-11-13T00:00:00Z"
    });

    let (response, code) = server.add_api_key(content).await;
    // must pass if add_valid_api_key test passes.
    assert_eq!(code, 201);
    assert!(response["key"].is_string());

    let key = response["key"].as_str().unwrap();

    // missing description
    let content = json!({
        "indexes": ["products"],
        "actions": [
            "documents.add"
        ],
        "expiresAt": "2050-11-13T00:00:00Z"
    });

    let (response, code) = server.put_api_key(&key, content).await;

    let expected_response = json!({
        "message": "`descrition` field is mandatory.",
        "code": "missing_parameter",
        "type": "invalid_request",
        "link":"https://docs.meilisearch.com/errors#missing_parameter"
    });

    assert_eq!(response, expected_response);
    assert_eq!(code, 400);

    // missing indexes
    let content = json!({
        "description": "Indexing API key",
        "actions": [
            "documents.add"
        ],
        "expiresAt": "2050-11-13T00:00:00Z"
    });

    let (response, code) = server.put_api_key(&key, content).await;

    let expected_response = json!({
        "message": "`indexes` field is mandatory.",
        "code": "missing_parameter",
        "type": "invalid_request",
        "link":"https://docs.meilisearch.com/errors#missing_parameter"
    });

    assert_eq!(response, expected_response);
    assert_eq!(code, 400);

    // missing actions
    let content = json!({
        "description": "Indexing API key",
        "indexes": ["products"],
        "expiresAt": "2050-11-13T00:00:00Z"
    });
    let (response, code) = server.put_api_key(&key, content).await;

    let expected_response = json!({
        "message": "`actions` field is mandatory.",
        "code": "missing_parameter",
        "type": "invalid_request",
        "link":"https://docs.meilisearch.com/errors#missing_parameter"
    });

    assert_eq!(response, expected_response);
    assert_eq!(code, 400);

    // missing expiresAt
    let content = json!({
        "description": "Indexing API key",
        "indexes": ["products"],
        "actions": [
            "documents.add"
        ],
    });
    let (response, code) = server.put_api_key(&key, content).await;

    let expected_response = json!({
        "message": "`expiresAt` field is mandatory.",
        "code": "missing_parameter",
        "type": "invalid_request",
        "link":"https://docs.meilisearch.com/errors#missing_parameter"
    });

    assert_eq!(response, expected_response);
    assert_eq!(code, 400);
}

#[actix_rt::test]
async fn error_put_api_key_indexes_invalid_parameters() {
    let mut server = Server::new_auth().await;
    server.use_api_key("MASTER_KEY");

    let content = json!({
        "description": "Indexing API key",
        "indexes": ["products"],
        "actions": [
            "search",
        ],
        "expiresAt": "2050-11-13T00:00:00Z"
    });

    let (response, code) = server.add_api_key(content).await;
    // must pass if add_valid_api_key test passes.
    assert_eq!(code, 201);
    assert!(response["key"].is_string());

    let key = response["key"].as_str().unwrap();

    // invalid description
    let content = json!({
        "description": 13,
        "indexes": ["products"],
        "actions": [
            "search",
        ],
        "expiresAt": "2050-11-13T00:00:00Z"
    });

    let (response, code) = server.put_api_key(&key, content).await;

    let expected_response = json!({
        "message": "description field value `13` is invalid. It should be a string or specified as a null value.",
        "code": "invalid_api_key_description",
        "type": "invalid_request",
        "link": "https://docs.meilisearch.com/errors#invalid_api_key_description"
    });

    assert_eq!(response, expected_response);
    assert_eq!(code, 400);

    // invalid indexes
    let content = json!({
        "description": "Indexing API key",
        "indexes": 13,
        "actions": [
            "search",
        ],
        "expiresAt": "2050-11-13T00:00:00Z"
    });

    let (response, code) = server.put_api_key(&key, content).await;

    let expected_response = json!({
        "message": "indexes field value `13` is invalid. It should be an array of string representing index names.",
        "code": "invalid_api_key_indexes",
        "type": "invalid_request",
        "link": "https://docs.meilisearch.com/errors#invalid_api_key_indexes"
    });

    assert_eq!(response, expected_response);
    assert_eq!(code, 400);

    // invalid actions
    let content = json!({
        "description": "Indexing API key",
        "indexes": ["products"],
        "actions": 13,
        "expiresAt": "2050-11-13T00:00:00Z"
    });
    let (response, code) = server.put_api_key(&key, content).await;

    let expected_response = json!({
        "message": "actions field value `13` is invalid. It should be an array of string representing action names.",
        "code": "invalid_api_key_actions",
        "type": "invalid_request",
        "link": "https://docs.meilisearch.com/errors#invalid_api_key_actions"
    });

    assert_eq!(response, expected_response);
    assert_eq!(code, 400);

    // invalid expiresAt
    let content = json!({
        "description": "Indexing API key",
        "indexes": ["products"],
        "actions": [
            "search",
        ],
        "expiresAt": 13
    });
    let (response, code) = server.put_api_key(&key, content).await;

    let expected_response = json!({
        "message": "expiresAt field value `13` is invalid. It should be in ISO-8601 format to represents a date or datetime in the future or specified as a null value. e.g. 'YYYY-MM-DD' or 'YYYY-MM-DDTHH:MM:SS'.",
        "code": "invalid_api_key_expires_at",
        "type": "invalid_request",
        "link": "https://docs.meilisearch.com/errors#invalid_api_key_expires_at"
    });

    assert_eq!(response, expected_response);
    assert_eq!(code, 400);
}

#[actix_rt::test]
async fn list_api_keys() {
    let mut server = Server::new_auth().await;
    server.use_api_key("MASTER_KEY");

    let content = json!({
        "description": "Indexing API key",
        "indexes": ["products"],
        "actions": [
            "search",
            "documents.add",
            "documents.get",
            "documents.delete",
            "indexes.add",
            "indexes.get",
            "indexes.update",
            "indexes.delete",
            "tasks.get",
            "settings.get",
            "settings.update",
            "settings.reset",
            "stats",
            "dumps"
        ],
        "expiresAt": "2050-11-13T00:00:00Z"
    });

    let (_response, code) = server.add_api_key(content).await;
    // must pass if add_valid_api_key test passes.
    assert_eq!(code, 201);

    let (response, code) = server.list_api_keys().await;
    assert!(response.is_array());
    let response = &response.as_array().unwrap()[0];
    assert!(response["key"].is_string());
    assert!(response["expiresAt"].is_string());
    assert!(response["createdAt"].is_string());
    assert!(response["updatedAt"].is_string());

    let expected_response = json!({
        "description": "Indexing API key",
        "indexes": ["products"],
        "actions": [
            "search",
            "documents.add",
            "documents.get",
            "documents.delete",
            "indexes.add",
            "indexes.get",
            "indexes.update",
            "indexes.delete",
            "tasks.get",
            "settings.get",
            "settings.update",
            "settings.reset",
            "stats",
            "dumps"
        ],
        "expiresAt": "2050-11-13T00:00:00Z"
    });

    assert_json_include!(actual: response, expected: expected_response);
    assert_eq!(code, 201);
}

#[actix_rt::test]
async fn error_list_api_keys_no_header() {
    let server = Server::new_auth().await;

    let (response, code) = server.list_api_keys().await;

    let expected_response = json!({
        "message": "The X-MEILI-API-KEY header is missing.",
        "code": "missing_authorization_header",
        "type": "auth",
        "link": "https://docs.meilisearch.com/errors#missing_authorization_header"
    });

    assert_eq!(response, expected_response);
    assert_eq!(code, 401);
}

#[actix_rt::test]
async fn error_list_api_keys_bad_key() {
    let mut server = Server::new_auth().await;
    server.use_api_key("d4000bd7225f77d1eb22cc706ed36772bbc36767c016a27f76def7537b68600d");

    let (response, code) = server.list_api_keys().await;

    let expected_response = json!({
        "message": "The provided API key is invalid.",
        "code": "invalid_api_key",
        "type": "auth",
        "link": "https://docs.meilisearch.com/errors#invalid_api_key"
    });

    assert_eq!(response, expected_response);
    assert_eq!(code, 403);
}