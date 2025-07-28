import java.math.BigInteger;

public class MersennePrime {
    public static boolean is_mersenne_prime(int number) {
        BigInteger sequence_value = new BigInteger("4");
        BigInteger mersenne_candidate = new BigInteger("1");
        mersenne_candidate = mersenne_candidate.multiply(BigInteger.TWO.pow(number)).subtract(BigInteger.ONE);

        int iterations = number - 2;

        for (int i = 0; i < iterations; i++) {
            sequence_value = sequence_value.multiply(sequence_value);
            sequence_value = sequence_value.subtract(BigInteger.TWO);
            sequence_value = sequence_value.mod(mersenne_candidate);
            System.out.printf("%d/%d%n", i + 1, iterations);
        }
        System.out.println("Lucas-Lehmer test complete");

        return sequence_value.equals(BigInteger.ZERO);
    }
}
