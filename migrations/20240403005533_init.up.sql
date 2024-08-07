CREATE TABLE activitypub_users (
	ap_user_id			BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
	id					TEXT NOT NULL UNIQUE,
	-- local				BOOLEAN NOT NULL,
	type_field			TEXT NOT NULL,
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

CREATE TABLE instance_actor (
	private_key			TEXT NOT NULL,
	public_key_pem		TEXT NOT NULL
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

CREATE TABLE following (
	actor		TEXT NOT NULL REFERENCES activitypub_users(id) ON DELETE CASCADE,
	following	TEXT NOT NULL REFERENCES activitypub_users(id) ON DELETE CASCADE,
	PRIMARY KEY (actor, following)
);

CREATE TABLE objects (
	obj_id		BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
	id			TEXT NULL UNIQUE,
	domain		TEXT NOT NULL,

	internal_type			TEXT NOT NULL, -- can be Object, Question, or whatever is represented
	activitystream_type		TEXT NOT NULL, 

	ap_user_id	BIGINT NOT NULL REFERENCES activitypub_users(ap_user_id) ON DELETE CASCADE, -- used to represent the owner
	published BIGINT NOT NULL --timestamp in milis
	-- obj_id	BIGINT NULL REFERENCES activity_objects(obj_id) ON DELETE CASCADE
);

CREATE TABLE files (
	file_id 		BIGSERIAL PRIMARY KEY NOT NULL UNIQUE
);

CREATE TABLE activity_objects (
	obj_id		BIGINT PRIMARY KEY NOT NULL UNIQUE REFERENCES objects(obj_id) ON DELETE CASCADE,

	type_field		TEXT NOT NULL DEFAULT 'Note',
	id				TEXT NOT NULL UNIQUE,
	name			TEXT NULL,
	-- attachment
	attributedTo	TEXT NOT NULL REFERENCES activitypub_users(id) ON DELETE CASCADE,
	content			TEXT,
	in_reply_to		TEXT NULL REFERENCES objects(id),
	
	published		BIGINT NOT NULL
);

CREATE TABLE attachments (
	obj_id			BIGINT NOT NULL REFERENCES activity_objects(obj_id) ON DELETE CASCADE,
	attach_id 		BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
	type_field		TEXT NOT NULL DEFAULT 'Image',
	content			TEXT NULL,
	url				TEXT NULL,
	file_id			BIGINT NOT NULL REFERENCES files(file_id) ON DELETE CASCADE
);



