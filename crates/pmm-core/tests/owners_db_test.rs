// ./pmm-core/tests/owners_db_test.rs

use pmm_core::database::sqlite_db::Database;
use pmm_core::modules::owners::{Owner, UserGroup};

use dotenvy::dotenv;

async fn init_db_for_tests() -> Database {
    // Load environment variables
    dotenv().ok();
    let db_path = pmm_utils::general::get_root_wd().join("data/database/db.sqlite");
    let db_url = format!("sqlite:{}", db_path.to_str().unwrap());

    // Initialize database
    let db = Database::new(&db_url)
        .await
        .expect("Failed to connect to DB");
    db.init_db().await.expect("Failed to init DB");
    db
}

#[tokio::test]
async fn test_connect_to_database() {
    // Load environment variables
    dotenv().ok();
    let db_path = pmm_utils::general::get_root_wd().join("data/database/db.sqlite");
    let db_url = format!("sqlite:{}", db_path.to_str().unwrap());

    // Initialize database
    let db = Database::new(&db_url)
        .await
        .expect("Failed to connect to DB");
    db.init_db().await.expect("Failed to init DB");
}

#[tokio::test]
async fn test_create_owner() {
    let db = init_db_for_tests().await;
    // Create new owner
    let owner = Owner::new(
        "dafer".to_string(),
        "dani@gmail.com".to_string(),
        "123abc".to_string(),
        Some(UserGroup::Trader),
        None,
    );

    let _ = Owner::create_owner(&db, &owner).await;
}

#[tokio::test]
async fn test_fetch_all_owners() {
    let db = init_db_for_tests().await;
    // Fetch all owners
    let owners = Owner::get_all_owners(&db)
        .await
        .expect("Failed to get owners");

    println!("Database owners: {:?}", owners);

    // ✅ Check that at least one owner exists
    assert!(
        owners.iter().any(|o| o.email == "dani@gmail.com"),
        "Owner was not found in DB"
    );
}
