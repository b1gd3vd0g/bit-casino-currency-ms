# Bit Casino -- Currency Microservice

> [!NOTE]
> This service is currently **stable** but under development.

A **REST API** written in **Rust** handling **safe currency transactions** for **Bit Casino** - a virtual gambling simulator.

### Features

- Stores logs for all transactions, successful and unsuccessful.
- Prevents negative balances and double spending using row-locking and transactions.

## How to use this repository

This service is not very useful on its own. It relies upon the [**Player Microservice**](https://github.com/b1gd3vd0g/bit-casino-player-ms) and a **PostgreSQL** database

To test this API alongside the whole environment, you can follow the instructions in the [Infrastructure](https://github.com/b1gd3vd0g/bit-casino-infra) repository to test all services locally using **Docker Compose**.

You can then interact via the frontend at `localhost:60000` or call the integrated currency microservice directly at `localhost:60601`.

## Functionality

The currency microservice currently supports the following functions:

- Create a new bit wallet to track a player's currency.
- Find a player's current balance.
- Attempt and log a new transaction.

## Related Repositories

- [Player Microservice](https://github.com/b1gd3vd0g/bit-casino-player-ms) - Handles account creation and player authentication.
- [Reward Microservice](https://github.com/b1gd3vd0g/bit-casino-reward-ms) - Handles daily bonus claims and streaks.
- [Slots Microservice](https://github.com/b1gd3vd0g/bit-casino-slots-ms) - Handles the backend for the custom slot machine game **Byte Builder**.
- [Frontend](https://github.com/b1gd3vd0g/bit-casino-frontend) - A react app creating a user-friendly interface with which to interact with the backend.
- [Infrastructure](https://github.com/b1gd3vd0g/bit-casino-infra) - Allows for integration testing locally using **docker compose**.
