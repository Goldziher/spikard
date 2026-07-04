```dart
import 'package:spikard/spikard.dart';
import 'dart:convert';

void main() async {
  final app = App();

  app.post(
    "/users",
    (requestJson) async {
      final request = jsonDecode(requestJson) as Map<String, dynamic>;
      final headers = request["headers"] as Map<String, dynamic>? ?? {};
      final authorization = headers["authorization"] as String? ?? "";

      if (authorization != "Bearer dev-token") {
        return jsonEncode({"error": "Unauthorized"});
      }

      final body = request["body"] as Map<String, dynamic>? ?? {};
      return jsonEncode({"id": 1, "name": body["name"]});
    },
  );

  await app.run();
}
```
