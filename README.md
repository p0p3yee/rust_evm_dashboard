# Rust EVM Dashboard

## In Development: Personal project for learning rust

---

## Frontend

1. Set backend url on .env

```
BACKEND_URL=127.0.0.1:8888
```

2. Install trunk
   
`cargo install trunk`

3. Start server
   
`trunk serve`

## Backend 

### Setup

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

5. Start
   
`cargo r`