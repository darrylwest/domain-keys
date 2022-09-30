# domain-keys

```
 _____                        __          __  __                    
|     \.-----.--------.---.-.|__|.-----. |  |/  |.-----.--.--.-----.
|  --  |  _  |        |  _  ||  ||     | |     < |  -__|  |  |__ --|
|_____/|_____|__|__|__|___._||__||__|__| |__|\__||_____|___  |_____|
                                                       |_____|      
```

_A rust library & cli for key generatoration for domain key identifiers e.g., user, provider, inventory item, etc._

### Overview

* random number generation based on the range of 2 terra to 13 peta for > 2^53 combinations.
* time based to the microsecond
* base62 encoded for size reduction: `[0-9][A-Z][a-z]`
* size is always 16 bytes

### When to use

* When you need to create unique identifiers for specified domains e.g. users with the minimum key size that will support billions of entities without collision. You also may want to extract the UTC datetime from the key.
* When you need to decode a portion of the key to implement data sharding to dozens of shards.

### When not to use

If you need to generate a key that is truely globally unique, then use UUID, probably v4 or v1.  You also are not concerned with key size.

### Sharded Key Generation


### TXID: Time Based Short Key Generation


### Library Use Examples


### CLI

`keys --shard`
`keys --txid`

`shard`
`txid`


###### darryl.west | 2022.09.30
