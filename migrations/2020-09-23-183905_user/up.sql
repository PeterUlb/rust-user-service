CREATE TABLE users (
  id BIGSERIAL PRIMARY KEY,
  username VARCHAR(128) NOT NULL,
  email VARCHAR(255) NOT NULL,  
  password VARCHAR(255) NOT NULL,
  password_version INTEGER NOT NULL,
  date_of_birth DATE NOT NULL,
  status INTEGER NOT NULL DEFAULT 0,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);
CREATE UNIQUE INDEX username_idx ON users (username);

CREATE TRIGGER set_update_timestamp
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_update_timestamp();