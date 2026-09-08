import 'package:test/test.dart';
import 'package:spikard/spikard.dart' as spikard;

void main() {
  test('FieldErrorSpec equality holds for identical field values', () {
    // Literal-constructs the generated `FieldErrorSpec` DTO twice with identical field
    // values and compares them for equality, so a constructor that drops/renames a
    // field, or generated equality that stops being field-based, fails `dart test`
    // immediately instead of shipping green with a suite that asserts nothing about
    // the generated API. Create-only scaffold seed. ~keep
    final a = spikard.FieldErrorSpec(path: 'alef-scaffold', message: 'alef-scaffold');
    final b = spikard.FieldErrorSpec(path: 'alef-scaffold', message: 'alef-scaffold');
    expect(a, equals(b));
  });
}
