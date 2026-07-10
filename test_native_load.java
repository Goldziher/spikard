public class test_native_load {
    public static void main(String[] args) {
        try {
            Class.forName("dev.spikard.NativeLib");
            System.out.println("SUCCESS: NativeLib loaded without UnsatisfiedLinkError");
        } catch (ClassNotFoundException e) {
            System.err.println("FAIL: Could not find NativeLib class: " + e);
        } catch (Exception e) {
            System.err.println("FAIL: " + e.getClass().getSimpleName() + ": " + e.getMessage());
            e.printStackTrace();
        }
    }
}
