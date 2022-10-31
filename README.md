# Fuel Order Book powered by predicates

Demo of using predicates to orchestrate a swap without any smart contracts.
The counter-party discovery / order matching is out of scope.

## TODO

- figure out how to dynamically construct a predicate,
- there is this concept of predicate inputs in the SDK

----

Note that predicates are not deployed to the chain. Rather, they are just a hash behind which some coins are locked.
if you can provide the bytecode that generates that hash AND have it evaluate to true, then ta-da you can spend it. This means that when creating an order, you just have to send coins to the code hash.
When executing the order, you just need the tx to make the predicate return true.

_Truly exciting stuff_
