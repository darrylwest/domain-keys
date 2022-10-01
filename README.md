# domain-keys

```bash
 _____                        __          __  __                    
|     \.-----.--------.---.-.|__|.-----. |  |/  |.-----.--.--.-----.
|  --  |  _  |        |  _  ||  ||     | |     < |  -__|  |  |__ --|
|_____/|_____|__|__|__|___._||__||__|__| |__|\__||_____|___  |_____|
                                                       |_____|      
```

_A rust library & cli for key generation for domain entity identifiers e.g., user, provider, inventory item, etc._

### Overview

Features...

* fast random number generation based on the range of 2 terra to 13 peta for > 2^53 combinations and uniformly distributed.
* time based to the microsecond
* base62 encoded for size reduction: `[0-9][A-Z][a-z]`
* size is always 16 characters
* similar to UUID V7 where a timestamp is mixed with random, specifically random + timestamp(micros) + random
* route-able, not sortable (_although sort_by could be implemented for the timestamp portion of the key_)

The goal of the random number generation is speed and uniformity--not security.  Domain keys are suitable for identifying elements in a specific domain.  Uniformaty is important for routing to insure equally.

### When to use

When you...

* need to create unique identifiers for specified domains e.g. users with the minimum key size that will support billions of entities without collision. You also may want to extract the UTC datetime from the key.
* need to decode a portion of the key to implement data routing to dozens of destinations or database shards.
* generate your keys on the rust (or other) application's server side.

### When not to use

If you need to generate a key that is truely globally unique, then use UUID, probably v4 or v1.  You also are not concerned with key size.


### Installation

`cargo install domain_keys`

or, if you have rust installed and are at or above version 1.63, do this...

```bash
git clone https://github.com/darrylwest/domain-keys.git
cd domain-keys
cargo install --path ../domain-keys --bins
```

This installs all the binary clis.

### Route Key: rtkey


### Timestamp Key: txkey

_TBD_

### Library Use Examples


### CLI

`rtkey` : crates a single routing key
`txkey` : creates a single txid key
`show-route --routes n key`: shows the route number (0..n) for the given key

Flags
* --count n: generates n keys
* --size n: for tx key, adjust the size of the key
* --decode: decodes either the rtkey or txkey showing bytes, timestamp, etc
* route key: define the number of routes; the output 

### References

* [Rust Rand Book](https://rust-random.github.io/book/intro.html)
* [UUID RFC4122](https://datatracker.ietf.org/doc/html/rfc4122.html)
* [PCG Fast Algos for Random Number Generation](https://www.pcg-random.org/pdf/hmc-cs-2014-0905.pdf)

### To Do

* replace chrono with std::time::{SystemTime, Duration, UNIX_EPOCH}; return timestamp in microseconds from keys
* example of how implement routing logic for various destinations
* doc tests
* code coverage - linux only
* fuzzing (cargo-fuzz)
* ~~const fn to generate the base62 chars~~
* ~~embed a sequence into keys? similar to RFC4122 (see uuid timestamp impl)~~
* ~~seed the time stamp sequence number (2 bytes u16) with a random number for RFC4122~~
* ~~fill bytes to replace current random range~~
* ~~is SmallRng the best choice?  it's the fastest, but not-portable (don't know what that means)~~

###### darryl.west | 2022.09.30
