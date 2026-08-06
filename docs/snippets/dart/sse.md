```dart
import 'package:spikard/spikard.dart';
import 'dart:convert';

void main() async {
  final app = App();

  // The Dart binding exposes `SseEvent` as a data type (see `SseEvent` /
  // `createSseEventFromJson` in `package:spikard/spikard.dart`), but route
  // registration on `App` is a single JSON-in/JSON-out handler — there is no
  // `app.sse(...)` streaming endpoint yet. Build event payloads with
  // `SseEvent` and return them as regular JSON.
  app.get_(
    "/events",
    (requestJson) async {
      final events = List.generate(
        3,
        (i) => SseEvent(eventType: "tick", data: jsonEncode({"tick": i})),
      );

      return jsonEncode(
        events
            .map((event) => {
                  "event": event.eventType,
                  "data": event.data,
                })
            .toList(),
      );
    },
  );

  await app.run();
}
```
