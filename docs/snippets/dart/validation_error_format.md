```dart
import 'package:spikard/spikard.dart';
import 'dart:convert';

void main() async {
  final app = App();

  app.post(
    "/validate",
    (requestJson) async {
      final body = jsonDecode(requestJson) as Map<String, dynamic>;
      final errors = <Map<String, dynamic>>[];

      if (body["age"] is! int || (body["age"] as int) < 18) {
        errors.add({
          "field": "age",
          "message": "Age must be at least 18",
          "type": "validation_error",
        });
      }

      if (body["email"] is! String || !(body["email"] as String).contains("@")) {
        errors.add({
          "field": "email",
          "message": "Email format is invalid",
          "type": "validation_error",
        });
      }

      if (errors.isNotEmpty) {
        return jsonEncode({
          "detail": "${errors.length} validation errors",
          "errors": errors,
        });
      }

      return jsonEncode({"status": "valid"});
    },
  );

  await app.run();
}
```
