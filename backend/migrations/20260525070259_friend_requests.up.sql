-- Add up migration script here
DO $$ BEGIN
    CREATE TYPE friend_status AS ENUM ('pending', 'accepted', 'rejected');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

CREATE TABLE friend_requests (
    uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    from_user_id TEXT NOT NULL,
    to_user_id TEXT NOT NULL,
    status friend_status DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    CONSTRAINT fk_from FOREIGN KEY (from_user_id) REFERENCES users(uuid) ON DELETE CASCADE,
    CONSTRAINT fk_to FOREIGN KEY (to_user_id) REFERENCES users(uuid) ON DELETE CASCADE,

    CONSTRAINT unique_request UNIQUE (from_user_id, to_user_id)
);
