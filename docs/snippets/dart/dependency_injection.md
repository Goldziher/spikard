```dart
import 'package:spikard/spikard.dart';
import 'dart:convert';

class Database {
  Future<String> query(String sql) async {
    return "result";
  }
}

class UserService {
  final Database db;
  UserService(this.db);

  Future<Map<String, dynamic>> getUser(int id) async {
    await db.query("SELECT * FROM users WHERE id = $id");
    return {"id": id, "name": "Alice"};
  }
}

void main() async {
  final app = App();
  final db = Database();
  final userService = UserService(db);

  app.get_(
    "/users/:id",
    (requestJson) async {
      final request = jsonDecode(requestJson) as Map<String, dynamic>;
      final id = int.parse(request["path_params"]?["id"] as String? ?? "0");
      final user = await userService.getUser(id);
      return jsonEncode(user);
    },
  );

  await app.run();
}
```
