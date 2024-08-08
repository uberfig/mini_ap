CREATE TABLE instance_actor (
	private_key			TEXT NOT NULL,
	public_key_pem		TEXT NOT NULL
);

-- federated activitypub users, doesn't include internal
CREATE TABLE activitypub_users (
	ap_user_id			BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
	id					TEXT NOT NULL UNIQUE,
	type_field			TEXT NOT NULL DEFAULT 'Person',
	preferred_username	TEXT NOT NULL,
	domain				TEXT NOT NULL,
	name				TEXT NULL,
	summary				TEXT NOT NULL DEFAULT '',
	inbox				TEXT NOT NULL,
	outbox				TEXT NOT NULL,
	followers			TEXT NOT NULL,
	following			TEXT NOT NULL,
	liked				TEXT NULL
	-- featured			TEXT,
	-- featuredTags		TEXT,
	-- url					TEXT,
	-- manuallyApprovesFollowers	BOOLEAN,
	-- discoverable		BOOLEAN,
	-- indexable			BOOLEAN,
	-- memorial			BOOLEAN
);

CREATE TABLE public_keys (
	pub_key_id			BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
	id					TEXT NOT NULL UNIQUE,
	owner				TEXT NOT NULL UNIQUE REFERENCES activitypub_users(id) ON DELETE CASCADE,
	public_key_pem		TEXT NOT NULL
);

CREATE TABLE internal_users (
	uid 		BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
	password	TEXT NOT NULL, --stored with argon2
	preferred_username	TEXT NOT NULL UNIQUE, --basically the username/login name
	activitypub_actor	BIGINT NOT NULL REFERENCES activitypub_users(ap_user_id) ON DELETE CASCADE,
	private_key		TEXT NOT NULL
);
