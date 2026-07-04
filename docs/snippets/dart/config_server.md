```dart
import 'package:spikard/spikard.dart';
import 'dart:convert';

void main() async {
  final app = App();

  final config = ServerConfig();
  app.config(config: config);

  app.get_(
    "/health",
    (requestJson) async {
      return jsonEncode({"status": "ok"});
    },
  );

  await app.run();
}
```
