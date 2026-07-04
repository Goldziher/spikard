```dart
import 'package:spikard/spikard.dart';
import 'dart:convert';

int maxRetries = 3;
Map<String, int> retryCount = {};

Future<String?> callExternalApi() async {
  return null;
}

void main() async {
  final app = App();

  app.post(
    "/process",
    (requestJson) async {
      final body = jsonDecode(requestJson) as Map<String, dynamic>;
      final taskId = body["task_id"] as String? ?? "";

      int retries = retryCount[taskId] ?? 0;

      final result = await callExternalApi();
      if (result == null && retries < maxRetries) {
        retries++;
        retryCount[taskId] = retries;
        return jsonEncode({
          "status": "retry",
          "attempt": retries,
          "max_attempts": maxRetries,
        });
      }

      return jsonEncode({
        "status": "completed",
        "result": result,
      });
    },
  );

  await app.run();
}
```
