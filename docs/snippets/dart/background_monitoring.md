```dart
import 'package:spikard/spikard.dart';
import 'dart:convert';

int activeJobs = 0;
int completedJobs = 0;

void main() async {
  final app = App();

  app.get_(
    "/health/jobs",
    (requestJson) async {
      return jsonEncode({
        "active_jobs": activeJobs,
        "completed_jobs": completedJobs,
      });
    },
  );

  app.post(
    "/task",
    (requestJson) async {
      activeJobs++;
      try {
        await Future.delayed(Duration(seconds: 1));
        completedJobs++;
        activeJobs--;
        return jsonEncode({"status": "completed"});
      } catch (e) {
        activeJobs--;
        return jsonEncode({"status": "failed", "error": e.toString()});
      }
    },
  );

  await app.run();
}
```
