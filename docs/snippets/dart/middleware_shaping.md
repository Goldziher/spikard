```dart
import 'package:spikard/spikard.dart';
import 'dart:convert';

void main() async {
  final app = App();

  app.get_(
    "/data",
    (requestJson) async {
      final request = jsonDecode(requestJson) as Map<String, dynamic>;
      final limit = (request["query_params"]?["limit"] as String? ?? "10");
      final offset = (request["query_params"]?["offset"] as String? ?? "0");

      return jsonEncode({
        "limit": int.tryParse(limit) ?? 10,
        "offset": int.tryParse(offset) ?? 0,
        "items": [],
      });
    },
  );

  await app.run();
}
```
