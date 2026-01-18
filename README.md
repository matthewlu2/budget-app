# Budget App

A full-stack budgeting web application with a Rust backend and a React frontend. This app allows users to track expenses and manage personal budgets through a simple web interface.

## Overview

This repository contains both the backend API and the frontend web application.

Backend:
- Rust
- Axum
- SeaORM
- SQLite

Frontend:
- React
- Vite
- JavaScript

## Prerequisites

- Rust & Cargo
- Node.js & npm
- Make
- sea-orm-cli

Install SeaORM CLI if needed:

    cargo install sea-orm-cli

## Setup and Running

All commands should be run from the root of the repository.

### Backend

Build the backend:

    make setup_backend

Initialize the database and run migrations:

    make initdb

Run the backend server:

    make run_backend

### Frontend

Install frontend dependencies:

    make setup_frontend

Run the frontend development server:

    make run_frontend
