-- Content tables
CREATE TABLE cards (
  id          VARCHAR(32)   PRIMARY KEY,
  file_key    VARCHAR(32)   NOT NULL REFERENCES files(id) ON DELETE CASCADE,

  owner_id    UUID          NOT NULL REFERENCES users(id) ON DELETE CASCADE,

  title       VARCHAR(255),
  description VARCHAR(2048),

  created     TIMESTAMPTZ   NOT NULL  DEFAULT CURRENT_TIMESTAMP,
  
  source_url  TEXT,
  
  is_private  BOOLEAN       NOT NULL  DEFAULT 0
);

CREATE TABLE files (
  id          VARCHAR(32)   PRIMARY KEY,

  uploader_id UUID          NOT NULL REFERENCES users(id) ON DELETE CASCADE,

  mimetype    VARCHAR(255)  NOT NULL  DEFAULT 'application/octet-stream',
  filesize    BIGINT        NOT NULL  DEFAULT 0,

  width       INTEGER       NOT NULL  DEFAULT 800,
  height      INTEGER       NOT NULL  DEFAULT 600,

  created     TIMESTAMPTZ   NOT NULL  DEFAULT CURRENT_TIMESTAMP,

  color       VARCHAR(6)    NOT NULL  DEFAULT 'ffffff',

  state       VARCHAR(32)   NOT NULL  DEFAULT 'pending'
);

-- Users tables
CREATE TABLE users (
  id          UUID          PRIMARY KEY,

  username    VARCHAR(255)  UNIQUE NOT NULL,
  password    VARCHAR(1024) NOT NULL,

  role        VARCHAR(25)   NOT NULL  DEFAULT 'regular',

  created     TIMESTAMPTZ   NOT NULL  DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE sessions (
  id          UUID          PRIMARY KEY,
  
  owner_id    UUID          NOT NULL REFERENCES users(id) ON DELETE CASCADE,

  -- USER-AGENT
  agent       VARCHAR(512),

  created     TIMESTAMPTZ   NOT NULL  DEFAULT CURRENT_TIMESTAMP,
  expires     TIMESTAMPTZ   NOT NULL
);
