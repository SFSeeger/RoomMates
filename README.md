# RoomMates

Making organizing easy

## Deployment

### Server

To deploy the server use the provided `Dockerfile` to build a docker image serving both frontend and api. This requires
docker on your machine and the server.

#### Server with Sqlite Database

> [!TIP]
> You can also use a `.env` file by using the `--env-file` flag with `docker run` or `docker compose` instead of passing
> environment variables directly via `-e`.

```bash
docker build -t roommates-server .
docker run -d -p 8080:8080 \
  -e DATABASE_URL="sqlite://db.sqlite?mode=rwc" \
  -v database:/app/db.sqlite \
  --name roommates-server roommates-server
```

#### Server with MySQL/MariaDB

Create a `docker-compose.yml` file with the following content:

```yaml
services:
  db:
    image: mariadb
    environment:
      MYSQL_ROOT_PASSWORD: password
      MYSQL_DATABASE: roommates
      MYSQL_USER: roommates
      MYSQL_PASSWORD: roommates_password
    volumes:
      - db_data:/var/lib/mysql

  roommates-server:
    build:
      context: .
      dockerfile: Dockerfile
    environment:
      DATABASE_URL: "mysql://roommates:roommates_password@db:3306/"
    ports:
      - "8080:8080"

volumes:
  db_data:
```

Then run:

````shell
docker compose up -d
````

### Clients

To bundle clients for production, install the [required tools](#tools-and-dependencies) or use the devcontainer.
Then choose the platform you want to bundle and optionally
the [package type](https://dioxuslabs.com/learn/0.7/tutorial/bundle#bundling-for-desktop-and-mobile)
Then run the following command in the root of the project[^1]:
> [!TIP]
> You can also bundle the server this way, if you dont want to use docker. In this case set `PLATFORM` to web. You can
> omit `SERVER_URL` as it is not needed for the web platform.

```bash
make bundle PLATFORM=<platform> SERVER_URL="<your-server-url>" [PACKAGES="<package1> [<package2> ...]"]
```

`SERVER_URL` should be the URL of your deployed server. Defaults to `http://localhost:8080`.

### Tools and Dependencies

For bundling, refer to
the [Dioxus documentation](https://dioxuslabs.com/learn/0.7/getting_started/#platform-specific-dependencies) for the
platform you want to bundle for.
Additionally you need:

* `The Rust toolchain`
* `Dioxus CLI`:
  [See installation instructions](https://dioxuslabs.com/learn/0.7/getting_started/#install-the-dioxus-cli)
* `Node` and `npm`

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
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── prelude.rs # Reexports of database entities
│       └── ...
├── form_hooks/ # Package providing utilities for handeling forms
│   ├── Cargo.toml
│   ├── form_hooks_derive/ # Package prividing derive macros for form traits
│   └── src/
└── frontend/
    ├── assets/ # Any assets that are used by the app should be placed here
    ├── Cargo.toml
    ├── src/
    │   ├── components/ # Collection of reusable components
    │   ├── main.rs # The entrypoint for the app. It also defines the routes for the app
    │   └── views/ # The views each route will render in the app
    └── tailwind.css
```

Modules in `routes` and `views` should be equal the URL structure. The API Route `/api/users/some-action` should be in a
module called `some_action` that is part of the `users` module. The full path of the route would be
`routes/users/some-action.rs`.

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
make dev-server
```

To run for a different platform, use the `PLATFORM` flag. E.g.

```bash
make dev-server PLATFORM=desktop
```

### Development Services

| Port | Service     | Description                                      |
|------|-------------|--------------------------------------------------|
| 8080 | Application | The Application served by the development server |
| 8000 | phpMyAdmin  | Database frontend for development                |
| 3306 | MariaDB     | Database Server                                  |

### Dev Container

1. Setup [Docker Desktop](https://www.docker.com/products/docker-desktop/)
1. Open this project in [Visual Studio Code](https://code.visualstudio.com/)
1. A popup should open prompting you to `Reopen in Container`. In case this does not happen use <kbd>Ctrl</kbd>+<kbd>
   Shift</kbd>+<kbd>P</kbd> (<kbd>⌘</kbd>+<kbd>⇧</kbd>+<kbd>P</kbd> on macOS) and type / select
   `> Dev Containers: Reopen in Container`

### Pre-Commit Hooks

We use pre-commit hooks to ensure consistent formatting and code quality in the project. `pre-commit` and the hooks
should already be installed in the devcontainer.

### Testing

To run the tests for the project, use the following command:

```bash
make tests
```

[^1]: Angle brackets (`<>`) indicate required arguments, square brackets (`[]`) indicate optional arguments.
