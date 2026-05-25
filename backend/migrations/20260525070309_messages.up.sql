-- Add up migration script here
CREATE TABLE messages (
    uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    conversation_id UUID NOT NULL,
    sender_id TEXT NOT NULL,
    message TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),

    CONSTRAINT fk_conversation FOREIGN KEY (conversation_id)
        REFERENCES conversations(uuid) ON DELETE CASCADE,

    CONSTRAINT fk_sender FOREIGN KEY (sender_id)
        REFERENCES users(uuid) ON DELETE CASCADE
);
