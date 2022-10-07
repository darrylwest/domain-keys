# domain-keys

## To Do

* badges for code coverage, build status, docs
* example of how implement routing logic for various destinations
* create min and max routes consts for Keys
* doc and integration tests
* create a key generation service - UDP request; do the same for base62
* create trait EncodeBase62 and DecodeBase62 to restrict generics to encode(any number) decode(vec<u8>, string, str, array[u8])
* code coverage - linux only
* add criterion, quickcheck for bench tests; [see this](https://github.com/fbernier/base62/blob/master/Cargo.toml)
* fuzzing (cargo-fuzz)
* ~~rethink Keys api; better names and replace routing_key with new; consider renaming Keys to RouteKey~~
* ~~finish keys implementation, unit, doc and integration tests, get_timestamp(), get_route()~~
* ~~add error enums for base62~~
* ~~extend the random number MIN to generate 5 chars then pad to 7 with zeros~~
* ~~refactor base62 encode / decode to domain_keys::base62::Base62 or to a separate module~~
* ~~decode base 62~~
* ~~replace chrono with std::time::{SystemTime, Duration, UNIX_EPOCH}; return timestamp in microseconds from keys~~
* ~~const fn to generate the base62 chars~~
* ~~embed a sequence into keys? similar to RFC4122 (see uuid timestamp impl)~~
* ~~seed the time stamp sequence number (2 bytes u16) with a random number for RFC4122~~
* ~~fill bytes to replace current random range~~
* ~~is SmallRng the best choice?  it's the fastest, but not-portable (don't know what that means)~~

###### darryl.west | 2022.10.06
