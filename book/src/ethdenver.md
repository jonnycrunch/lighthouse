# ETHDenver 2020

<img style="height:300px" src="https://www.ethdenver.com/wp-content/themes/understrap/img/hero-logo-2020.png"/>

**Hello ETHDenver hackers!**

As you know, [Eth2 is
coming](https://img.buzzfeed.com/buzzfeed-static/static/2019-01/4/12/asset/buzzfeed-prod-web-01/sub-buzz-26378-1546623248-1.jpg)
and Danny Ryan (Ethereum Foundation) has posted some
[bounties](https://notes.ethereum.org/@djrtwo/ethdenver-bounties)! We're well
on track to producing one of the first production Eth2 clients and we'd love
you to hack on our client!

To make things easier for you we've prepared:

- This guide, to help get you started.
- A public testnet, especially for you.
- Created the [`ethdenver`](https://github.com/sigp/lighthouse/pull/854) fork on [sigp/lighthouse](https://github.com/sigp/lighthouse) so we can move quickly with
	your development needs.
- Updated our API docs to include _all_ endpoints, even our development ones.
- Flown one of developers, Michael, over from [Australia](https://sexysciencefacts.files.wordpress.com/2015/07/kangaroo.jpg)! (He's _actually_ there to see talks and enjoy himself, but he's keen to help).
- Dedicated other remote team members to help you ideate and develop, you can
	reach them on our
	[Discord](https://sexysciencefacts.files.wordpress.com/2015/07/kangaroo.jpg).

## Lighthouse Overview

There are two main components to Lighthouse:

- Beacon Node (shorthand: BN): this is like Geth/Parity-Ethereum, it connects to the P2P
	network, verifies blocks and keeps them it's database.
- Validator Client (shorthand: VC): this is something that doesn't exist in Eth1, it's a
	**small** component designed to handle signing and slashing protection.

You can learn how to use these two components and connect to our testnet in
[Become a Validator](./become-a-validator.md).

## Ideas

### Evil Validators (from the EF bounties doc)

This is a fun one, you can modify our validator client and/or beacon node to
nefarious things. If you do this, it would be great if you can make it a
feature we can turn on/off with a CLI flag, instead of just hard-coded in a
branch somewhere. This way we can run it on our testnets to test against
malicious actors. BN CLI is [here](https://github.com/sigp/lighthouse/blob/master/beacon_node/src/cli.rs) and VC CLI is [here](https://github.com/sigp/lighthouse/blob/master/validator_client/src/cli.rs).

#### Produce double blocks

In [this
code](https://github.com/sigp/lighthouse/blob/371e5adcf89d99a5958b802cf9925a990bd66ba6/validator_client/src/block_service.rs#L196-L233)
we can see the validator takes the following steps:

- Locally produces a randao reveal (`randao_reveal`)
- Requests a block from the BN (`produce_block`).
- Locally signs the block (`sign_block`).
- Sends the block back to BN to be published (`publish_block`).


All you need to do is add some code that copies the block you've received from
the BN, modify one copy so it's different (but still valid), sign them both
and send them both back.

#### Slashable attestations (a.k.a. double attestations)

This one is slightly more complex than the previous because we do some fancy
pre-aggregation of attestations (we'll have to stop this since it became
illegal in a recent spec update).

- First we iterate through all validators and find the ones that are in the
	same committee for the given slot (see [here](https://github.com/sigp/lighthouse/blob/371e5adcf89d99a5958b802cf9925a990bd66ba6/validator_client/src/attestation_service.rs#L191-L217)).
- Then, we go to the BN and get an attestation for each committee,
	have it signed by each validator in that committee, then send it back to
	the BN (see [here](https://github.com/sigp/lighthouse/blob/371e5adcf89d99a5958b802cf9925a990bd66ba6/validator_client/src/attestation_service.rs#L232-L279)).

Just like "Produce double blocks" (above), what we need to do is copy the
attestation, modify it (so it's still valid), sign both and send them both back
to the beacon node.

#### Invalid Eth1 votes

This one is really easy (so you probably wont win with it alone). Eth1 votes
are injected into blocks
[here](https://github.com/sigp/lighthouse/blob/371e5adcf89d99a5958b802cf9925a990bd66ba6/beacon_node/beacon_chain/src/beacon_chain.rs#L1396).
Modify `eth1_data` and you're done.

### Creating local testnets and co-ordinating attacks

We use local testnets a lot during development so we've made it quick and easy
to start them. Check out the (Simple Local Testnet)[./simple-testnet.md] page
to start a local testnet with two nodes in 3 commands.

You may want to include some tactics from the previous "Evil Validators"
section to create co-ordinated attacks.

## Partitioned Networks

As we mentioned in the previous section, it's easy to create testnets with
Lighthouse.

When you've partitioned the network you're going to need to observe the
outcome. You can use the [HTTP API](./http.md) to compare which head block the
node is on or you can view metrics on nodes using
[lighthouse-metrics](https://github.com/sigp/lighthouse-metrics). (Hint: monitor more than
one node at once by adding it to
[`scrape-targets.json`](https://github.com/sigp/lighthouse-metrics/blob/master/scrape-targets/scrape-targets.json)).

## Noisy Network

We already have Noise implemented and ready to switch on with a few lines of
code, so this ones probably not a good fit for Lighthouse. Sorry!

## Optimizations

Optimizing Lighthouse is really rewarding, especially since you can really easy
see how long operations are taking whilst the node is running with taking using
[lighthouse-metrics](https://github.com/sigp/lighthouse-metrics).

Issue [#847](https://github.com/sigp/lighthouse/issues/847) is tracking the
most important optimizations required for Lighthouse. We've started a few of
them, but these ones might be interesting:

- Introducing a `BeaconState` cache that means we don't have to do copy the
	`BeaconState` or load it from the database before we start processing it.
	[This](https://github.com/sigp/lighthouse/blob/371e5adcf89d99a5958b802cf9925a990bd66ba6/beacon_node/beacon_chain/src/beacon_chain.rs#L1198-L1202) is where we obtain the state for processing. Be careful to keep the cache small though (1-5 states) because they're very big structs!
- Analyzing memory usage in Lighthouse. Presently we're seeing lots more
	physical RAM usage in `htop` than compared to what shows in `valgrind` or
	`heaptrack`. We suspect this is fragmentation, but that's quite hard to
	prove. If you can cut our RAM usage in half without comprimising
	block/attestation processing times then we'll be _very, very_ grateful!
	We have some time put away for this next week, if you can do it at the
	hackathon we can focus on other things!