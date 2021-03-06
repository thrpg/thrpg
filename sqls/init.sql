/*
PostgreSQL's SQL file
*/
CREATE DATABASE thrpg;

\connect thrpg

CREATE TABLE userdata (
	user_id 	text 	NOT NULL PRIMARY KEY,
	player 		text,
	level 		bigint,
	exp 		bigint,
	battle_uuid Uuid
);

CREATE TABLE playdata (
	battle_uuid 	Uuid 	NOT NULL PRIMARY KEY,
    player 		Json,
    enemy 		Json,
    elapesd_turns 	bigint,
    start_time 	timestamp,
    start_turn 	bigint,
	play_mode Text
);
