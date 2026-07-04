```dart
import 'package:test/test.dart';
import 'package:spikard/spikard.dart';
import 'dart:convert';

void main() {
  test('middleware logs requests', () async {
    final requests = <String>[];

    final handler = (requestJson) async {
      final request = jsonDecode(requestJson) as Map<String, dynamic>;
      requests.add("${request["method"]} ${request["path"]}");
      return jsonEncode({"status": "ok"});
    };

    await handler(jsonEncode({
      "method": "GET",
      "path": "/test",
    }));

    expect(requests, contains("GET /test"));
  });
}
```
