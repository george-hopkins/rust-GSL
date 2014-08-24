#
# A rust binding for the GSL library by Guillaume Gomez (guillaume1.gomez@gmail.com)
#

all: rgsl examples doc

rgsl:
	mkdir -p lib
	rustc --out-dir=lib src/rgsl.rs

examples: rgsl
	  mkdir -p bin
	  rustc -o bin/simple -L ./lib examples/simple/main.rs
	  rustc -o bin/rng -L ./lib examples/rng/main.rs
	  rustc -o bin/random_number_distribution -L ./lib examples/random_number_distribution/main.rs
	  rustc -o bin/permutation -L ./lib examples/permutation/main.rs
	  rustc -o bin/sorting -L ./lib examples/sorting/main.rs
	  rustc -o bin/chebyshev_approximation -L ./lib examples/chebyshev_approximation/main.rs
	  rustc -o bin/combination_example -L ./lib examples/combination_example/main.rs
	  rustc -o bin/roots_of_polynomial -L ./lib examples/roots_of_polynomial/main.rs
	  rustc -o bin/numerical_differentiation -L ./lib examples/numerical_differentiation/main.rs
	  rustc -o bin/radix -L ./lib examples/radix/main.rs

doc:
	rustdoc -o doc src/rgsl.rs

clean:
	rm -rf lib
	rm -rf bin/simple
	rm -rf bin/rng
	rm -rf bin/random_number_distribution
	rm -rf bin/permutation
	rm -rf bin/sorting
	rm -rf bin/chebyshev_approximation
	rm -rf bin/combination_example
	rm -rf bin/roots_of_polynomial
	rm -rf bin/numerical_differentiation
	rm -rf bin/radix

re: clean all