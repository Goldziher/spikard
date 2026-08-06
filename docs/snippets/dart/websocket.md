```dart
import 'package:spikard/spikard.dart';
import 'dart:convert';

void main() async {
  final app = App();

  // The Dart binding models WebSocket frames as `WebSocketMessage` (see
  // `package:spikard/spikard.dart`), but `App` does not yet expose a
  // dedicated `app.websocket(...)` route registration — handlers are the
  // same single JSON-in/JSON-out contract used by `app.post`. This shows
  // how an incoming frame's payload can be decoded and echoed back.
  app.post(
    "/ws/echo",
    (requestJson) async {
      final body = jsonDecode(requestJson) as Map<String, dynamic>;
      final incoming = WebSocketMessage.text(field0: body["message"] as String);

      final WebSocketMessage outgoing;
      if (incoming is WebSocketMessage_Text) {
        outgoing = WebSocketMessage.text(field0: incoming.field0);
      } else {
        outgoing = const WebSocketMessage.close(code: 1000, reason: "done");
      }

      if (outgoing is WebSocketMessage_Text) {
        return jsonEncode({"echo": outgoing.field0});
      }
      return jsonEncode({"closed": true});
    },
  );

  await app.run();
}
```
