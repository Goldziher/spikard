```dart
import 'package:spikard/spikard.dart';
import 'dart:convert';

void main() async {
  final app = App();

  app.get_(
    "/protected",
    (requestJson) async {
      final request = jsonDecode(requestJson) as Map<String, dynamic>;
      final headers = request["headers"] as Map<String, dynamic>? ?? {};
      final authHeader = headers["authorization"] as String? ?? "";

      if (!authHeader.startsWith("Bearer ")) {
        return jsonEncode({"error": "Unauthorized"});
      }

      final token = authHeader.substring(7);

      if (token != "dev-token") {
        return jsonEncode({"error": "Invalid token"});
      }

      return jsonEncode({"data": "protected resource"});
    },
  );

  await app.run();
}
```
