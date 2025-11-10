-- Your SQL goes here
CREATE TABLE "posts"(
	"id" INTEGER NOT NULL,
	"reply" INTEGER,
	"repost" INTEGER,
	"owner" VARCHAR NOT NULL,
	"contents" TEXT NOT NULL,
	"likes" INTEGER NOT NULL,
	"time" TIMESTAMPTZ NOT NULL,
	PRIMARY KEY ("id", "owner")
);

