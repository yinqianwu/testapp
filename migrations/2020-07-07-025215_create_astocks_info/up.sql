CREATE TABLE astocks_info (
    id SERIAL PRIMARY KEY,
    stock_id TEXT NOT NULL,
    cn_name TEXT NOT NULL,
    en_name TEXT NOT NULL,
    description TEXT NOT NULL,
    head_location TEXT NOT NULL,
    founded_at TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);