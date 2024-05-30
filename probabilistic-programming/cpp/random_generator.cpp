#include <cmath>
#include <iostream>
#include <random>
#include <vector>

std::vector<double> rnd_exp(double lambda, int n) {
  // Step 1: Create random number generator
  std::random_device rd;
  std::mt19937 gen(rd());
  std::uniform_real_distribution<> dis(0.0, 1.0);

  std::vector<double> exponential_random_numbers;
  exponential_random_numbers.reserve(n);

  // Step 2: Generate exponential random numbers using the inverse transform
  // method
  for (int i = 0; i < n; ++i) {
    double u = dis(gen);  // Generate a uniform random number in [0, 1)
    double exp_random =
        -std::log(1.0 - u) / lambda;  // Transform to exponential distribution
    exponential_random_numbers.push_back(exp_random);
  }

  return exponential_random_numbers;
}

int main() {
  double lambda = 2.0;  // Set the lambda value
  int n = 10;           // Number of random numbers to generate

  std::vector<double> random_numbers = rnd_exp(lambda, n);

  std::cout << "Generated exponential random numbers:" << std::endl;
  for (double num : random_numbers) {
    std::cout << num << " ";
  }
  std::cout << std::endl;

  return 0;
}