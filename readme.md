# XCHAT-API

## Installation
```
curl https://sh.rustup.rs -sSf | sh
rustup component add clippy # cargo clippy

apt install pkg-config libssl-dev
apt install default-libmysql-client-dev # apt search libmysql
cargo install diesel_cli --no-default-features --features "mysql"
```

## Development
```
cargo install catflap # https://github.com/passcod/catflap
cargo install cargo-watch # https://github.com/passcod/cargo-watch
catflap -- cargo watch -x run
```

## Database
```
# diesel is used for migration only
diesel migration run
diesel migration redo
```

## Testing
DATABASE_URL=mysql://root:root@127.0.0.1:3306/auto-chat-test diesel migration run
DATABASE_URL=mysql://root:root@127.0.0.1:3306/auto-chat-test cargo test

## Ideas: 
- Use SQS for resilience?
- database to run every X hours to delete password update token?
- analyze .unwrap() calls, make sure that it is handled properly
- use futures for sending emails
