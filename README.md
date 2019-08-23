
<img src="https://cdn-images-1.medium.com/fit/t/1600/480/1*mkZDyso7UvVyutxpP5ugDA.png" width="100%">

# Zero-Knowledge-Proof Use Cases By ZoKrates

ZoKrates is a toolbox for zkSNARKs on Ethereum. We use this toolbox to implement differen zk proof use cases

_The toolbox is a proof-of-concept implementation. It has not been tested for production._

## Getting Started

```
cd target/release
```

There are two use cases: user login and money transfer implementation in 'release' folder.

If you want to play with zokrates toolbox, please get started from [Here](https://zokrates.github.io/gettingstarted.html). 
Have a look at the toolbox's [documentation](https://zokrates.github.io/) for more information about using ZoKrates.  

## Use Case 1.

Here's the scenario (2 parties): 

Current websites store the hash value of the user’s password in their web servers. In order to verify that the client actually knows the password, most websites currently use the method of hashing the password input by the client and comparing it with the stored result.

Zero-Knowledge Proof can protect user’ account from leakage. if Zero-Knowledge Proof can be realized, then the client password is unknown to anyone but can still authenticate the client login. When a server is attacked, the user’s account is still secure because the client’s password is not stored in the web server.

## Use Case 2.

Here's the scenario (3 parties):

<img src="https://hackernoon.com/hn-images/1*qlTlvErILjQL9Y7LewAPQA.png" width="100%">

Reference from: [Here](https://hackernoon.com/wtf-is-zero-knowledge-proof-be5b49735f27).

## File Structure

`out`: after compile client_program.code or server_program.code, we'll get `out` and `out.code` (the later one is human-readable format).

`witness`: compute with witness command, it generates a witness file which contains two outputs as 128 bit number. By concatenating the outputs, we can get the hash for our original value.

`proving.key` & `verification.key`: run the setup phase, there're 2 key files will be generated; proving.key for prover to create a proof.json, verification.key for verifier to check if the proof is valid or not.

`verifier.sol`: use zokrates export-verifier command, we can generate a solidity file includes all keys and verifyTx function.

`verify.js`: run node to delpoy the contract and call verify function