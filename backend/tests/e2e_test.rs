use axum_test::TestServer;
use axum::http::StatusCode;
use memory_memo::{create_app, database::create_test_database};
use serde_json::{json, Value};

#[tokio::test]
async fn test_complete_user_workflow() {
    // Setup test server
    let pool = create_test_database().await.unwrap();
    let app = create_app(pool).await.unwrap();
    let server = TestServer::new(app).unwrap();

    // Test 1: User Registration
    let register_response = server
        .post("/api/auth/register")
        .json(&json!({
            "username": "e2euser", 
            "password": "e2epass123"
        }))
        .await;
    
    register_response.assert_status(StatusCode::CREATED);
    println!("✅ User registration successful");

    // Test 2: User Login
    let login_response = server
        .post("/api/auth/login")
        .json(&json!({
            "username": "e2euser",
            "password": "e2epass123"
        }))
        .await;
    
    login_response.assert_status_ok();
    let session_cookie = login_response
        .cookies()
        .iter()
        .find(|c| c.name() == "session_id")
        .unwrap()
        .clone();
    println!("✅ User login successful");

    // Test 3: Create Forever Memo
    let forever_memo_response = server
        .post("/api/memos/forever")
        .add_cookie(session_cookie.clone())
        .json(&json!({
            "content": "This is my forever memo for E2E testing"
        }))
        .await;
    
    forever_memo_response.assert_status(StatusCode::CREATED);
    let forever_memo: Value = forever_memo_response.json();
    let forever_memo_id = forever_memo["id"].as_str().unwrap();
    println!("✅ Forever memo created: {}", forever_memo_id);

    // Test 4: Create Flush Memo
    let flush_memo_response = server
        .post("/api/memos/flush")
        .add_cookie(session_cookie.clone())
        .json(&json!({
            "content": "This is my flush memo for E2E testing"
        }))
        .await;
    
    flush_memo_response.assert_status(StatusCode::CREATED);
    let flush_memo: Value = flush_memo_response.json();
    let flush_memo_id = flush_memo["id"].as_str().unwrap();
    println!("✅ Flush memo created: {}", flush_memo_id);

    // Test 5: List All Memos
    let list_response = server
        .get("/api/memos")
        .add_cookie(session_cookie.clone())
        .await;
    
    list_response.assert_status_ok();
    let memos_list: Value = list_response.json();
    
    // Verify both memos are in the list
    assert_eq!(memos_list["forever_memos"].as_array().unwrap().len(), 1);
    assert_eq!(memos_list["flush_memos"].as_array().unwrap().len(), 1);
    
    assert!(memos_list["forever_memos"][0]["content"]
        .as_str().unwrap().contains("forever memo"));
    assert!(memos_list["flush_memos"][0]["content"]
        .as_str().unwrap().contains("flush memo"));
    println!("✅ Memo listing successful");

    // Test 6: Update Forever Memo
    let update_response = server
        .put(&format!("/api/memos/forever/{}", forever_memo_id))
        .add_cookie(session_cookie.clone())
        .json(&json!({
            "content": "Updated forever memo content"
        }))
        .await;
    
    update_response.assert_status_ok();
    let updated_memo: Value = update_response.json();
    assert_eq!(updated_memo["content"], "Updated forever memo content");
    println!("✅ Forever memo update successful");

    // Test 7: Delete Flush Memo
    let delete_response = server
        .delete(&format!("/api/memos/flush/{}", flush_memo_id))
        .add_cookie(session_cookie.clone())
        .await;
    
    delete_response.assert_status(StatusCode::NO_CONTENT);
    println!("✅ Flush memo deletion successful");

    // Test 8: Verify Memo Deletion
    let final_list_response = server
        .get("/api/memos")
        .add_cookie(session_cookie.clone())
        .await;
    
    final_list_response.assert_status_ok();
    let final_memos: Value = final_list_response.json();
    
    // Should have 1 forever memo and 0 flush memos
    assert_eq!(final_memos["forever_memos"].as_array().unwrap().len(), 1);
    assert_eq!(final_memos["flush_memos"].as_array().unwrap().len(), 0);
    
    // Verify updated content
    assert_eq!(
        final_memos["forever_memos"][0]["content"],
        "Updated forever memo content"
    );
    println!("✅ Final verification successful");

    // Test 9: Logout
    let logout_response = server
        .post("/api/auth/logout")
        .add_cookie(session_cookie.clone())
        .await;
    
    logout_response.assert_status_ok();
    println!("✅ User logout successful");

    // Test 10: Verify Logout (should be unauthorized)
    let unauthorized_response = server
        .get("/api/memos")
        .add_cookie(session_cookie)
        .await;
    
    unauthorized_response.assert_status_unauthorized();
    println!("✅ Authorization check after logout successful");

    println!("\n🎉 Complete E2E workflow test passed!");
    println!("✅ Registration → Login → Create Memos → List → Update → Delete → Logout");
}