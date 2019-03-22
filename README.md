# corrodo

...

## Rust greetings

Build and run locally 
```
cargo build
cargo run
```

Now go to [localhost:8080](http://localhost:8080) and it should show some greetings.

```
docker run -it --name rust-greetings-dev-env -p 8080:8080 -v <path/to/rust-greetings>:/usr/src/rust-greetings/ -w /usr/src/rust-greetings rust:latest bash
# or command below if docker container already exists
docker container start -i rust-greetings-dev-env

cargo build
cargo run

# go to http://localhost:8080 and it should work
```

Check app at [localhost:8080](http://localhost:8080).

Build image and upload to Google Contianer Registry
```
# go to 'rust-greetings' directory
docker build -t gcr.io/corrodo/rust-greetings:latest .

# verify image
docker run --name rust-greetings -p 8080:8080 gcr.io/corrodo/rust-greetings:latest
# check app locally
docker container rm --force rust-greetings

docker push gcr.io/corrodo/rust-greetings:latest
```

## Kubernetes Cloudflare Sync

```
docker build -t gcr.io/corrodo/kubernetes-cloudflare-sync:latest .
docker push gcr.io/corrodo/kubernetes-cloudflare-sync:latest

kubectl create secret generic cloudflare --from-literal=email=<email> --from-literal=api-key=<cloudflare global api key>
kubectl create clusterrolebinding cluster-admin-binding --clusterrole cluster-admin --user <email>
```

## Deployment on GKE

Just go to [k8s-config](/k8s-config) directory and run `kubectl apply -f .`. Check it on [corrodo.bizm.co.ua](http://corrodo.bizm.co.ua).
