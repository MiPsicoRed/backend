CREATE TABLE blog_posts (
    id UUID PRIMARY KEY,
    author_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE, 
    blog_post_status_id INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    summary TEXT,
    content TEXT NOT NULL,
    tags TEXT[],
    featured_image TEXT,
    reading_time_minutes INTEGER,
    view_count INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

