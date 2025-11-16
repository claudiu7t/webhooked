# Webhooked - webhook testing and forwarding service
This is a work in progress. Not meant to be used yet.
## Features
Currently can create a tunnel (which is just a record in a db table) and receive requests, extracting headers, body, http method and putting them in a structure. This struct is for now simply returned and not yet saved in the db due to issues with the JSONB field used to store the headers.

The plan is to display the requests as they come in the UI via server sent events. A CLI that can be configured to forward all requests to a locally running server (i.e. a clone of ngrok) is also planned.

Websockets connections are not supported yet but they will be. Plain TCP and UDP sockets are also planned, but in a limited manner (because we can't identify the subdomain for such connections, so would have to allocate a port per user for those).

## Installation
Note that the project is only set up for dev usage. The backend service is currently started by going into the docker container terminal and manually doing `diesel migration run` (only at the start to prepare db) and `cargo run` to start the server. Things are set up this way because I wanted to run everything in containers from the get-go and this allows me to quickly restart the server when changes are made. When the project is more stable a production version will be provided.

These steps are what I use when running the project using Docker Desktop on Windows 10. Docker is set up to use wsl for virtualization. The commands themselves are ran in WSL, though presumably git bash also works. Note that these are not really tested, and that the nginx configuration is quite shoddy on Windows 10:
```bash
git clone ...
cd webhooked
mv docker-compose.example.yml docker-compose.yml
mv .server.example.env .server.env
docker-compose up -d
docker-compose exec -it {backend_service_name} sh
# in the docker terminal
diesel migration run
cargo run
```

An nginx service runs to forward all subdomains to the main Rust server. The backend paths will therefore have the format `http://subdomain.127.0.0.1.nip.io/whatever`, because for some reason forwarding *.localhost doesn't work and nip.io does some magic to make it work. Fortunately when we'll deploy the service this shouldn't be an issue anymore.

The project is not ready to be used but if you're feeling adventurous make a POST request to `http://manage.127.0.0.1.nip.io/api/tunnels/`. You will receive json containing the key `tunnel_name`. Then you can make any request to `http://{tunnel_name}.127.0.0.1.nip.io/anything` with whatever body, headers, HTTP method you want and you'll see the server returning information about your request. This has been extracted from your request but currently is not saved to the DB (because there's a JSONB field which for some reason doesn't currently work).

You can use the test_requests.py python script to make some requests. You will have to modify the url at the top to use your tunnel_name. Note that the script sends multiple requests but for now there's an `exit()` after the first request: the rest of the requests are not tested, but they probably work.

You can also check out the somewhat stylish landing page at `http://localhost:5173/`.

# Stack
UI is done in Vue, though for now only a landing page exists. Backend is in Rust with Axum. Uses postgresql with diesel ORM. nginx is used to forward all subdomains to the server.

# Dev instructions
Install the pre-commit hook with:
```
chmod +x hooks/pre-commit
git config core.hooksPath hooks
```