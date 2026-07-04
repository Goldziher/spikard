```dart
import 'package:spikard/spikard.dart';
import 'dart:convert';
import 'dart:io';

void main() async {
  final app = App();

  final host = Platform.environment["SPIKARD_HOST"] ?? "127.0.0.1";
  final portStr = Platform.environment["SPIKARD_PORT"] ?? "8000";
  final port = int.tryParse(portStr) ?? 8000;

  final config = ServerConfig();
  app.config(config: config);

  app.get_(
    "/config",
    (requestJson) async {
      return jsonEncode({
        "host": host,
        "port": port,
      });
    },
  );

  await app.run();
}
```
