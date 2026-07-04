```dart
import 'package:test/test.dart';
import 'package:spikard/spikard.dart';
import 'dart:convert';

void main() {
  test('validates request body', () async {
    final handler = (requestJson) async {
      final body = jsonDecode(requestJson) as Map<String, dynamic>;

      if (body["email"] is! String || !(body["email"] as String).contains("@")) {
        return jsonEncode({"error": "Invalid email"});
      }

      return jsonEncode({"status": "valid"});
    };

    final validRequest = await handler(jsonEncode({
      "email": "test@example.com",
    }));
    final validData = jsonDecode(validRequest) as Map<String, dynamic>;
    expect(validData["status"], equals("valid"));

    final invalidRequest = await handler(jsonEncode({
      "email": "invalid-email",
    }));
    final invalidData = jsonDecode(invalidRequest) as Map<String, dynamic>;
    expect(invalidData["error"], isNotNull);
  });
}
```
