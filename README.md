# RoomMates

Making organizing easy

## Development

### Project Structure

```
packages/
├── api/ # Everything that the server needs to handle goes here
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── routes/ # Api routes
│       └── server/ # Server specific code not to be compiled into frontend
├── entity/ # Collection of database entities
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── prelude.rs # Reexports of database entities
│       └── ...
└── frontend/
    ├── assets/ # Any assets that are used by the app should be placed here
    ├── Cargo.toml
    ├── src/
    │   ├── components/ # Collection of reusable components
    │   ├── main.rs # The entrypoint for the app. It also defines the routes for the app
    │   └── views/ # The views each route will render in the app
    └── tailwind.css
```

Modules in `routes` and `views` should be equal the URL structure. The API Route `/api/users/some-action` should be in a module called `some_action` that is part of the `users` module. The full path of the route would be `routes/users/some-action.rs`.

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve --package frontend
```

To run for a different platform, use the `--platform platform` flag. E.g.

```bash
dx serve --platform desktop --package frontend
```

### Development Services

| Port | Service     | Description                                      |
| ---- | ----------- | ------------------------------------------------ |
| 8080 | Application | The Application served by the development server |
| 8000 | phpMyAdmin  | Database frontend for development                |
| 3306 | MariaDB     | Database Server                                  |

### Dev Container

1. Setup [Docker Desktop](https://www.docker.com/products/docker-desktop/)
1. Open this project in [Visual Studio Code](https://code.visualstudio.com/)
1. A popup should open prompting you to `Reopen in Container`. In case this does not happen use <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>P</kbd> (<kbd>⌘</kbd>+<kbd>⇧</kbd>+<kbd>P</kbd> on macOS) and type / select `> Dev Containers: Reopen in Container`

### Pre-Commit Hooks

We use pre-commit hooks to ensure consistent formatting and code quality in the project. `pre-commit` and the hooks should already be installed in the devcontainer.
