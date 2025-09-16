# Sekrol

* setup
```bash
cd client
trunk build --release
```

* run
```bash
cd server
cargo run
```

* url
```bash
http://localhost:2000/

#video
POST | GET http://localhost:2000/api/videos
```

* env
```bash
SERVER_HOST = "127.0.0.1"
SERVER_PORT = 2000
SERVER_DATABASE_URL = "./your/path/"
SERVER_DATABASE_NAME = "your.db.name.sqlite"
SERVER_DATABASE_PASSWORD = "your_database_password"
SERVER_PRIVATE_JWT_KEY = "private_jwt_key"
SERVER_UPLOADS_URL = "./your/upload/path/"
```