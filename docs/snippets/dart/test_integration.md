```dart
import 'package:test/test.dart';
import 'package:spikard/spikard.dart';
import 'dart:io';
import 'dart:convert';

void main() {
  test('multiple routes work together', () async {
    final app = App();

    app.get_(
      "/users/:id",
      (requestJson) async {
        final request = jsonDecode(requestJson) as Map<String, dynamic>;
        final id = request["path_params"]?["id"] ?? "1";
        return jsonEncode({"id": id, "name": "Alice"});
      },
    );

    app.post(
      "/users",
      (requestJson) async {
        final body = jsonDecode(requestJson) as Map<String, dynamic>;
        return jsonEncode({"id": 2, "name": body["name"]});
      },
    );

    expect(app, isNotNull);
  });
}
```
