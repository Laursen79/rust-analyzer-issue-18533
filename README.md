# Minimal Example of rust-analyzer issue 18533
This example contains a server and a client.
If you run ``cargo build``, the build should pass.

In EITHER ``client/src/main.rs`` OR ``server/src/main.rs``, rust-analyzer will state 
that the database table that was queried in the SQL does not exist.

Let's say the error occurs in ``client``. If the error occurs in server, the 
problem is the same.
The query looks like this:

```rust 
sqlx::query!(
        r#"
        INSERT INTO client (thing)
        VALUES (31)
        "#
    )
    .execute(&pool)
    .await?;
```
You will get an error on the line:
```rust
    .execute(&pool)
```
saying that the table ``client`` does not exist. It is evaluating the query against the server DB??
If you change the query from:
```rust
r#"
INSERT INTO client (thing)
VALUES (31)
"#
```
to:
```rust
r#"
INSERT INTO server (other)
VALUES (31)
"#
```
you will instead get the error from the line:
```rust
sqlx::query!(
      r#"
      INSERT INTO server (other)
      VALUES (31)
      "#
    )
```
saying that the table server does not exist BECAUSE IT IS EVALUATING AGAINST THE CLIENT DB??
