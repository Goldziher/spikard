```dart
import 'package:test/test.dart';
import 'package:spikard/spikard.dart';
import 'dart:io';
import 'dart:convert';

void main() {
  test('app starts and serves requests', () async {
    final app = App();

    app.get_(
      "/health",
      (requestJson) async {
        return jsonEncode({"status": "ok"});
      },
    );

    final config = ServerConfig();
    app.config(config: config);

    expect(app, isNotNull);
  });
}
```
