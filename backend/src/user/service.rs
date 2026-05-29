use crate::{
    app::AppState,
    cache::user_cache::{get_user_cache, set_user_cache},
    errors::error::{AppError, AppResult},
    user::{
        dto::{FriendList, FriendRequestReceived, FriendRequestSent, ProfileUser, UserCache},
        repository::{
            add_friend_repository, all_friend_repository, find_by_uuid, friend_accepted_repository,
            friend_received_repository, friend_rejected_repository, friend_sent_repository,
            profile_repository,
        },
    },
};

pub async fn profile_service(state: &AppState, uuid: String) -> AppResult<ProfileUser> {
    if let Some(user) = get_user_cache(state, &uuid).await? {
        println!("REDIS HIT");
        return Ok(ProfileUser {
            uuid: user.uuid,
            name: user.name,
            email: user.email,
            created_at: user.created_at,
        });
    }
    println!("REDIS MISS");

    let profile = profile_repository(state, uuid).await?;

    let cache = UserCache {
        uuid: profile.uuid.clone(),
        name: profile.name.clone(),
        email: profile.email.clone(),
        created_at: profile.created_at,
    };

    set_user_cache(state, &cache).await?;

    Ok(profile)
}

pub async fn add_friend_service(
    state: &AppState,
    uuid: String,
    friend_uuid: String,
) -> AppResult<()> {
    if uuid == friend_uuid {
        return Err(AppError::BadRequest(
            "cannot add self as friend".to_string(),
        ));
    }

    if find_by_uuid(&state, &friend_uuid).await?.is_none() {
        return Err(AppError::NotFound("friend not found".to_string()));
    }

    add_friend_repository(&state, uuid, friend_uuid).await?;
    Ok(())
}

pub async fn friend_sent_service(
    state: &AppState,
    uuid: String,
) -> AppResult<Vec<FriendRequestSent>> {
    let user = friend_sent_repository(&state, uuid).await?;
    Ok(user)
}

pub async fn friend_received_service(
    state: &AppState,
    uuid: String,
) -> AppResult<Vec<FriendRequestReceived>> {
    let user = friend_received_repository(&state, uuid).await?;
    Ok(user)
}

pub async fn friend_accepted_service(
    state: &AppState,
    to_user: String,
    from_user: String,
) -> AppResult<()> {
    let (user1_id, user2_id) = if to_user < from_user {
        (to_user.clone(), from_user.clone())
    } else {
        (from_user.clone(), to_user.clone())
    };

    friend_accepted_repository(state, from_user, to_user, &user1_id, &user2_id).await?;
    Ok(())
}

pub async fn friend_rejected_service(
    state: &AppState,
    to_user: String,
    from_user: String,
) -> AppResult<()> {
    friend_rejected_repository(state, from_user, to_user).await?;
    Ok(())
}

pub async fn all_friend_service(state: &AppState, uuid: String) -> AppResult<Vec<FriendList>> {
    let friends = all_friend_repository(state, uuid).await?;

    let mut result = Vec::new();

    for friend in friends {
        let user = if let Some(cached) = get_user_cache(state, &friend.friend_id).await? {
            println!("REDIS HIT");

            cached
        } else {
            println!("REDIS MISS");

            let db_user = find_by_uuid(state, &friend.friend_id)
                .await?
                .ok_or(AppError::NotFound("user not found".to_string()))?;

            let cache = UserCache {
                uuid: db_user.uuid.clone(),
                name: db_user.name.clone(),
                email: db_user.email.clone(),
                created_at: db_user.created_at,
            };

            set_user_cache(state, &cache).await?;

            cache
        };

        result.push(FriendList {
            uuid: user.uuid,
            name: user.name,
            email: user.email,
            conversation_id: friend.conversation_id,
            created_at: user.created_at,
        });
    }

    Ok(result)
}
