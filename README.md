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

This crate implements the BLS signature scheme.

To start, all that is needed is to add this to your `Cargo.toml`.

```toml
[dependencies]
indy-blssignatures = "0.1"
```
