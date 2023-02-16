# Rust Pushcut

This project is supposed to become a Rust library around the API of the iOS App [Pushcut](https://www.pushcut.io/).

Reference for this Library is the [official API specification](https://www.pushcut.io/webapi).

As I am not a Pushcut Extended subscriber, I can't currently test any functionality related to Extended.

Currently it is not yet in a library form, as I am early in my Rust journey.

I am trying to make use of Rusts rich typing system and constrain generic strings as much as makes sense.

## Endpoint Implementation Status

Notifications:

- [x] GET /devices
- [ ] GET /notifications
- [x] POST /notifications/{notificationName}
- [ ] DELETE /submittedNotifications/{notificationId}

Automation Server:

- [ ] POST /execute
- [ ] POST /cancelExecution

Subscriptions:

- [ ] GET /subscriptions
- [ ] POST /subscriptions
- [ ] DELETE /subscriptions/{subscriptionId}

Images:

- [ ] PUT /images/{imageName}
- [ ] POST /images/{imageName}/move
