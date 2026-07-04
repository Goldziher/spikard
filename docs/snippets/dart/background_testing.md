```dart
import 'package:test/test.dart';
import 'package:spikard/spikard.dart';
import 'dart:convert';
import 'dart:io';

void main() {
  test('background task is queued', () async {
    final app = App();

    app.post(
      "/task",
      (requestJson) async {
        final body = jsonDecode(requestJson) as Map<String, dynamic>;
        return jsonEncode({
          "task_id": "task-001",
          "status": "queued",
        });
      },
    );

    final httpClient = HttpClient();
    final request = await httpClient.post(
      'localhost',
      8000,
      '/task',
    );
    request.write(jsonEncode({"name": "test"}));
    final response = await request.close();

    expect(response.statusCode, equals(200));
    httpClient.close();
  });
}
```
