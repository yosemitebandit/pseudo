the pseudo language

[![Travis build status](https://travis-ci.org/yosemitebandit/pseudo.svg?branch=master)](https://travis-ci.org/yosemitebandit/pseudo)
[![Appveyor build status](https://ci.appveyor.com/api/projects/status/mmky10ccdp303y7h?svg=true)](https://ci.appveyor.com/project/yosemitebandit/pseudo)



#### usage
Write some pseudcode, preferably in English:

```
$ cat primes.pseudo

for numbers less than 100
  if the number is prime, print it
  and if the number is 2, print "woo!" because that's my lucky number
```

Then run the compiler, specifying an output language.
Warning: compilation requires an internet connection,
are not guaranteed to be deterministic and can be quite slow.

```shell
$ pseudoc primes.pseudo --language cpp --output primes.cpp
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


#### building the server and compiler locally
* pin rust to a specific nightly, then build and setup the database:

```
$ multirust override nightly-2016-04-09`
$ cargo build
$ diesel database setup
```

* to try this out locally you need a `.env` file -- see `.example.env`


#### todo
* maybe split up this repo so we can build `pseudoc` on appveyor?


#### ops
* server: adjust via `sudo service pseudo-lang-<PORT> restart`
  * there's an nginx config in `sites-available` ..check that out for the `<PORT>`
  * logs are in `/var/log`
* your `.env` file is important..check it out before you compile
