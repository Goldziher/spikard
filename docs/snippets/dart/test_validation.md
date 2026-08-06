```dart
import 'package:test/test.dart';
import 'dart:convert';

void main() {
  test('rejects a request body with an invalid field type', () async {
    final handler = (String requestJson) async {
      final body = jsonDecode(requestJson) as Map<String, dynamic>;

      if (body["age"] is! int) {
        return jsonEncode({
          "detail": "Validation failed",
          "errors": [
            {"field": "age", "message": "age must be an integer"},
          ],
        });
      }

      return jsonEncode({"name": body["name"], "age": body["age"]});
    };

    final invalidResponse = await handler(jsonEncode({
      "name": "Bob",
      "age": "not a number",
    }));
    final invalidData = jsonDecode(invalidResponse) as Map<String, dynamic>;
    expect(invalidData["detail"], equals("Validation failed"));
    expect((invalidData["errors"] as List).length, equals(1));

    final validResponse = await handler(jsonEncode({
      "name": "Bob",
      "age": 30,
    }));
    final validData = jsonDecode(validResponse) as Map<String, dynamic>;
    expect(validData["age"], equals(30));
  });
}
```
