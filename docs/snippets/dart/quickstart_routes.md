```dart
import 'package:spikard/spikard.dart';
import 'dart:convert';

void main() async {
  final app = App();

  app.get_(
    "/users/:id",
    (requestJson) async {
      final request = jsonDecode(requestJson) as Map<String, dynamic>;
      final id = request["path_params"]?["id"] ?? "0";
      return jsonEncode({
        "id": int.parse(id.toString()),
        "name": "Alice",
      });
    },
  );

  app.post(
    "/users",
    (requestJson) async {
      final data = jsonDecode(requestJson) as Map<String, dynamic>;
      return jsonEncode(data);
    },
  );

  final config = ServerConfig();
  app.config(config: config);
  await app.run();
}
```
