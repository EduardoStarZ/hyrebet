-- Your SQL goes here
CREATE TABLE "posts"(
	"id" INTEGER NOT NULL PRIMARY KEY,
	"reply" INTEGER,
	"repost" INTEGER,
	"owner" VARCHAR NOT NULL,
	"contents" TEXT NOT NULL,
	"likes" INTEGER NOT NULL,
	"time" TIMESTAMPTZ NOT NULL
);

