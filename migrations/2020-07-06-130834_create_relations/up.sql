CREATE TABLE relations (
    id SERIAL PRIMARY KEY,
    article_id VARCHAR(100) NOT NULL,
    section_tag INT NOT NULL,
    event_tag INT NOT NULL,
    stakeholder INT NOT NULL,
    relationship TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);