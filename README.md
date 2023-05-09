# BLS Signatures Rust

A BLS digital signature—also known as [Boneh–Lynn–Shacham (BLS)]—is a
cryptographic signature scheme which allows a user to verify that a signer is
authentic.

[Boneh–Lynn–Shacham (BLS)]: https://www.semanticscholar.org/paper/Short-Signatures-from-the-Weil-Pairing-Boneh-Lynn/3c0c82f42172bc1da4acc36b656d12351bf53dae

The scheme uses a bilinear pairing for verification, and signatures are elements
of an elliptic curve group. Working in an elliptic curve group provides some
defense against index calculus attacks (with the caveat that such attacks are
still possible in the target group $G_{T}$ of the pairing), allowing shorter
signatures than Full Domain Hash signatures for a similar level of security.

Signatures produced by the BLS signature scheme are often referred to as short
signatures, BLS short signatures, or simply BLS signatures. The signature
scheme is provably secure (the scheme is existentially unforgeable under
adaptive chosen-message attacks) in the random oracle model assuming the
intractability of the computational Diffie–Hellman problem in a gap
Diffie–Hellman group.

## Rust Crate

This crate implements a version of the BLS signature scheme. The implementation
doesn't match up with the [BLS signatures specification at the IETF]. This
implementation uses the BN254 curve defined here:
[https://neuromancer.sk/std/bn/bn254](https://neuromancer.sk/std/bn/bn254)
(sometimes called BN254N), and should not to be confused with the one used in
Ethereum.

[BLS signatures specification at the IETF]: https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-bls-signature-05

To start, all that is needed is to add this to your `Cargo.toml`.

```toml
[dependencies]
indy-blssignatures = "0.1"
```

This implementation of BLS Signatures was initially in the [Hyperledger Ursa]
project and is used in the [Hyperledger Indy Node] and [Hyperledger Indy Plenum]
repositories/artifacts.

A Python wrapper for this crate can be found in the [indy-bls-wrapper-python] repository.

[Hyperledger Ursa]: https://github.com/hyperledger/ursa
[Hyperledger Indy Node]: https://github.com/hyperledger/indy-node
[Hyperledger Indy Plenum]: https://github.com/hyperledger/indy-plenum
[indy-bls-wrapper-python]: https://github.com/hyperledger/indy-bls-wrapper-python
