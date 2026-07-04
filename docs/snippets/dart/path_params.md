```dart
import 'package:spikard/spikard.dart';
import 'dart:convert';

void main() async {
  final app = App();

  app.get_(
    "/orders/:order_id",
    (requestJson) async {
      final request = jsonDecode(requestJson) as Map<String, dynamic>;
      final orderId = request["path_params"]?["order_id"] ?? "0";
      final includeDetails = request["query_params"]?["include_details"] == "true";

      return jsonEncode({
        "id": int.parse(orderId.toString()),
        "details": includeDetails,
      });
    },
  );

  await app.run();
}
```
