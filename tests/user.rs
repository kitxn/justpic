use justpic_backend::database::schemas::users::User;

#[tokio::test]
async fn insert_user_in_db() {
    let mut conn = justpic_backend::database::sqlite::open_in_memory_db()
        .await
        .unwrap();

    let user = User::new("john_doe".to_string(), "hunter42".to_string());
    let id = user.id();

    justpic_backend::database::repositories::users::insert(&user, &mut conn)
        .await
        .unwrap();

    let inserted = justpic_backend::database::repositories::users::fetch_by_id(id, &mut conn)
        .await
        .unwrap()
        .expect("User not inserted in db");

    assert_eq!(user.username(), inserted.username());
    assert!(matches!(
        user.role(),
        justpic_backend::database::schemas::users::UserRole::Regular
    ));
}

#[tokio::test]
async fn insert_many_users_in_db() {
    todo!()
}

#[tokio::test]
async fn insert_user_with_username_conflict_in_db() {
    todo!()
}

#[tokio::test]
async fn fetch_user_by_username() {
    todo!()
}
