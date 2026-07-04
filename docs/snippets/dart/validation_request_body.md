```dart
import 'package:spikard/spikard.dart';
import 'dart:convert';

void main() async {
  final app = App();

  app.post(
    "/users",
    (requestJson) async {
      final body = jsonDecode(requestJson) as Map<String, dynamic>;

      if (body["name"] is! String || (body["name"] as String).isEmpty) {
        return jsonEncode({
          "error": "Validation error",
          "field": "name",
          "message": "Name is required",
        });
      }

      if (body["email"] is! String || !(body["email"] as String).contains("@")) {
        return jsonEncode({
          "error": "Validation error",
          "field": "email",
          "message": "Email is invalid",
        });
      }

      return jsonEncode(body);
    },
  );

  await app.run();
}
```
