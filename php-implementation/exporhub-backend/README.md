# Exporhub PHP REST API Server

This directory contains the source code for the PHP implementation of the REST API backend server for Exporhub

## Prerequisites

- Install Docker Desktop

Make sure to have a 'certificates' folder, only containing 'exporhub.crt' and 'private.key'

Configure the .env.example file by following the instructions on the [Laravel website](https://laravel.com/docs/12.x) and rename the file to .env

## Running the server

Make sure that Docker Desktop is running and use the command 'docker compose up -d' in order to build the docker container.

Access the API server at [https://api.exporhub.com](https://api.exporhub.com) or at [http://localhost](http://localhost)
