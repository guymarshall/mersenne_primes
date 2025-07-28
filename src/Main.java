public class Main {
    public static void main(String[] args) {
        int number = 25873;
//        int number = 104729;
//        int number = 1299709;
//        int number = 15485863;
//        int number = 2147483647;

        if (!Prime.is_prime(number)) {
            System.out.printf("Exponent %d is not prime. Lucas-Lehmer test is only valid for prime exponents.%n", number);
            System.exit(0);
        }

        System.out.printf("Testing whether 2^%d - 1 is a Mersenne prime...%n", number);

        if (MersennePrime.is_mersenne_prime(number)) {
            System.out.printf("✅ 2^%d - 1 is a Mersenne prime!%n", number);
        } else {
            System.out.printf("❌ 2^%d - 1 is NOT a Mersenne prime.%n", number);
        }
    }
}
