# Todo CLI

A simple command-line Todo application written in **Rust**.
This is a pet project built for learning Rust fundamentals, including:

* File I/O
* JSON serialization/deserialization
* Error handling
* CLI design
* Custom error types

---

## Features

*  List all todos
*  Add a new todo
*  Remove a todo by ID
*  Mark a todo as done
*  Custom error handling
*  Todo priarity
*  Todo tags and search

---

##  Commands

### `list`

Displays all existing todos.

```bash
todo list
```

---

### `add`

Adds a new todo item.

```bash
todo add "Buy groceries"
```

---

### `remove`

Removes a todo by its ID.

```bash
todo remove 3
```

---

### `done`

Marks a todo as completed.

```bash
todo done 2
```

---

##  Error Handling

The application uses custom error types to provide clear and structured error reporting.

### Custom Errors

| Error Type | Description                                           |
| ---------- | ----------------------------------------------------- |
| `IO`       | File read/write errors                                |
| `Json`     | JSON parsing/serialization errors                     |
| `NotFound` | Returned when a todo with the given ID does not exist |

Errors are handled gracefully and displayed in a user-friendly format.

---

## Project Structure (Example)

```
src/
 ├── main.rs
 ├── todo.rs
 ├── storage.rs
 ├── error.rs
```

* `main.rs` – CLI entry point
* `todo.rs` – Todo model
* `storage.rs` – File persistence logic
* `error.rs` – Custom error definitions

---

## Future Improvements

* [ ] Add due dates
* [ ] Add priority levels
* [ ] Improve terminal formatting

