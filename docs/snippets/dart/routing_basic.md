```dart
import 'package:spikard/spikard.dart';
import 'dart:convert';

void main() async {
  final app = App();

  app.get_(
    "/health",
    (requestJson) async {
      return jsonEncode({"status": "ok"});
    },
  );

  app.post(
    "/users",
    (requestJson) async {
      final data = jsonDecode(requestJson) as Map<String, dynamic>;
      return jsonEncode({
        "id": data["id"],
        "name": data["name"],
      });
    },
  );

  await app.run();
}
```
