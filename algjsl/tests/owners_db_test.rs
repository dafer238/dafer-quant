use algjsl::database::sqlite_db::Database;
use algjsl::modules::owners::{Owner, UserGroup};

use dotenvy::dotenv;

#[tokio::test]
async fn test_create_and_fetch_owner() {
    // Load environment variables
    dotenv().ok();
    let db_path = utils::general::get_root_wd().join("data/database/db.sqlite");
    let db_url = format!("sqlite:{}", db_path.to_str().unwrap());

    // Initialize database
    let db = Database::new(&db_url)
        .await
        .expect("Failed to connect to DB");
    db.init_db().await.expect("Failed to init DB");

    // Create new owner
    let owner = Owner::new(
        "dafer".to_string(),
        "dani@gmail.com".to_string(),
        "123abc".to_string(),
        Some(UserGroup::Trader),
        None,
    );

    Owner::create_owner(&db, &owner)
        .await
        .expect("Failed to create owner");

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
