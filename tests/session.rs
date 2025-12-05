use justpic_backend::database::{
    repositories,
    schemas::{sessions::DbSession, users::DbUser},
    sqlite::open_in_memory_db,
};

#[tokio::test]
/// Checking the insertion and
/// retrieval of a session entity
/// from the database
async fn insert_session_in_db() {
    let mut conn = open_in_memory_db().await.unwrap();

    let user = DbUser::new("john_doe".to_string(), "test-password".to_string());

    repositories::users::insert(&user, &mut conn).await.unwrap();

    let session = DbSession::new(*user.id(), None);

    repositories::sessions::insert(&session, &mut conn)
        .await
        .unwrap();

    let inserted = repositories::sessions::fetch_by_id(&session.id(), &mut conn)
        .await
        .unwrap()
        .expect("User not inserted in db");

    assert_eq!(*user.id(), inserted.owner_id());
}
