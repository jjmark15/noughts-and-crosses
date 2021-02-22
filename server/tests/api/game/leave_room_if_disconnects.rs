use spectral::prelude::*;

use crate::helpers::{app_client, create_user};

#[tokio::test]
async fn leaves_room_if_client_disconnects() {
    let mut app_client = app_client();
    let user_id = create_user(&app_client).await;
    app_client.join_new_room(user_id).await.unwrap();

    app_client.close_socket_connection().await;

    asserting("User is not already in a room")
        .that(&app_client.join_new_room(user_id).await)
        .is_ok();
    app_client.close_socket_connection().await;
}
