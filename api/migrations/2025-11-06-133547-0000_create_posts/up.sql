-- Your SQL goes here
CREATE TABLE "posts"(
	"id" INTEGER NOT NULL,
	"reply" VARCHAR,
	"repost" VARCHAR,
	"owner" VARCHAR NOT NULL,
	"contents" TEXT NOT NULL,
	"total_likes" INTEGER NOT NULL,
	"time" TIMESTAMPTZ NOT NULL,
	PRIMARY KEY ("id", "owner")
);

CREATE TABLE "likes" (
	"route" VARCHAR NOT NULL,
	"user" VARCHAR NOT NULL,
	PRIMARY KEY ("route", "user")
);
