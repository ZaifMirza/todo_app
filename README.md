# To-Do List App Backend (DFX + Rust)

## Project Description
This project is a backend for a simple To-Do List application built using Rust and DFX (Dfinity's development framework). It supports user registration, authentication, and task management features, including task creation, updating, deletion, and retrieval.

## Features
- **User Management:**
  - Register new users
  - User login and logout

- **Task Management:**
  - Create tasks with due dates and importance flags
  - Mark tasks as completed
  - Toggle task importance
  - Delete tasks by title
  - Retrieve lists of tasks, including completed tasks

## Requirements
- Rust (latest stable version recommended)
- DFX SDK
- Internet Computer setup
- Cargo package manager

## Installation and Setup
1. **Clone the Repository:**
   ```bash
   git clone <repository-url>
   cd todo_app
   ```

2. **Install DFX:**
   Follow the official DFINITY instructions to install DFX:
   [DFX Installation Guide](https://internetcomputer.org/docs/current/developer-docs/quickstart/dfx-quickstart)

3. **Start the DFX environment:**
   ```bash
   dfx start --background
   ```

4. **Deploy the Canister:**
   ```bash
   dfx deploy
   ```

## Project Structure
- `src/`
  - `todo_app_backend/`: Rust source code for backend logic
  - `lib.rs`: Core logic for user management and task handling

- `candid/`
  - `todo_app_backend.did`: Candid interface definition for interacting with the backend

```


