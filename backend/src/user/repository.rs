use crate::{
    app::AppState,
    entity::user_entity::User,
    errors::error::AppResult,
    user::dto::{FriendList, FriendRequestReceived, FriendRequestSent, ProfileUser},
};

pub async fn profile_repository(state: &AppState, uuid: String) -> AppResult<ProfileUser> {
    let profile = sqlx::query_as::<_, ProfileUser>(
        "SELECT uuid, name, email, created_at FROM users WHERE uuid = $1",
    )
    .bind(uuid)
    .fetch_one(&state.db)
    .await?;
    Ok(profile)
}

pub async fn find_by_uuid(state: &AppState, uuid: &String) -> AppResult<Option<User>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE uuid = $1")
        .bind(uuid)
        .fetch_optional(&state.db)
        .await?;
    Ok(user)
}

pub async fn add_friend_repository(
    state: &AppState,
    uuid: String,
    friend_uuid: String,
) -> AppResult<()> {
    sqlx::query("INSERT INTO friend_requests (from_user_id, to_user_id) VALUES ($1, $2)")
        .bind(uuid)
        .bind(friend_uuid)
        .execute(&state.db)
        .await?;
    Ok(())
}

pub async fn friend_sent_repository(
    state: &AppState,
    uuid: String,
) -> AppResult<Vec<FriendRequestSent>> {
    let friend_requests = sqlx::query_as::<_, FriendRequestSent>(
        "SELECT fr.uuid, fr.to_user_id, u.name, fr.status, fr.created_at
        FROM friend_requests fr
        JOIN users u ON u.uuid = fr.to_user_id
        WHERE fr.from_user_id = $1 AND fr.status = 'pending'",
    )
    .bind(uuid)
    .fetch_all(&state.db)
    .await?;
    Ok(friend_requests)
}

pub async fn friend_received_repository(
    state: &AppState,
    uuid: String,
) -> AppResult<Vec<FriendRequestReceived>> {
    let friend_requests = sqlx::query_as::<_, FriendRequestReceived>(
        "SELECT fr.uuid, fr.from_user_id, u.name, fr.status, fr.created_at
        FROM friend_requests fr
        JOIN users u ON u.uuid = fr.from_user_id
        WHERE fr.to_user_id = $1 AND fr.status = 'pending'",
    )
    .bind(uuid)
    .fetch_all(&state.db)
    .await?;
    Ok(friend_requests)
}

pub async fn friend_accepted_repository(
    state: &AppState,
    from_user: String,
    to_user: String,
    user1_id: &str,
    user2_id: &str,
) -> AppResult<()> {
    let mut tx = state.db.begin().await?;

    sqlx::query(
        "UPDATE friend_requests SET status = 'accepted'
            WHERE from_user_id = $1 AND to_user_id = $2 AND status = 'pending'",
    )
    .bind(from_user)
    .bind(to_user)
    .execute(&mut *tx)
    .await?;

    sqlx::query("INSERT INTO friends (user_id, friend_id) VALUES ($1, $2)")
        .bind(user1_id)
        .bind(user2_id)
        .execute(&mut *tx)
        .await?;

    sqlx::query("INSERT INTO friends (user_id, friend_id) VALUES ($1, $2)")
        .bind(user2_id)
        .bind(user1_id)
        .execute(&mut *tx)
        .await?;

    sqlx::query("INSERT INTO conversations (user1_id, user2_id) VALUES ($1, $2)")
        .bind(user1_id)
        .bind(user2_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(())
}

pub async fn friend_rejected_repository(
    state: &AppState,
    from_user: String,
    to_user: String,
) -> AppResult<()> {
    sqlx::query(
            "DELETE FROM friend_requests WHERE from_user_id = $1 AND to_user_id = $2 AND status = 'pending'",
        )
        .bind(from_user)
        .bind(to_user)
        .execute(&state.db)
        .await?;
    Ok(())
}

pub async fn all_friend_repository(state: &AppState, uuid: String) -> AppResult<Vec<FriendList>> {
    let friends = sqlx::query_as::<_, FriendList>(
        "SELECT u.uuid, u.name, u.email, u.created_at
            FROM friends f
            JOIN users u ON u.uuid = f.friend_id
            WHERE f.user_id = $1",
    )
    .bind(uuid)
    .fetch_all(&state.db)
    .await?;
    Ok(friends)
}
