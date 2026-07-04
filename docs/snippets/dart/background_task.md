```dart
import 'package:spikard/spikard.dart';
import 'dart:convert';
import 'dart:async';

void processEmailAsync(String email) {
  Future.delayed(Duration(seconds: 1), () {
    print("Email sent to $email");
  });
}

void main() async {
  final app = App();

  app.post(
    "/send-email",
    (requestJson) async {
      final body = jsonDecode(requestJson) as Map<String, dynamic>;
      final email = body["email"] as String? ?? "";

      processEmailAsync(email);

      return jsonEncode({
        "status": "queued",
        "email": email,
      });
    },
  );

  await app.run();
}
```
