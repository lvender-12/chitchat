-- Add up migration script here
CREATE TABLE conversations (
    uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user1_id TEXT NOT NULL,
    user2_id TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),

    CONSTRAINT fk_user1 FOREIGN KEY (user1_id) REFERENCES users(uuid) ON DELETE CASCADE,
    CONSTRAINT fk_user2 FOREIGN KEY (user2_id) REFERENCES users(uuid) ON DELETE CASCADE,

    -- FIX: pastikan urutan konsisten
    CONSTRAINT unique_pair UNIQUE (user1_id, user2_id)
);
