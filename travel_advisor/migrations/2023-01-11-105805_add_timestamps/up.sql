ALTER TABLE comments 
    ADD updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP(),
    ADD created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP();