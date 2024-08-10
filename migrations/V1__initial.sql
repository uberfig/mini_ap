CREATE TABLE internal_users (
	uid 				BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
	password			TEXT NOT NULL, --stored with argon2
	preferred_username	TEXT NOT NULL UNIQUE, --basically the username/login name
	display_name		TEXT NULL,
	email				TEXT NULL,
	summary				TEXT NULL, -- used as a user's bio
	private_key_pem		TEXT NOT NULL,
	public_key_pem		TEXT NOT NULL,
	permission_level 	SMALLINT NOT NULL,
	manual_followers	BOOLEAN NOT NULL DEFAULT false, -- manually approves followers
	custom_domain		TEXT NULL
);

CREATE TABLE ap_instance_actor (
	private_key			TEXT NOT NULL,
	public_key_pem		TEXT NOT NULL
);

-- federated activitypub users, doesn't include internal
CREATE TABLE federated_ap_users (
	ap_user_id			BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
	id					TEXT NOT NULL UNIQUE,
	type_field			TEXT NOT NULL DEFAULT 'Person',
	preferred_username	TEXT NOT NULL,
	domain				TEXT NOT NULL,
	name				TEXT NULL, --their display name
	summary				TEXT NULL,
	url					TEXT NULL,
	public_key_pem		TEXT NOT NULL,

	-- links
	inbox				TEXT NOT NULL,
	outbox				TEXT NOT NULL,
	followers			TEXT NOT NULL,
	following			TEXT NOT NULL,

	manual_followers	BOOLEAN NOT NOT DEFAULT false,
	memorial			BOOLEAN NOT NOT DEFAULT false,
	indexable			BOOLEAN NOT NOT DEFAULT false,
	discoverable		BOOLEAN NOT NOT DEFAULT false
	-- featured			TEXT,
	-- featuredTags		TEXT,
);

CREATE TABLE public_keys (
	pub_key_id			BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
	id					TEXT NOT NULL UNIQUE,
	owner				TEXT NOT NULL UNIQUE REFERENCES activitypub_users(id) ON DELETE CASCADE,
	public_key_pem		TEXT NOT NULL
);


