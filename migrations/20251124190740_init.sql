-- Content tables
CREATE TABLE cards (
  id          BIGINT        PRIMARY KEY,
  file_key    VARCHAR(32)   UNIQUE NOT NULL,

  title       VARCHAR(255),
  description VARCHAR(2048),

  created     TIMESTAMPTZ   NOT NULL  DEFAULT CURRENT_TIMESTAMP,
  
  source_url  TEXT,

  mimetype    VARCHAR(255)  NOT NULL  DEFAULT 'application/octet-stream',
  filesize    BIGINT        NOT NULL  DEFAULT 0,

  width       INTEGER       NOT NULL  DEFAULT 800,
  height      INTEGER       NOT NULL  DEFAULT 600,
  
  is_private  BOOLEAN       NOT NULL  DEFAULT 0,

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
