to run locally:
cargo run
127.0.0.1:8080

docker local:
docker build -t actix-web-app .
docker run -p 8080:8080 actix-web-app

docker for nas:
docker buildx build --platform linux/amd64 -t actix-web-app --output type=docker .
docker save -o actix-web-app.tar actix-web-app
