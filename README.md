# starcoder-server
Simple StarCoder server written in Rust

## Development
```
cd infra-dev
docker-compose up -d
```

## Release build
```
cd server
docker build -f ../infra-release/Dockerfile -t starcoder-server-release .
docker run --rm -d -p 8080:80 starcoder-server-release
```
