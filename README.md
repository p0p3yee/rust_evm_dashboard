# rust_evm_dashboard

1. Install sqlite3
   
```
mac: brew install sqlite3
```

2. Install diesel cli:

`cargo install diesel_cli --no-default-features --features sqlite`

3. Create .env:

```
DATABASE_URL: xxx.sqlite
```

4. Start migration

`diesel migration run`

5. Run

`cargo r`