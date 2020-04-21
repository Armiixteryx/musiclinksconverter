# MusicLinksConverter

## Description
This is a Telegram bot to convert links between Deezer and Spotify.

This bot relies on [tbot][] crate to establish the connection to Telegram API.
tbot relies on [tokio][] which is a runtime to async Rust.

I consume the Deezer and Spotify Web API manually.

_Q: But why you consume the Spotify API manually if [there are so many Spotify
crates that do it?][spotify-crates]_

_R: For learning purposes._

_Q: And for Deezer API?_

_R: The same reason :)_

## What does it convert?
This bot currently converts:

- [x] Songs.

- [ ] Albums.

- [ ] Artists.

## Running
* Clone this repository.

### Tbot configuration
* Register a new [Telegram bot][telegram-bot]. Then export a variable named
`BOT_TOKEN` with the Telegram token:

In bash:

`export BOT_TOKEN="TOKEN"`

### Bot configuration

#### Spotify
Spotify requires a registered app to make calls to its API.

* [Register a new Spotify app][spotify-register]

* Get the `CLIENT_ID` and the `CLIENT_SECRET` of your registered app in the
[Spotify developers dashboard][spotify-dashboard]. Then, export those
variables. In bash:

`export BOT_SPOTIFY_CLIENT_ID="CLIENT_ID"`

`export BOT_SPOTIFY_CLIENT_SECRET="CLIENT_SECRET"`

#### Deezer
Deezer API allows to use its public data __without regist an app__.

Deezer is good.

Be like Deezer :)

### Compilation

* If you just want to use this bot, you should compile with the release option:

`cargo build --release`

It will last so much time because of Rust and LLVM optimizations.

* If you want to hack, compile without optimization:

`cargo build`

The compilation in this case will be faster.

### Running

`cargo run`

## Actual state
This bot is in __ALPHA__ state.

Currently, if a link is malformated the bot will panic without sending a error
response to the user. However, the bot will still working because tokio (the
async machine used by tbot) will remain active.

I am refactoring the code in _error_handling_ branch to handle errors properly.

[tbot]: https://tbot.rs/
[tokio]: https://tokio.rs/
[spotify-crates]: https://lib.rs/search?q=spotify
[telegram-bot]: https://core.telegram.org/bots#3-how-do-i-create-a-bot
[spotify-register]: https://developer.spotify.com/documentation/general/guides/app-settings/#register-your-app
[spotify-dashboard]: https://developer.spotify.com/dashboard/
