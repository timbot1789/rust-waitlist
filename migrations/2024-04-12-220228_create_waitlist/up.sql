CREATE TABLE waitlist_entries IF NOT EXISTS(
    email text PRIMARY KEY NOT NULL,
    first_name text,
    last_name text,
    notes text 
);

