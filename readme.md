the pseudo language


#### usage
First write some pseudcode, preferably in English.

```
$ cat primes.pseudo

for numbers less than 100
  if the number is prime, print it
  and if the number is 2, print "woo!" because that's my lucky number
```

Then run the compiler, specifying an output language.
Warning: compilation requires an internet connection and can be quite slow.

```shell
$ pseudoc primes.pseudo --lang cpp --output primes.cpp
$ cat primes.cpp

#include <stdio.h>
#include <cmath>

int main(void) {
  printf("2 ");

  for (int i=2; i<100; i++) {
    for (int j=2; j<=i; j++) {
      if (i % j == 0) {
        break;
      } else if (j+1 > sqrt(i)) {
        printf("%d ", i);
        break;
      }
    }

    if (i == 2) {
      printf("woo! ");
    }
  }
}
```

It should compile and run:

```shell
$ g++ -o primes primes.cpp
$ ./primes
2 woo! 3 5 7 11 13 17 19 23 29 31 37 41 43 47 53 59 61 67 71 73 79 83 89 97
```

If it fails to compile or run, please file an issue!


#### gameplan
* pseudoc is a rust binary that sends the contents of the input file
and the file's md5 to some endpoint,
and then it periodically polls that endpoint for a compiled result
* the server running at the endpoint stores the contents in postgres, keyed by the md5
* someone logs in to a webapp to view the pseudocode and compile it
