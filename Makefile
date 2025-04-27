.ONESHELL:

.PHONY: setup_backend
setup_backend:
	cd backend/budget-backend && cargo build

.PHONY: initdb
initdb:
	cd backend/budget-backend && touch budget.db
	cd backend/budget-backend && DATABASE_URL="sqlite://budget.db" sea-orm-cli migrate refresh
	
.PHONY: run_backend
run_backend:
	cd backend/budget-backend && RUST_LOG=debug cargo run

.PHONY: setup_frontend
setup_frontend:
	cd frontend/budget-app && npm install

.PHONY: run_frontend
run_frontend:
	cd frontend/budget-app && npm run dev
