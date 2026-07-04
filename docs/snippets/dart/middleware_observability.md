```dart
import 'package:spikard/spikard.dart';
import 'dart:convert';
import 'dart:async';
import 'package:uuid/uuid.dart';

const uuid = Uuid();

void main() async {
  final app = App();

  app.get_(
    "/data",
    (requestJson) async {
      final request = jsonDecode(requestJson) as Map<String, dynamic>;
      final requestId = uuid.v4();

      print("request_id=$requestId method=${request["method"]} path=${request["path"]}");

      return jsonEncode({
        "request_id": requestId,
        "data": "response",
      });
    },
  );

  await app.run();
}
```
