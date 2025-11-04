-- Your SQL goes here
CREATE TABLE "users"(
	"name" VARCHAR NOT NULL PRIMARY KEY,
	"password" TEXT NOT NULL,
	"join_date" TIMESTAMPTZ NOT NULL
);

