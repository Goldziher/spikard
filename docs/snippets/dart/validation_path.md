```dart
import 'package:spikard/spikard.dart';
import 'dart:convert';

void main() async {
  final app = App();

  app.get_(
    "/users/:id",
    (requestJson) async {
      final request = jsonDecode(requestJson) as Map<String, dynamic>;
      final idStr = request["path_params"]?["id"] as String? ?? "0";

      final id = int.tryParse(idStr);
      if (id == null || id < 1) {
        return jsonEncode({
          "error": "Validation error",
          "field": "id",
          "message": "ID must be a positive integer",
        });
      }

      return jsonEncode({"id": id, "name": "User $id"});
    },
  );

  await app.run();
}
```
