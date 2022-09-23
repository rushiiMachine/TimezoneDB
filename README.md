# TimezoneDB ![Hits](https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2FDiamondMiner88%2FTimezoneDB&count_bg=%2379C83D&title_bg=%23555555&icon=github.svg&icon_color=%23E7E7E7&title=views&edge_flat=true)

A rewrite for the backend of the BetterDiscord/Aliucord's Timezones plugins.

# Setup

Before you build, create a new Discord application in the [portal](https://discord.com/developers/applications).
You should make a redirect uri in the OAuth tab that is equal to `{HOST}/api/auth`, `{HOST}` being the environment
variable below. Copy the secret and id from the OAuth tab to set as environment variables, shown below.

### Environment variables

Can be modified in the [.env](./.env) file.

| PORT             | TYPE   | Default                                              | Description                                                                  | 
|------------------|--------|------------------------------------------------------|------------------------------------------------------------------------------|
| `PORT`           | u16    | 8000 (unmodifiable in debug)                         | The port to serve the app on                                                 |
| `HOST`           | String | crash in release, `http://localhost:{PORT}` in debug | The host string as the base section of the Discord redirect.                 |
| `DISCORD_ID`     | u64    | crash                                                | The app client id from the OAuth section of the portal.                      |
| `DISCORD_SECRET` | String | crash                                                | The app secret from the **OAuth section** of the portal.                     |
| `JWT_SECRET`     | any    | crash in release, `timezone_db` in debug build       | Any value used for encrypting JWT tokens.                                    |
| `POSTGRES_URL`   | String | crash                                                | The full [connection string](https://stackoverflow.com/a/20722229/13964629). |

### Manual

```sh
# Manually:
$ pnpm install
$ pnpm build
$ cargo build --release
# Set environment variables manually
$ export PORT=<port>;DISCORD_ID=<id>;DISCORD_SECRET=<secret>;HOST=<host>;JWT_SECRET=<key>POSTGRES_URL=<connection_string>;
# or using .env
$ mv .env.local .env
$ ./target/release/timezone_db
```

## Development

Due to rocket not supporting proxying requests, I've had to do it the other way around in development; interface through
CRA and proxy unknown requests to the backend server. This leads to a few weird issues like any 404s from either end
will end up serving index.html from CRA. However, the rust portion is built in release mode, the React app will be
bundled with the executable and served using rocket.

1. Clone repo
2. Install pnpm & rust toolchain & perl (for openssl-sys)
3. Pull dependencies: `pnpm install`
4. Launch postgres: `docker run --name test-postgres -e POSTGRES_PASSWORD=password -d -p 5432:5432 postgres`
6. Run backend: `DISCORD_ID=<id>;DISCORD_SECRET=<secret>;POSTGRES_URL="postgres://postgres:password@localhost:5432/postgres" cargo run`
5. Run frontend: `pnpm start` 
7. App: `http://localhost:3000`

## API
Authentication is done through a JWT token in the `loginInfo` cookie that is sent with every request.

### GET `/api` (auth optional)
Response: `{ loggedIn: bool }`

### GET `/api/auth`
Redirects to the currently configured Discord OAuth url.

### GET `/api/auth?error=access_denied`
Redirects to `/`

### GET `/api/auth?code=<code>`
Authenticates the user with JWT, sets the cookie and redirects to `/`

### GET `/api/auth/logout`
Removes the auth cookie and redirects to `/`

### GET `/api/user` (auth required)
Redirects to `/api/user/<current_id>` based on the JWT otherwise throws 401 if no auth.

### GET `/api/user/<id>`
Gets the data from the DB and returns json. Returns 404 if no user with that id is found.

| Field      | Type   | Description                                                       |
|------------|--------|-------------------------------------------------------------------|
| userId     | string | Discord user snowflake id.                                        |
| timezoneId | string | The timezone name.                                                |
| timezone   | string | The calculated UTC offset of the timezone. Ex. `+5`, `-5` `+5:30` |

### GET `/api/user/<id>/exists`
Checks whether a user is stored in the DB. Returns 200/404 status code.

### DELETE `/api/user` (auth required)
Deletes the current user from the db completely.

### PUT `/api/user` (auth required)
Updates the current user's data.

Body:

| Field    | Type    | Description                |
|----------|---------|----------------------------|
| timezone | string? | A new valid timezone name. |
