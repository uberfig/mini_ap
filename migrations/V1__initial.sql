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

CREATE TABLE following (
	follow_type			SMALLINT NOT NULL, -- local to local, local to federated, federated to local
	creator				BIGINT NOT NULL, -- the person trying to follow
	target_user			BIGINT NOT NULL, -- the person to be followed
	pending				BOOLEAN NOT NULL DEFAULT true,
	PRIMARY KEY(follow_type, creator, target_user)
);

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

CREATE TABLE posts (
	obj_id		BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
	id			TEXT NULL UNIQUE,	--not used for internal posts
	surtype		TEXT NOT NULL,
	subtype		TEXT NULL,
	local_post	BOOLEAN NOT NULL, -- created by a local user
	local_only	BOOLEAN NOT NULL DEFAULT false,
	published	BIGINT NOT NULL,
	in_reply_to	BIGINT NOT NULL REFERENCES posts(obj_id),
	
	block_replies BOOLEAN NOT NULL DEFAULT false,
	restrict_replies BOOLEAN NOT NULL DEFAULT false, --only those followed by or mentoned by the creator can comment
	local_only_replies BOOLEAN NOT NULL DEFAULT false,

	content		TEXT NULL,

	-- used for questions
	multi_select 		BOOLEAN NULL,
	options				TEXT NULL, -- the array of json options in text
	closed				BIGINT NULL,
	local_only_voting 	BOOLEAN NULL,

	fedi_actor	BIGINT NULL REFERENCES federated_ap_users(ap_user_id) ON DELETE CASCADE,
	local_actor	BIGINT NULL REFERENCES internal_users(uid) ON DELETE CASCADE
);
