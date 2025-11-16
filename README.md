# Webhooked - webhook testing and forwarding service
This is a work in progress. Not meant to be used yet.
## Features
Currently can create a tunnel (which is just a record in a db table) and receive requests, extracting headers, body, http method and putting them in a structure. 
The plan is to display them as they come in the UI via server sent events. A CLI that can be configured to forward all requests to a locally running server (i.e. a clone of ngrok) is also planned.
Websockets connections are not supported yet but will be. Plain TCP and UDP sockets are also planned, but in a limited manner (because we can't identify the subdomain for such connections, so would have to allocate a port per user for those).
## Installation
Note that the project is only set up for dev usage. The backend service is currently started by going into the docker container terminal and manually doing `diesel migration run` (to prepare db) and `cargo run` to start the server. These steps are tested with Docker Desktop running on Windows 10 and the commands below running via WSL.
```
git clone ...
cd webhooked
mv docker-compose.example.yml docker-compose.yml\
mv .server.example.env .server.env
docker-compose up -d
docker-compose exec -it {backend_service_name} sh
diesel migration run
cargo run
```

A nginx service runs to forward all subdomains to the main Rust server. The backend paths will therefore have the format `http://subdomain.127.0.0.1.nip.io/whatever`.

The project is not ready to be used but if you're feeling adventurous make a POST request to `http://manage.127.0.0.1.nip.io/api/tunnels/'. You will receive json containing your brand spankin webhook url. Then you can make any request to `http://{tunnel_name}.127.0.0.1.nip.io/anything` with whatever body, headers, HTTP method you want and you'll see the server returning information about your request. This has been extracted from your request but currently is not saved to the DB (because there's a JSONB field which for reasons doesn't currently work).

You can also check out the somewhat stylish incomplete landing page at `http://localhost:5173/`.

# Stack
UI is done in Vue, though for now only a landing page exists. Backend is in Rust with Axum. Uses postgresql with diesel ORM. nginx is used to forward all subdomains to the server.

# Dev instructions
Install the pre-commit hook with:
```
chmod +x hooks/pre-commit
git config core.hooksPath hooks
```