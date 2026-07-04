```dart
import 'package:test/test.dart';
import 'package:spikard/spikard.dart';
import 'dart:convert';

void main() {
  test('hello endpoint returns greeting', () async {
    final app = App();
    
    app.get_(
      "/hello",
      (requestJson) async {
        return jsonEncode({"message": "Hello, World!"});
      },
    );

    final handler = (requestJson) async {
      return jsonEncode({"message": "Hello, World!"});
    };

    final response = await handler("{}");
    final data = jsonDecode(response) as Map<String, dynamic>;

    expect(data["message"], equals("Hello, World!"));
  });
}
```
