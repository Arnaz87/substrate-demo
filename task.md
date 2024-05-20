Clone the substrate [node template](https://github.com/substrate-developer-hub/substrate-node-template) and add the following features. All extrinsics should be benchmarked and unit tested, and should return proper errors on failure.

### Basic Features

Create a new pallet named pallet-tags with the following features

- create a tag
    - a tag contains a bounded vector of bytes provided by the caller
    - a deposit is reserved from the caller when the tag is created. The amount of the deposit should be set in the Config.
    - an event should be emitted with the tag id and its data
    - each tag has an id as a key, and this id increments with every tag created
- destroy a tag
    - only the account that created the tag can destroy it
    - the deposit should be unreserved when the tag is destroyed
    - an event should be emitted with the id of the destroyed tag
    - a tag cannot be destroyed if any nfts are tagged with it (if extended features are added)

### Extended features

Add the substrate [nfts pallet](https://github.com/paritytech/polkadot-sdk/tree/master/substrate/frame/nfts) to the runtime. Add the following features to the tags pallet.

- tag an nft
    - An NFT can only be tagged if it exists in the nfts pallet
    - an event should be emitted with the tag id and the nft collection and item id
    - fail properly if the tag doesn’t exist
- untag an nft
    - Remove the tag from the NFT
    - an event should be emitted with the tag id and the nft collection and item id
    - fail properly if the tag doesn’t exist or the nft is not tagged
    - the tag can still be removed if the nft doesn’t exist

### Resources

- https://docs.substrate.io/tutorials/build-a-blockchain/
- https://github.com/paritytech/polkadot-sdk/tree/master/substrate/frame/examples/basic