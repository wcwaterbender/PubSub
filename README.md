# pubsub 

Novel implementation of the publisher/subscription design pattern, in RustLang

Two applications:

1. publisher : connects to a Redis instance and publishes a message to the `stats` channel then terminates

2. subscriptor : connects to a Redis instance and subscribes to the `stats` channel queue to listen and handle messages asynchronously. prints message payloads when receieved.

# Building / test
PreReqs: 
1. must have a running redis server on your host, or in a container
2. Podman must be installed


To build either app, subscriptor as example:

1. `cd subscriptor`
2. `podman build . -t redis-sub:latest`

To Run and connect to redis server on host network:

1. `podman run --net=host redis-sub:latest`

If done correctly you will get the following output:
```
service started
connected to queue
```

likewise after starting `redis-pub` you should see:

```
service started
published
```

and the subscriptor will handle the published message:
```
Message { id: "13a22505cf7f448b98a13fc4358c326e", channel: "stats", payload: "Ping" }
```

where id is a randomly generated uuid

