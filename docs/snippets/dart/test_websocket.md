```dart
import 'package:test/test.dart';
import 'package:spikard/spikard.dart';
import 'dart:convert';

void main() {
  test('echoes a text websocket message', () async {
    String echo(WebSocketMessage message) {
      if (message is WebSocketMessage_Text) {
        return message.field0;
      }
      throw StateError('expected a text message');
    }

    final incoming = const WebSocketMessage.text(field0: "Hello");
    expect(echo(incoming), equals("Hello"));

    final jsonIncoming = const WebSocketMessage.text(
      field0: '{"type":"ping"}',
    );
    final decoded = jsonDecode(echo(jsonIncoming)) as Map<String, dynamic>;
    expect(decoded["type"], equals("ping"));
  });

  test('close message carries a code and reason', () async {
    const closeMessage = WebSocketMessage.close(
      code: 1000,
      reason: "normal closure",
    );

    expect(closeMessage, isA<WebSocketMessage_Close>());
    final close = closeMessage as WebSocketMessage_Close;
    expect(close.code, equals(1000));
    expect(close.reason, equals("normal closure"));
  });
}
```
