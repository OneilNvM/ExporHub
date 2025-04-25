# Exporhub Rust REST API Server

This directory contains the source code for the Rust implementation of the REST API backend server for Exporhub

## Prerequisites

- Install the latest version of Rust
- Install a MySQL database to run locally or get access to a remote MySQL database
- Create a .env file containing a variable called 'DATABASE_URL' and set its value to your MySQL database url
- Install the diesel-cli binary via this command 'cargo install diesel_cli'

Ensure that you have completed the steps described in the Website Frontend README.md, as this implementation requires the 'certificates' folder in that directory.

## Running the server

Before running the API server, you will need to use this diesel command to setup the database on your MySQL server 'diesel setup'.

After this you can run 'diesel migration run' to run the migrations.

Then, you can run the server by either using 'cargo run' or 'cargo run -- db-seed' (the second command runs the database seeder to insert dummy data into the database)

You can then access the server at [https://api.exporhub.com:9000](https://api.exporhub.com:9000) with your browser to see the API server is running.
