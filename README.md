<p align="center">
  <img src="https://raw.githubusercontent.com/SFSeeger/RoomMates/4ab1125851ddc569f2063b7d383ecc33173fa996/packages/frontend/assets/icon.svg" alt="Logo" width="150" height="150" />
</p>
<h1 align="center">@SFSeeger/RoomMates</h1>
<p align="center">
  Making organizing easy
</p>

<details open>
<summary><h2>Table of Contents</h2></summary>

- [Features](#features)
- [Deployment](#deployment)
  - [Server](#server)
    - [Server with Sqlite Database](#server-with-sqlite-database)
    - [Server with MySQL/MariaDB](#server-with-mysqlmariadb)
  - [Clients](#clients)
    - [Android](#android)
  - [Tools and Dependencies](#tools-and-dependencies)
- [Development](#development)
  - [Project Structure](#project-structure)
  - [Serving Your App](#serving-your-app)
  - [Development Services](#development-services)
  - [Dev Container](#dev-container)
  - [Pre-Commit Hooks](#pre-commit-hooks)
  - [Testing](#testing)
- [Disclosure of AI Usage](#disclosure-of-ai-usage)

</details>

## Features



## Deployment

### Server

To deploy the server use the provided `Dockerfile` to build a docker image serving both frontend and api. This requires
docker on your machine and the server.

#### Server with Sqlite Database

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
      MYSQL_ROOT_PASSWORD: ${MYSQL_ROOT_PASSWORD}
      MYSQL_DATABASE: roommates
      MYSQL_USER: roommates
      MYSQL_PASSWORD: ${MYSQL_PASSWORD}
    volumes:
      - db_data:/var/lib/mysql

  roommates-server:
    build:
      context: https://github.com/SFSeeger/RoomMates.git#main
    restart: unless-stopped
    ports:
      - "8080:8080"
    environment:
      DATABASE_URL: "mysql://roommates:${MYSQL_PASSWORD}@db:3306/roommates"
      ACCESS_LOG: true # Optional shows a access log in stdout
    depends_on:
      - db

volumes:
  db_data:
```

And a `.env` file like this in the same directory:
```shell
MYSQL_PASSWORD = <super secret password>
MYSQL_ROOT_PASSWORD = <super secret password 2>
```
Then run:

````shell
docker compose --env-file .env up -d
````

### Clients

Bundling the following targets have been tested. While bundling untested targets may work, there is a chance they require additional configuration.
- [X] Web
- [X] Linux
- [ ] Windows
- [ ] macOS
- [X] Android
- [ ] iOS


To bundle clients for production, install the [required tools](#tools-and-dependencies) or use the devcontainer.
Then choose the platform you want to bundle and optionally
the [package type](https://dioxuslabs.com/learn/0.7/tutorial/bundle#bundling-for-desktop-and-mobile).
Run the following command in the root of the project[^1]:
> [!IMPORTANT]
> `SERVER_URL` should be the URL of your deployed server. Defaults to `http://localhost:8080`.

> [!TIP]
> You can also bundle the server this way, if you don't want to use docker. In this case set `PLATFORM` to web. You can omit `SERVER_URL` as it is not needed for the web platform.

```shell
make bundle PLATFORM=<platform> SERVER_URL="<your-server-url>" [PACKAGES="<package1> [<package2> ...]"]
```
#### Android
> [!IMPORTANT]
> This bundeling config assumes you have a valid keystore in `~/.android/keystore.jks`. You can override the keystore location by using the `KEYSTORE_PATH` argument when bundling.
> Refer to [the android docs](https://developer.android.com/studio/publish/app-signing) on how to create one

> [!NOTE]
> This creates a `.apk` file for sideloading. The `.aab` bundle created by dioxus does not include the app icon


```shell
make bundle PLATFORM=android SERVER_URL="<your-server-url>" KEYSTORE_PASSWORD="<your-keystore-password>"
```


### Tools and Dependencies

For bundling, refer to
the [Dioxus documentation](https://dioxuslabs.com/learn/0.7/getting_started/#platform-specific-dependencies) for the
platform you want to bundle for.
Additionally you need:

* `The Rust toolchain`
* `Dioxus CLI`:
  [See installation instructions](https://dioxuslabs.com/learn/0.7/getting_started/#install-the-dioxus-cli)
* `Node` and `npm`
* `make`

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
├── form_hooks/ # Package providing utilities for handling forms
│   ├── Cargo.toml
│   ├── form_hooks_derive/ # Package providing derive macros for form traits
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

## Disclosure of AI Usage
AI was used for Tab-Completing and Debugging, never for generating whole sections of code without a human creating derivatives of said generated code.
Model used were:
- ChatGPT 4o, 4.1 and 5
- Github Copilot
- Google Gemini


[^1]: Angle brackets (`<>`) indicate required arguments, square brackets (`[]`) indicate optional arguments.
