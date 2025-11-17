use axum_test::TestServer;
use mdserve::{new_router, scan_markdown_files, ServerMessage};
use std::fs;
use std::time::Duration;
use tempfile::{tempdir, Builder, NamedTempFile, TempDir};

const WEBSOCKET_TIMEOUT_SECS: u64 = 5;

const TEST_FILE_1_CONTENT: &str = "# Test 1\n\nContent of test1";
const TEST_FILE_2_CONTENT: &str = "# Test 2\n\nContent of test2";
const TEST_FILE_3_CONTENT: &str = "# Test 3\n\nContent of test3";

fn create_test_server_impl(content: &str, use_http: bool) -> (TestServer, NamedTempFile) {
    let temp_file = Builder::new()
        .suffix(".md")
        .tempfile()
        .expect("Failed to create temp file");
    fs::write(&temp_file, content).expect("Failed to write temp file");

    let canonical_path = temp_file
        .path()
        .canonicalize()
        .unwrap_or_else(|_| temp_file.path().to_path_buf());

    let base_dir = canonical_path
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .to_path_buf();
    let tracked_files = vec![canonical_path];
    let is_directory_mode = false;

    let router =
        new_router(base_dir, tracked_files, is_directory_mode).expect("Failed to create router");

    let server = if use_http {
        TestServer::builder()
            .http_transport()
            .build(router)
            .expect("Failed to create test server")
    } else {
        TestServer::new(router).expect("Failed to create test server")
    };

    (server, temp_file)
}

async fn create_test_server(content: &str) -> (TestServer, NamedTempFile) {
    create_test_server_impl(content, false)
}

async fn create_test_server_with_http(content: &str) -> (TestServer, NamedTempFile) {
    create_test_server_impl(content, true)
}

fn create_directory_server_impl(use_http: bool) -> (TestServer, TempDir) {
    let temp_dir = tempdir().expect("Failed to create temp dir");

    fs::write(temp_dir.path().join("test1.md"), TEST_FILE_1_CONTENT)
        .expect("Failed to write test1.md");
    fs::write(temp_dir.path().join("test2.markdown"), TEST_FILE_2_CONTENT)
        .expect("Failed to write test2.markdown");
    fs::write(temp_dir.path().join("test3.md"), TEST_FILE_3_CONTENT)
        .expect("Failed to write test3.md");

    let base_dir = temp_dir.path().to_path_buf();
    let tracked_files = scan_markdown_files(&base_dir).expect("Failed to scan markdown files");
    let is_directory_mode = true;

    let router =
        new_router(base_dir, tracked_files, is_directory_mode).expect("Failed to create router");

    let server = if use_http {
        TestServer::builder()
            .http_transport()
            .build(router)
            .expect("Failed to create test server")
    } else {
        TestServer::new(router).expect("Failed to create test server")
    };

    (server, temp_dir)
}

async fn create_directory_server() -> (TestServer, TempDir) {
    create_directory_server_impl(false)
}

async fn create_directory_server_with_http() -> (TestServer, TempDir) {
    create_directory_server_impl(true)
}

#[tokio::test]
async fn test_websocket_connection() {
    let (server, _temp_file) = create_test_server_with_http("# WebSocket Test").await;

    // Test that WebSocket endpoint exists and can be connected to
    let response = server.get_websocket("/ws").await;
    response.assert_status_switching_protocols();
}

#[tokio::test]
async fn test_file_modification_updates_via_websocket() {
    let (server, temp_file) = create_test_server_with_http("# Original Content").await;

    let mut websocket = server.get_websocket("/ws").await.into_websocket().await;

    // Modify the file
    fs::write(&temp_file, "# Modified Content").expect("Failed to modify file");


    // Should receive reload signal via WebSocket (with timeout)
    let update_result = tokio::time::timeout(
        Duration::from_secs(WEBSOCKET_TIMEOUT_SECS),
        websocket.receive_json::<ServerMessage>(),
    )
    .await;

    match update_result {
        Ok(update_message) => {
            if let ServerMessage::Reload = update_message {
                // Success - we received a reload signal
            } else {
                panic!("Expected Reload message after file modification");
            }
        }
        Err(_) => {
            panic!("Timeout waiting for WebSocket update after file modification");
        }
    }
}

#[tokio::test]
async fn test_404_for_unknown_routes() {
    let (server, _temp_file) = create_test_server("# 404 Test").await;

    let response = server.get("/unknown-route").await;

    assert_eq!(response.status_code(), 404);
}

#[tokio::test]
async fn test_non_image_files_not_served() {
    use tempfile::tempdir;

    // Create a temporary directory
    let temp_dir = tempdir().expect("Failed to create temp dir");

    // Create a markdown file
    let md_content = "# Test";
    let md_path = temp_dir.path().join("test.md");
    fs::write(&md_path, md_content).expect("Failed to write markdown file");

    // Create a non-image file (txt)
    let txt_path = temp_dir.path().join("secret.txt");
    fs::write(&txt_path, "secret content").expect("Failed to write txt file");

    // Create router with the markdown file (single-file mode)
    let base_dir = temp_dir.path().to_path_buf();
    let tracked_files = vec![md_path];
    let is_directory_mode = false;
    let router =
        new_router(base_dir, tracked_files, is_directory_mode).expect("Failed to create router");
    let server = TestServer::new(router).expect("Failed to create test server");

    // Test that non-image files return 404
    let response = server.get("/secret.txt").await;
    assert_eq!(response.status_code(), 404);
}

// Directory mode tests

#[tokio::test]
async fn test_directory_mode_file_not_found() {
    let (server, _temp_dir) = create_directory_server().await;

    // Test non-existent markdown file
    let response = server.get("/nonexistent.md").await;
    assert_eq!(response.status_code(), 404);
}

#[tokio::test]
async fn test_single_file_mode_no_navigation_sidebar() {
    let (server, _temp_file) = create_test_server("# Single File Test").await;

    let response = server.get("/").await;
    assert_eq!(response.status_code(), 200);
    let body = response.text();

    // Verify no navigation sidebar in single-file mode
    assert!(!body.contains(r#"<nav class="sidebar">"#));
    assert!(!body.contains("<h3>Files</h3>"));
    assert!(!body.contains(r#"<ul class="file-list">"#));
}

#[tokio::test]
async fn test_directory_mode_websocket_file_modification() {
    let (server, temp_dir) = create_directory_server_with_http().await;

    let mut websocket = server.get_websocket("/ws").await.into_websocket().await;

    // Modify one of the tracked files
    let test_file = temp_dir.path().join("test1.md");
    fs::write(&test_file, "# Modified Test 1\n\nContent has changed")
        .expect("Failed to modify file");


    // Should receive reload signal via WebSocket
    let update_result = tokio::time::timeout(
        Duration::from_secs(WEBSOCKET_TIMEOUT_SECS),
        websocket.receive_json::<ServerMessage>(),
    )
    .await;

    match update_result {
        Ok(update_message) => {
            if let ServerMessage::Reload = update_message {
                // Success - we received a reload signal
            } else {
                panic!("Expected Reload message after file modification");
            }
        }
        Err(_) => {
            panic!("Timeout waiting for WebSocket update after file modification");
        }
    }
}

// File rename and removal tests

#[tokio::test]
async fn test_folder_based_routing_404_for_nonexistent_path() {
    let temp_dir = tempdir().expect("Failed to create temp dir");

    let folder1 = temp_dir.path().join("folder1");
    fs::create_dir(&folder1).expect("Failed to create folder1");
    fs::write(folder1.join("doc.md"), "# Doc").expect("Failed to write file");

    let base_dir = temp_dir.path().to_path_buf();
    let tracked_files = scan_markdown_files(&base_dir).expect("Failed to scan");
    
    let router = new_router(base_dir, tracked_files, true).expect("Failed to create router");
    let server = TestServer::new(router).expect("Failed to create test server");

    // Test 404 for non-existent folder path
    let response = server.get("/nonexistent/doc.md").await;
    assert_eq!(response.status_code(), 404);

    // Test 404 for non-existent file in existing folder
    let response = server.get("/folder1/nonexistent.md").await;
    assert_eq!(response.status_code(), 404);
}

// ===========================
// Tree Structure Tests
// ===========================

// ===========================
// Folder Removal Tests
// ===========================

// ===========================
// Root Route Tests
// ===========================

// ============================================================================
// React API Tests - GET /api/files
// ============================================================================

#[tokio::test]
async fn test_api_get_files_single_file() {
    let (server, _temp_dir) = create_directory_server().await;

    let response = server.get("/api/files").await;

    assert_eq!(response.status_code(), 200);

    let json = response.json::<serde_json::Value>();
    let files = json["files"].as_array().expect("files should be an array");

    assert_eq!(files.len(), 3);
    assert_eq!(files[0]["path"], "test1.md");
    assert_eq!(files[1]["path"], "test2.markdown");
    assert_eq!(files[2]["path"], "test3.md");
}

#[tokio::test]
async fn test_api_get_files_nested_folders() {
    let temp_dir = tempdir().expect("Failed to create temp dir");

    // Create folder structure
    let folder1 = temp_dir.path().join("folder1");
    fs::create_dir(&folder1).expect("Failed to create folder1");
    fs::write(folder1.join("file1.md"), "# File 1").expect("Failed to write file1");

    let folder2 = temp_dir.path().join("folder2");
    fs::create_dir(&folder2).expect("Failed to create folder2");
    fs::write(folder2.join("file2.md"), "# File 2").expect("Failed to write file2");

    fs::write(temp_dir.path().join("root.md"), "# Root").expect("Failed to write root");

    let base_dir = temp_dir.path().to_path_buf();
    let tracked_files = scan_markdown_files(&base_dir).expect("Failed to scan");
    let router = new_router(base_dir, tracked_files, true).expect("Failed to create router");
    let server = TestServer::new(router).expect("Failed to create server");

    let response = server.get("/api/files").await;

    assert_eq!(response.status_code(), 200);

    let json = response.json::<serde_json::Value>();
    let files = json["files"].as_array().expect("files should be an array");

    assert_eq!(files.len(), 3);

    // Check file paths
    let paths: Vec<String> = files.iter()
        .map(|f| f["path"].as_str().unwrap().to_string())
        .collect();

    assert!(paths.contains(&"folder1/file1.md".to_string()) || paths.contains(&"folder1\\file1.md".to_string()));
    assert!(paths.contains(&"folder2/file2.md".to_string()) || paths.contains(&"folder2\\file2.md".to_string()));
    assert!(paths.contains(&"root.md".to_string()));
}

// ============================================================================
// React API Tests - GET /api/files/:path
// ============================================================================

#[tokio::test]
async fn test_api_get_file_returns_markdown_content() {
    let (server, _temp_dir) = create_directory_server().await;

    let response = server.get("/api/files/test1.md").await;

    assert_eq!(response.status_code(), 200);

    let json = response.json::<serde_json::Value>();
    assert_eq!(json["markdown"], TEST_FILE_1_CONTENT);
}

#[tokio::test]
async fn test_api_get_file_nested_folder() {
    let temp_dir = tempdir().expect("Failed to create temp dir");

    let folder1 = temp_dir.path().join("folder1");
    fs::create_dir(&folder1).expect("Failed to create folder1");
    fs::write(folder1.join("nested.md"), "# Nested File").expect("Failed to write nested");

    let base_dir = temp_dir.path().to_path_buf();
    let tracked_files = scan_markdown_files(&base_dir).expect("Failed to scan");
    let router = new_router(base_dir, tracked_files, true).expect("Failed to create router");
    let server = TestServer::new(router).expect("Failed to create server");

    // Try both path separators
    let response = server.get("/api/files/folder1/nested.md").await;

    if response.status_code() != 200 {
        // Try Windows-style path
        let response = server.get("/api/files/folder1\\nested.md").await;
        assert_eq!(response.status_code(), 200);
        let json = response.json::<serde_json::Value>();
        assert_eq!(json["markdown"], "# Nested File");
    } else {
        let json = response.json::<serde_json::Value>();
        assert_eq!(json["markdown"], "# Nested File");
    }
}

#[tokio::test]
async fn test_api_get_file_not_found() {
    let (server, _temp_dir) = create_directory_server().await;

    let response = server.get("/api/files/nonexistent.md").await;

    assert_eq!(response.status_code(), 404);
}

// ============================================================================
// React API Tests - PUT /api/files/:path
// ============================================================================

#[tokio::test]
async fn test_api_update_file_success() {
    let (server, temp_dir) = create_directory_server().await;

    let update_payload = serde_json::json!({
        "markdown": "# Updated Content\n\nThis is new content"
    });

    let response = server.put("/api/files/test1.md")
        .json(&update_payload)
        .await;

    assert_eq!(response.status_code(), 200);

    let json = response.json::<serde_json::Value>();
    assert_eq!(json["success"], true);

    // Verify file was actually updated on disk
    let file_content = fs::read_to_string(temp_dir.path().join("test1.md"))
        .expect("Failed to read updated file");
    assert_eq!(file_content, "# Updated Content\n\nThis is new content");
}

#[tokio::test]
async fn test_api_update_file_checkbox() {
    let temp_dir = tempdir().expect("Failed to create temp dir");

    let checkbox_content = "# Todo List\n\n- [ ] Task 1\n- [ ] Task 2\n- [ ] Task 3";
    fs::write(temp_dir.path().join("todo.md"), checkbox_content).expect("Failed to write todo");

    let base_dir = temp_dir.path().to_path_buf();
    let tracked_files = scan_markdown_files(&base_dir).expect("Failed to scan");
    let router = new_router(base_dir.clone(), tracked_files, true).expect("Failed to create router");
    let server = TestServer::new(router).expect("Failed to create server");

    // Update first checkbox to checked
    let updated_content = "# Todo List\n\n- [x] Task 1\n- [ ] Task 2\n- [ ] Task 3";
    let update_payload = serde_json::json!({
        "markdown": updated_content
    });

    let response = server.put("/api/files/todo.md")
        .json(&update_payload)
        .await;

    assert_eq!(response.status_code(), 200);

    // Verify file was updated
    let file_content = fs::read_to_string(temp_dir.path().join("todo.md"))
        .expect("Failed to read updated file");
    assert_eq!(file_content, updated_content);
}

#[tokio::test]
async fn test_api_update_file_triggers_websocket_reload() {
    let (server, temp_dir) = create_directory_server_with_http().await;

    let mut websocket = server.get_websocket("/ws").await.into_websocket().await;

    // Update file via API
    let update_payload = serde_json::json!({
        "markdown": "# Updated via API"
    });

    let response = server.put("/api/files/test1.md")
        .json(&update_payload)
        .await;

    assert_eq!(response.status_code(), 200);

    // Should receive reload signal via WebSocket
    let update_result = tokio::time::timeout(
        Duration::from_secs(WEBSOCKET_TIMEOUT_SECS),
        websocket.receive_json::<ServerMessage>(),
    )
    .await;

    match update_result {
        Ok(update_message) => {
            assert_eq!(update_message, ServerMessage::Reload);
        }
        Err(_) => {
            panic!("Timeout waiting for WebSocket reload after API update");
        }
    }

    // Verify file was actually updated
    let file_content = fs::read_to_string(temp_dir.path().join("test1.md"))
        .expect("Failed to read updated file");
    assert_eq!(file_content, "# Updated via API");
}

#[tokio::test]
async fn test_api_update_file_not_found() {
    let (server, _temp_dir) = create_directory_server().await;

    let update_payload = serde_json::json!({
        "markdown": "# New Content"
    });

    let response = server.put("/api/files/nonexistent.md")
        .json(&update_payload)
        .await;

    assert_eq!(response.status_code(), 500);

    let json = response.json::<serde_json::Value>();
    assert!(json["error"].as_str().unwrap().contains("not found"));
}
