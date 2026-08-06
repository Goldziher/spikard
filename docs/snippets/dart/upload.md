```dart
import 'package:spikard/spikard.dart';
import 'dart:convert';

void main() async {
  final app = App();

  app.post(
    "/upload",
    (requestJson) async {
      final body = jsonDecode(requestJson) as Map<String, dynamic>;
      final fileJson = jsonEncode(body["file"]);

      final file = await createUploadFileFromJson(json: fileJson);

      return jsonEncode({
        "filename": file.filename,
        "size": file.content.length,
        "content_type": file.contentType,
      });
    },
  );

  await app.run();
}
```
