```dart
import 'package:test/test.dart';
import 'package:spikard/spikard.dart';
import 'dart:convert';

void main() {
  test('builds sse events with expected data', () async {
    final events = List.generate(
      3,
      (i) => SseEvent(eventType: "count", data: jsonEncode({"count": i})),
    );

    expect(events.length, equals(3));
    expect(events.first.eventType, equals("count"));

    final firstData = jsonDecode(events.first.data) as Map<String, dynamic>;
    final lastData = jsonDecode(events.last.data) as Map<String, dynamic>;
    expect(firstData["count"], equals(0));
    expect(lastData["count"], equals(2));
  });

  test('createSseEventFromJson round-trips an event payload', () async {
    final json = jsonEncode({
      "event_type": "notification",
      "data": jsonEncode({"message": "hello"}),
      "id": "evt-1",
      "retry": 3000,
    });

    final event = await createSseEventFromJson(json: json);

    expect(event.eventType, equals("notification"));
    expect(event.id, equals("evt-1"));
    expect(event.retry, equals(3000));
  });
}
```
