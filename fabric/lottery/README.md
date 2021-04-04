## noble-lottery

A lottery noble that uses participation in the network to purchase tickets.

With this noble, you can configure a lottery, which is a pot of money that
users contribute to, and that is reallocated to a single user at the end of
the lottery period. Just like a normal lottery system, to participate, you
need to "buy a ticket", which is used to fund the pot.

The unique feature of this lottery system is that tickets can only be
purchased by making a "valid call" dispatched through this noble.
By configuring certain calls to be valid for the lottery, you can encourage
users to make those calls on your network. An example of how this could be
used is to set validator nominations as a valid lottery call. If the lottery
is set to repeat every month, then users would be encouraged to re-nominate
validators every month. A user can ony purchase one ticket per valid call
per lottery.

This noble can be configured to use dynamically set calls or statically set
calls. Call validation happens through the `ValidateCall` implementation.
This noble provides one implementation of this using the `CallIndices`
storage item. You can also make your own implementation at the runtime level
which can contain much more complex logic, such as validation of the
parameters, which this noble alone cannot do.

This noble uses the modulus operator to pick a random winner. It is known
that this might introduce a bias if the random number chosen in a range that
is not perfectly divisible by the total number of participants. The
`MaxGenerateRandom` configuration can help mitigate this by generating new
numbers until we hit the limit or we find a "fair" number. This is best
effort only.
