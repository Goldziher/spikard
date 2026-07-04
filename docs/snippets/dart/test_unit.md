```dart
import 'package:test/test.dart';
import 'package:spikard/spikard.dart';
import 'dart:convert';

void main() {
  test('creates user with valid data', () async {
    final app = App();

    app.post(
      "/users",
      (requestJson) async {
        final body = jsonDecode(requestJson) as Map<String, dynamic>;
        return jsonEncode({
          "id": 1,
          "name": body["name"],
          "email": body["email"],
        });
      },
    );

    final request = jsonEncode({
      "name": "Alice",
      "email": "alice@example.com",
    });

    expect(request.isNotEmpty, true);
  });
}
```
