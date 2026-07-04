```dart
import 'package:spikard/spikard.dart';
import 'dart:convert';

void main() async {
  final app = App();

  app.get_(
    "/products",
    (requestJson) async {
      final request = jsonDecode(requestJson) as Map<String, dynamic>;
      final queryParams = request["query_params"] as Map<String, dynamic>? ?? {};

      final limitStr = queryParams["limit"] as String? ?? "10";
      final limit = int.tryParse(limitStr) ?? 10;

      if (limit < 1 || limit > 100) {
        return jsonEncode({
          "error": "Validation error",
          "field": "limit",
          "message": "Limit must be between 1 and 100",
        });
      }

      return jsonEncode({
        "limit": limit,
        "products": [],
      });
    },
  );

  await app.run();
}
```
