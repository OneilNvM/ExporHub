# Exporhub Website Frontend

This directory contains all of the source code for the website frontend for Exporhub, developed in NextJs

## Prerequisites

- Install latest Node.js version

First, make sure that you have run ```npm i``` in order to install the dependencies for this application.

Create a **certificates** folder and create a Certificate Authority called **rootCA.crt**, a TLS certificate called **exporhub.crt**, and an elliptic-curve private key called **private.key**.
Make sure to install the **rootCA** as a **Trusted Certificate Authority** and the server certificate afterwards.

This can be done through the use of **OpenSSL**, or any other cryptographic library which can make certificates and keys.

Next, create a **.env** file and add a ```SESSION_KEY``` variable, with a **base64 encoded string** value, this value can be generated online or with **OpenSSL.**

## IMPORTANT

Make sure to add **exporhub.com** and **api.exporhub.com** as **127.0.0.1** to your **hosts** file on Windows, the certificates will **not** work for **localhost**.

## Running the server

Then, run the development server:

```bash
npm run dev
# or
yarn dev
# or
pnpm dev
# or
bun dev
```

Open [https://exporhub.com:3000](https://exporhub.com:3000) with your browser to see the result.

## API Server

After starting up the frontend server, you will then need to either start up the **Rust API server**, or the **PHP API server** through the Dockerfile.
