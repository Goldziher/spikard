```dart
import 'package:test/test.dart';
import 'package:spikard/spikard.dart';
import 'dart:convert';

void main() {
  test('middleware processes request', () async {
    final logs = <String>[];

    final handler = (requestJson) async {
      final request = jsonDecode(requestJson) as Map<String, dynamic>;
      logs.add("${request["method"]} ${request["path"]}");
      return jsonEncode({"status": "ok"});
    };

    await handler(jsonEncode({
      "method": "POST",
      "path": "/api/data",
    }));

    expect(logs, contains("POST /api/data"));
  });
}
```
