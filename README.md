# corrodo

...

## Kubernetes Cloudflare Sync

```
docker build -t gcr.io/corrodo/kubernetes-cloudflare-sync:latest .
```

```
docker push gcr.io/corrodo/kubernetes-cloudflare-sync:latest
```

```
kubectl create secret generic cloudflare --from-literal=email=<email> --from-literal=api-key=<cloudflare global api key>
```

```
kubectl create clusterrolebinding cluster-admin-binding --clusterrole cluster-admin --user <email>
```

```
kubectl apply -f .
```
