-- Add up migration script here
CREATE TABLE friends (
    uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id TEXT NOT NULL,
    friend_id TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),

    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(uuid) ON DELETE CASCADE,
    CONSTRAINT fk_friend FOREIGN KEY (friend_id) REFERENCES users(uuid) ON DELETE CASCADE,

    CONSTRAINT unique_friendship UNIQUE (user_id, friend_id)
);
