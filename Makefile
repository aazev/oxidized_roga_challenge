start:
	mkdir -p .local/data
	docker compose -f .local/docker-compose.yml up -d

stop:
	docker compose -f .local/docker-compose.yml down

reset: stop start reset_migrations

reset_migrations:
	.local/scripts/reset_database.sh
