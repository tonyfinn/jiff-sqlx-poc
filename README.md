# Instructions

1. Set up a running postgres server.
2. Set the PGHOST, PGUSER, PGPASS and DATABASE_URL variables.
   - If you use direnv, then these will be set in `.envrc`, apart from PGPASS
   - Otherwise run

```
export PGUSER=postgres
export PGHOST=127.0.0.1
export PGDATABASE=jiff_sqlx_poc
export PGPASS=password
export DATABASE_URL=postgres://$PGUSER@$PGHOST/$PGDATABASE
```

Or the equivalent for your shell. Make sure the PG environment variables are valid for your local database setup.

3. Install sqlx-cli (`cargo install sqlx-cli` or use the Nix file)
4. Run `sqlx database create`
4. Run `sqlx migrate run`
5. Run `cargo build`
