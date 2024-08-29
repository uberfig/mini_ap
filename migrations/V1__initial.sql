CREATE TABLE internal_users (
	local_id 			BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
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
	private_key_pem		TEXT NOT NULL,
	public_key_pem		TEXT NOT NULL
);

----------------------------------------------------------------

CREATE TABLE federated_instances (
	instance_id			BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
	domain				TEXT NOT NULL UNIQUE,
	blocked				BOOLEAN NOT NULL DEFAULT false,
	allowlisted			BOOLEAN NOT NULL DEFAULT false,
	software			TEXT NULL,
	favicon				BYTEA NULL
);

-- federated activitypub users, doesn't include internal
CREATE TABLE federated_ap_users (
	fedi_id			BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
	id					TEXT NOT NULL UNIQUE,
	type_field			TEXT NOT NULL DEFAULT 'Person',
	preferred_username	TEXT NOT NULL,
	domain				TEXT NOT NULL,
	name				TEXT NULL, --their display name
	summary				TEXT NULL,
	url					TEXT NULL,
	public_key_pem		TEXT NOT NULL,
	public_key_id		TEXT NOT NULL,

	-- links
	inbox				TEXT NOT NULL,
	outbox				TEXT NOT NULL,
	followers			TEXT NOT NULL,
	following			TEXT NOT NULL,

	manual_followers	BOOLEAN NOT NULL DEFAULT false,
	memorial			BOOLEAN NOT NULL DEFAULT false,
	indexable			BOOLEAN NOT NULL DEFAULT false,
	discoverable		BOOLEAN NOT NULL DEFAULT false
	-- featured			TEXT,
	-- featuredTags		TEXT,
);

CREATE TABLE unified_users (
	uid			BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
	is_local	BOOLEAN NOT NULL,
	fedi_id		BIGINT NULL REFERENCES federated_ap_users(fedi_id) ON DELETE CASCADE,
	local_id	BIGINT NULL REFERENCES internal_users(local_id) ON DELETE CASCADE
);

CREATE TABLE following (
	-- the user that is following
	follower		BIGINT NOT NULL REFERENCES unified_users(uid) ON DELETE CASCADE,
	-- the user that is being followed
	target_user		BIGINT NOT NULL REFERENCES unified_users(uid) ON DELETE CASCADE,
	pending			BOOLEAN NOT NULL DEFAULT true,
	published		BIGINT NOT NULL,
	PRIMARY KEY(fedi_from, local_from, target_fedi, target_local)
);

CREATE TABLE posts (
	obj_id		BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
	is_local	BOOLEAN NOT NULL,
	fedi_id		TEXT NULL UNIQUE,	--not used for internal posts
	surtype		TEXT NOT NULL,
	subtype		TEXT NOT NULL,

	likes		BIGINT NOT NULL DEFAULT 0,
	-- local_post	BOOLEAN NOT NULL, -- created by a local user
	local_only	BOOLEAN NOT NULL DEFAULT false,
	published	BIGINT NOT NULL,
	in_reply_to	BIGINT NULL REFERENCES posts(obj_id) ON DELETE SET NULL,
	
	block_replies BOOLEAN NOT NULL DEFAULT false,
	restrict_replies BOOLEAN NOT NULL DEFAULT false, --only those followed by or mentoned by the creator can comment
	local_only_replies BOOLEAN NOT NULL DEFAULT false,

	content		TEXT NULL,
	domain		TEXT NOT NULL,
	-- REFERENCES federated_instances(domain) ON DELETE CASCADE,

	-- used for questions
	multi_select 		BOOLEAN NULL,
	options				TEXT NULL, -- the array of json options in text
	closed				BIGINT NULL,
	local_only_voting 	BOOLEAN NULL,

	actor	BIGINT NULL REFERENCES unified_users(uid) ON DELETE CASCADE
);

CREATE TABLE likes (
	actor		BIGINT NULL REFERENCES unified_users(uid) ON DELETE CASCADE,
	post 		BIGINT NOT NULL REFERENCES posts(obj_id) ON DELETE CASCADE,
	published	BIGINT NOT NULL,
	PRIMARY KEY(fedi_actor, local_actor, post)
);
