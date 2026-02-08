-- Users table (parent)
CREATE TABLE IF NOT EXISTS users (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	discord_id TEXT NOT NULL UNIQUE
);

-- Riot-specific user info (child of users)
CREATE TABLE IF NOT EXISTS riot_users (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	user_id INTEGER NOT NULL UNIQUE,
	riot_id TEXT NOT NULL UNIQUE,
	puuid TEXT UNIQUE,
	region TEXT NOT NULL,
	access_token TEXT,
	refresh_token TEXT,
	token_expires_at DATETIME,
	last_synced_at DATETIME,
	FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE
);

-- Indexes to speed lookups
CREATE INDEX IF NOT EXISTS idx_riot_users_riot_id ON riot_users(riot_id);
CREATE INDEX IF NOT EXISTS idx_riot_users_puuid ON riot_users(puuid);
