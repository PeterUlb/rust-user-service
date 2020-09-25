# Welcome

This is a **development API prototype**.

# Getting started with development

Following steps must be performed to run the project in an editor or IDE of your choice:

- [Install rust](https://www.rust-lang.org/tools/install)
- [Install postgres database](https://www.postgresql.org/download/)
- Clone this repository
- Navigate to the project's folder
- Use **config/local.yml** to adjust config. You can also use environment variables (e.g. APP_DATABASE.URL)
- Open a terminal in the project's folder and perform commands
  - "cargo install"
  - "cargo install diesel_cli --no-default-features --features postgres" (once to install cli)
  - "diesel setup" (once)
  - "diesel migration run" (only upon changes)
  - "cargo run" (every time to run the project, optionally use --release)

# Create new migrations

- Check the [diesel page](http://diesel.rs/guides/getting-started/)
- e.g. "diesel migration generate create_posts"

# Project Structure

WIP. Currently 3 layered approach.
