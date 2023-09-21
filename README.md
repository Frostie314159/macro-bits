# macro-bits
A collection of declarative macros, which makes working with bitfields and similar things trivial.
## philosophy
Every macro which makes your live simpler should have zero performance penalty in a sense, that even though they might look complex and intimidating.
They actually just evaluate, to exactly the same code you'd write by hand and LLVM can optimize them to dust.
## motivation
Frame formats, like the one used by IEEE802.11, use a metric crap ton(definetly an SI unit) of bitfields and, since I appear to be addicted to writing binary parsers, have developed it to takeover some repetitive tasks.
