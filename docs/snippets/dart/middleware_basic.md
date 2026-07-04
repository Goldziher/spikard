```dart
import 'package:spikard/spikard.dart';
import 'dart:convert';

void main() async {
  final app = App();

  app.get_(
    "/data",
    (requestJson) async {
      final request = jsonDecode(requestJson) as Map<String, dynamic>;
      final method = request["method"] ?? "GET";
      final path = request["path"] ?? "/";

      print("$method $path");

      return jsonEncode({"method": method, "path": path});
    },
  );

  await app.run();
}
```
