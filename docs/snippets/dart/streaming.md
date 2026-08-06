```dart
import 'package:spikard/spikard.dart';
import 'dart:convert';

void main() async {
  final app = App();

  // `App` route handlers return a single `Future<String>` — there is no
  // chunked/streaming response API exposed to Dart yet. Build the full
  // payload up front (here as newline-delimited JSON) and return it as one
  // JSON-encoded body.
  app.get_(
    "/events",
    (requestJson) async {
      final lines = List.generate(
        3,
        (i) => jsonEncode({"tick": i}),
      );

      return jsonEncode({"lines": lines});
    },
  );

  await app.run();
}
```
