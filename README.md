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

#### Key Size Reduction

Our primary objective is to minimize key sizes when used with key/value stores like Redis or even SQL databases.  Domain keys are gauranteed to be unique for a given domain.  For example, your
set of users may grow into the 10s or 100s of millions.  You need a unique identifier for each user.  But, you don't need a globally unique identifier because these keys are restricted to just 
your domain of users.  Or a domain of customers, orders, etc.

Rather than use a V4 UUID with 36 characters, it's much better to use a shorter key based on the same technology--a combination of date-time and random numbers.  If you restrict a key
to a specific domain, e.g., users, customers, businesses, etc, the key can be reduced to a combination of a millisecond time stamp and either a random number, or a counter--especially
if the counter range is in the 200K.  Add a bit of random number generation and a two character domain prefix, like `US`, `CU`, `BU`.

So, let say you do reach that 10M users goal?  If you use a UUID, the keys will consume 360,000,000+ bytes.  
Compare that to 160,000,000 and you have cut your memory requirements by more than one half.

#### Routing for Sharding

The composition of the DomainKey includes two character domain followed by a two character random number to provide automatic shard-routing for 256 shards, or any divisable of 256.  
Lets say your user base starts to approach 1M.  If you use domain keys with random rounting, it's easy to split this into two, four, or eight shards.  With four shards, each database instance
now has 250K users--a much more manageable number.

So the idea is to start with a single instance but include the uniformally random routing as the key.  When it's time to shard, you simply implement shard-routing logic and you are good to go.
This is a life saver for startups that should think about sharding, but don't really need to shard until the time comes.

#### Domain Routing Key Features...

* fast, uniformly distributed random number generation based on large range (10^40?) of values
* time based to the microsecond
* base62 encoded for size reduction: `[0-9][A-Z][a-z]`
* routing key is always 16 characters, 9 date and 7 random including routing key (first two chars)
* similar to UUID V7 where a timestamp is mixed with random, specifically random + timestamp(micros) + random
* route-able, not sortable (_although sort_by could be implemented for the timestamp portion of the key_)
* short, time based keys from _txkey_ generate 12 character keys.

The goal of the random number generation is speed and uniformity--not security.  Domain keys are suitable for identifying elements in a specific domain.  Uniformaty is important for routing to insure equally.

#### Routing Key Features...

* fast, uniformly distributed random number generation based on large range (10^40?) of values
* time based to the microsecond
* base62 encoded for size reduction: `[0-9][A-Z][a-z]`
* routing key is always 16 characters, 9 date and 7 random including routing key (first two chars)
* similar to UUID V7 where a timestamp is mixed with random, specifically random + timestamp(micros) + random
* route-able, not sortable (_although sort_by could be implemented for the timestamp portion of the key_)

The goal of the random number generation is speed and uniformity--not security.  Domain keys are suitable for identifying elements in a specific domain.  Uniformaty is important for routing to insure equally.

### When to use

When you...

* need to create unique identifiers for specified domains e.g. users with the minimum key size that will support billions of entities without collision. You also may want to extract the UTC datetime from the key.
* need to decode a portion of the key to implement data routing to dozens of destinations or database shards.
* generate your keys on the rust (or other) application's server side.

### When not to use

If you need to generate a key that is truely globally unique, then use UUID, probably v4 or v1.  You also are not concerned with key size or being compatible with RFC4122 (UUID standard).

### Installation

`cargo install domain_keys`

or, if you have rust installed and are at or above version 1.63, do this...

```bash
git clone https://github.com/darrylwest/domain-keys.git
cd domain-keys
cargo add --path ../domain-keys --bins

# or

git submodule add https://github.com/darrylwest/domain-keys.git
cargo add --path domain-keys
```

### Library Use

```rust

use domain_keys::keys::RouteKey;

fn main() {
    let key = RouteKey::create();

    assert_eq!(key.len(), 16);

    println!("key: {}", &key);
}
```

### Binaries

* `rtkey` : crates a single routing key
* `txkey` : creates a single timestamp key
* `show-route --routes n key`: shows the route number (0..n) for the given key

### References

* [Base62 Defined](https://en.wikipedia.org/wiki/Base62)
* [UUID RFC4122](https://datatracker.ietf.org/doc/html/rfc4122.html)
* [PCG Fast Algos for Random Number Generation](https://www.pcg-random.org/pdf/hmc-cs-2014-0905.pdf)
* [Redis serialization protocol spec](https://redis.io/docs/reference/protocol-spec/)

## License

Except where noted (below and/or in individual files), all code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

###### darryl.west | 2023.09.05

