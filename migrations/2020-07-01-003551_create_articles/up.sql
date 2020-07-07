CREATE TABLE articles (
    id SERIAL NOT NULL PRIMARY KEY,
    slug TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    body TEXT NOT NULL,
    market_type INT NOT NULL,
    media_source TEXT NOT NULL,
    section_tag_list INT[] NOT NULL,
    event_tag_list INT[] NOT NULL,
    stakeholder_list TEXT[] NOT NULL,
    occurred_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);