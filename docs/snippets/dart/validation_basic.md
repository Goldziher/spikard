```dart
import 'package:spikard/spikard.dart';
import 'dart:convert';

void main() async {
  final app = App();

  app.post(
    "/payments",
    (requestJson) async {
      final payment = jsonDecode(requestJson) as Map<String, dynamic>;

      if (payment is! Map) {
        return jsonEncode({"error": "Invalid payment object"});
      }

      if (payment["id"] is! String || (payment["id"] as String).isEmpty) {
        return jsonEncode({"error": "Payment id is required"});
      }

      if (payment["amount"] is! num || (payment["amount"] as num) <= 0) {
        return jsonEncode({"error": "Payment amount must be positive"});
      }

      return jsonEncode(payment);
    },
  );

  await app.run();
}
```
