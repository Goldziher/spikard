```dart
import 'package:spikard/spikard.dart';
import 'dart:convert';

void main() async {
  final app = App();

  app.get_(
    "/data",
    (requestJson) async {
      final response = {
        "id": 123,
        "value": "test",
        "timestamp": DateTime.now().toIso8601String(),
      };

      if (response["id"] is! int || (response["id"] as int) < 0) {
        throw Exception("Response validation: id must be non-negative integer");
      }

      if (response["value"] is! String || (response["value"] as String).isEmpty) {
        throw Exception("Response validation: value must be non-empty string");
      }

      return jsonEncode(response);
    },
  );

  await app.run();
}
```
