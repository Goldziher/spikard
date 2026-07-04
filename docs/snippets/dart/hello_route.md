```dart
import 'package:spikard/spikard.dart';
import 'dart:convert';

void main() async {
  final app = App();
  
  app.get_(
    "/hello",
    (requestJson) async {
      return jsonEncode({"message": "Hello, World!"});
    },
  );

  await app.run();
}
```
