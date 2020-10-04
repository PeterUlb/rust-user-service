CREATE TABLE sessions (
  id uuid PRIMARY KEY,
  user_id BIGINT NOT NULL,
  platform VARCHAR(255) NOT NULL,  
  sub_platform VARCHAR(255) NOT NULL,
  refreshed_at TIMESTAMP WITH TIME ZONE NOT NULL,
  expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
  status INTEGER NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);
CREATE INDEX user_id_idx ON sessions (user_id);

CREATE TRIGGER set_update_timestamp
BEFORE UPDATE ON sessions
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_update_timestamp();