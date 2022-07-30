# TimezoneDB (WIP)

A rewrite for the backend of the BetterDiscord/Aliucord's Timezones plugins.

# Setup

Before you build, create a new Discord application in the [portal](https://discord.com/developers/applications).
You should make a redirect uri in the OAuth tab that is equal to `{HOST}/api/auth`, `{HOST}` being the environment
variable below. Copy the secret and id from the OAuth tab to set as environment variables, shown below.

### Environment variables

| PORT             | TYPE   | Default                                                | Description                                                  | 
|------------------|--------|--------------------------------------------------------|--------------------------------------------------------------|
| `PORT`           | u16    | 8000                                                   | The port to serve the app on                                 |
| `HOST`           | String | *
crash* in release, `http://localhost:{PORT}` in debug | The host string as the base section of the Discord redirect. |
| `DISCORD_ID`     | u64    | *
crash*                                                | The app client id from the OAuth section of the portal.      |
| `DISCORD_SECRET` | String | *crash*                                                | The app secret **from the OAuth
section of the portal**.     |
| `JWT_SECRET`     | any    | *
crash* in release, `timezone_db` in debug build       | Any value used for encrypting JWT tokens.                    | 

### Manual

```sh
# Manually:
$ pnpm install
$ pnpm build
$ cargo build --release
$ export PORT=<port>;DISCORD_ID=<id>;DISCORD_SECRET=<secret>;HOST=<host>;JWT_SECRET=<key>;
$ ./target/release/timezone_db
```

### Docker

TODO
