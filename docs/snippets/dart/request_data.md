```dart
import 'package:spikard/spikard.dart';
import 'dart:convert';

void main() async {
  final app = App();

  app.post(
    "/data",
    (requestJson) async {
      final request = jsonDecode(requestJson) as Map<String, dynamic>;
      final method = request["method"] ?? "GET";
      final path = request["path"] ?? "/";
      final headers = request["headers"] as Map<String, dynamic>? ?? {};

      return jsonEncode({
        "method": method,
        "path": path,
        "headers": headers,
      });
    },
  );

  await app.run();
}
```
