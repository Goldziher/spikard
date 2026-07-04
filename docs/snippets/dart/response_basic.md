```dart
import 'package:spikard/spikard.dart';
import 'dart:convert';

void main() async {
  final app = App();

  app.get_(
    "/users",
    (requestJson) async {
      return jsonEncode([
        {"id": 1, "name": "Alice"},
        {"id": 2, "name": "Bob"},
      ]);
    },
  );

  app.post(
    "/users",
    (requestJson) async {
      final body = jsonDecode(requestJson) as Map<String, dynamic>;
      return jsonEncode({
        "id": 1,
        "name": body["name"],
        "created": true,
      });
    },
  );

  await app.run();
}
```
