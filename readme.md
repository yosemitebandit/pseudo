the pseudo language


#### usage
First write some pseudcode, preferably in English.

```
$ cat some-code.txt

show a list of primes
only if they are less than 100
and if the number is 2, print 'woo!' because that is my lucky number
```

Then run the compiler, specifying an output language.
Warning: compilation can be slow and requires an internet connection.

```shell
$ pseudoc some-code.txt --lang cpp --outfile some-code.cpp
$ cat some-code.cpp

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
$ g++ -o some-code some-code.cpp
$ ./some-code
2 woo! 3 5 7 11 13 17 19 23 29 31 37 41 43 47 53 59 61 67 71 73 79 83 89 97
```

If it fails to compile or run, please file an issue!
