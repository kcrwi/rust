- Vi utvecklar en *"Binary Crate"*, dvs. en executable. De crates vi laddar ner är *"Library Crates"*.

- Vi använder oss av enum:en *"results"* vid felhantering.

- "*Ordering"* is a useful enum. It contains (atleast) three variants
    - Less
    - Greater
    - Equal
- I can access the variants (members) of that enum with the turbofish (::).

- If nothing else is said, an "int" is of type i32 in Rust.

- .trim() kan kallas på en string. Den tar bort alla white-spaces.