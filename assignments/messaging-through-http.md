# An HTTP messaging broker

The purpose of this exercise is to implement a small messaging server on top of the standard hyper HTTP library.

The server should (in the end) be asychronous, tested and handle a number of clients.

A client should also be created.

The purpose of this exercise is to get used to shared ownership techniques, futures and async programming in Rust.

The example consciously doesn't implement a real-world protocol for simplicities sake, but can be expanded to one.

## Specification

The server implements a simple messaging broker on top of the hyper framework. For that, it should implement three endpoints:

## Preface

The API is a JSON API and should use the appropriate mime-types.

### Heartbeat

```
GET /
HEAD /
```

Requests on "/" should respond with `200 OK` and a short message:

```
{ "ok": true }
```

### Subscribe

```
GET /channels/<name>[,<name>,...]
```

Subscribes the client to one or more channels. As long as the client keeps the connection open, the server will send any event happening on that channel to the client. The format is:

```
{ "channel": "<name>", "payload": <structured json|text>}\n
{ "channel": "<name>", "payload": <structured json|text>}\n
```

Negotiation of the payload structure is out of scope for the server system.

A disconnected client does not receive events.

### Publish

```
POST /channel/<name>
```

Sends an arbitrary event to a channel. All currently connected subscribers to the channel will receive the event as _payload_.

The event can be JSON or arbitary text. Use the appropriate MIME types to indicate either.

## Extensions

### 1. Persistence

Channels are persisted: On subscription, clients can pass the GET parameter `last_message`, and will receive all messages from that point in the channel. 

### 2. Persistence on Disk

Like extension 1., but actually saving to disk.