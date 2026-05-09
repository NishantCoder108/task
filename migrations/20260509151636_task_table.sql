create table task (
    -- Define an auto-incrementing integer column named 'id' that is the primary key of the table
    id SERIAL PRIMARY KEY,
    
    -- Define a string column named 'title' with a maximum length of 255 characters that cannot be null
    title VARCHAR(255) NOT NULL,
    
    -- Define a timestamp column named 'created_at' that cannot be null and sets its default value to the current timestamp
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Define a timestamp column named 'updated_at' that cannot be null and sets its default value to the current timestamp
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)