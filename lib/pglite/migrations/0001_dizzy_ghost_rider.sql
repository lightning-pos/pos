CREATE TABLE IF NOT EXISTS "taxes" (
	"id" text PRIMARY KEY NOT NULL,
	"name" text NOT NULL,
	"rate" integer NOT NULL,
	"description" text,
	"created_at" timestamp DEFAULT now(),
	"updated_at" timestamp DEFAULT now()
);
