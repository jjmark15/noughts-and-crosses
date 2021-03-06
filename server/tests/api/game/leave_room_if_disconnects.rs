use spectral::prelude::*;

use crate::helpers::{app_client, create_room, create_user};

#[tokio::test]
async fn leaves_room_if_client_disconnects() {
    let mut app_client = app_client();
    let user_id = create_user(&app_client).await;
    let room_id = create_room(&app_client, user_id).await;
    app_client.join_room(user_id, room_id).await.unwrap();

    app_client.close_socket_connection().await;

    let other_room_id = create_room(&app_client, user_id).await;

    asserting("User is not already in a room")
        .that(&app_client.join_room(user_id, other_room_id).await)
        .is_ok();
    app_client.close_socket_connection().await;
}
