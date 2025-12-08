// use justpic_backend::database::{
//     repositories,
//     schemas::users::{DbUser, UserRole},
//     sqlite::open_in_memory_db,
// };

// #[tokio::test]
// /// Checking the insertion and
// /// retrieval of a user entity
// /// from the database
// async fn insert_user_in_db() {
//     let mut conn = open_in_memory_db().await.unwrap();

//     let user = DbUser::new("john_doe".to_string(), "test-password".to_string());
//     let id = user.id();

//     repositories::users::insert(&user, &mut conn).await.unwrap();

//     let inserted = repositories::users::fetch_by_id(id, &mut conn)
//         .await
//         .unwrap()
//         .expect("User not inserted in db");

//     assert_eq!(user.username(), inserted.username());
//     assert!(matches!(user.role(), UserRole::Regular));
// }

// #[tokio::test]
// /// Test inserting multiple
// /// different user entities
// /// and finding one
// async fn insert_many_users_in_db() {
//     let mut conn = open_in_memory_db().await.unwrap();

//     let user = DbUser::new("john_doe".to_string(), "hunter42".to_string());
//     let user2 = DbUser::new("not_john_doe".to_string(), "hunter52".to_string());
//     let user3 = DbUser::new("big_john_doe".to_string(), "hunter32".to_string());

//     repositories::users::insert(&user, &mut conn).await.unwrap();
//     repositories::users::insert(&user2, &mut conn)
//         .await
//         .unwrap();
//     repositories::users::insert(&user3, &mut conn)
//         .await
//         .unwrap();

//     let inserted = repositories::users::fetch_by_id(user2.id(), &mut conn)
//         .await
//         .unwrap()
//         .expect("User not inserted in db");

//     assert_eq!(user2.username(), inserted.username());
//     assert!(matches!(user2.role(), UserRole::Regular));
// }

// #[tokio::test]
// /// Checking the insertion
// /// of a user with a taken nickname
// async fn insert_user_with_username_conflict_in_db() {
//     let mut conn = open_in_memory_db().await.unwrap();

//     let user = DbUser::new("john_doe".to_string(), "hunter42".to_string());
//     let user2 = DbUser::new("john_doe".to_string(), "hunter52".to_string());

//     repositories::users::insert(&user, &mut conn).await.unwrap();
//     assert!(
//         repositories::users::insert(&user2, &mut conn)
//             .await
//             .is_err()
//     );
// }

// #[tokio::test]
// /// Insert and search
// /// for a user by name
// async fn fetch_user_by_username() {
//     let mut conn = open_in_memory_db().await.unwrap();

//     let user = DbUser::new("john_doe".to_string(), "test-password".to_string());

//     repositories::users::insert(&user, &mut conn).await.unwrap();

//     let inserted = repositories::users::fetch_by_username(user.username(), &mut conn)
//         .await
//         .unwrap()
//         .expect("User not inserted in db");

//     assert_eq!(user.id(), inserted.id());
// }
