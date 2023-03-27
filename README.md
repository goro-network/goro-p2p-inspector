# GoRo Network - P2P Inspector/Probe

This is a CLI program inspired by [libp2p-lookup](https://github.com/mxinden/libp2p-lookup), will be used as a testing tool in the future.

## Installations

First, ensure Rust Toolchain is installed. Then:

```bash
cargo install goro-p2p-inspector
```

## Example Runs - Direct

```bash
❯ goro-p2p-inspector direct --address "/ip4/108.136.33.203/tcp/30333/p2p/12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2"
[2023-03-27T17:18:42.499780696Z INFO  goro_p2p_inspector::peer] Local peer id: "12D3KooWSbKZLBYhkRY64iBYFKZw9128ziQUVRYdCnB7rnafaczd"
[2023-03-27T17:18:42.524846651Z INFO  goro_p2p_inspector::peer] Connection established in 24.20522ms for "/ip4/108.136.33.203/tcp/30333/p2p/12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2" (role override: Dialer)
[2023-03-27T17:18:43.031472030Z INFO  goro_p2p_inspector] Peer information from lookup
********
[Peer ID]
  "12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2"
[Public Key]
  "Ed25519(PublicKey(compressed): 1ce5f0ef6e89374afb625f1ae4c1546d31234e87e3c3f51a62b91dd6bfa57df)"
[Protocol Version]
  "/substrate/1.0"
[Agent Version]
  "GoRo Node/v1.0.0-alpha.3-f7187296f0c (krigan-boot)"
[Observed Address]
  "/ip4/103.81.222.143/tcp/56636"
[Listen Addresses]
  - "/ip4/108.136.33.203/tcp/30333"
  - "/ip4/127.0.0.1/tcp/30333"
  - "/ip4/172.31.22.112/tcp/30333"
  - "/ip4/192.168.233.1/tcp/30333"
  - "/ip4/192.168.96.1/tcp/30333"
  - "/ip4/172.17.0.1/tcp/30333"
[Protocols]
  - "/c2f2d8da3e2d6b63970b6de8e365b84393ee0c69a9aec3a0bc750029a3552d07/block-announces/1"
  - "/sup/block-announces/1"
  - "/c2f2d8da3e2d6b63970b6de8e365b84393ee0c69a9aec3a0bc750029a3552d07/transactions/1"
  - "/sup/transactions/1"
  - "/c2f2d8da3e2d6b63970b6de8e365b84393ee0c69a9aec3a0bc750029a3552d07/grandpa/1"
  - "/paritytech/grandpa/1"
  - "/ipfs/ping/1.0.0"
  - "/ipfs/id/1.0.0"
  - "/ipfs/id/push/1.0.0"
  - "/c2f2d8da3e2d6b63970b6de8e365b84393ee0c69a9aec3a0bc750029a3552d07/kad"
  - "/sup/kad"
  - "/c2f2d8da3e2d6b63970b6de8e365b84393ee0c69a9aec3a0bc750029a3552d07/state/2"
  - "/sup/state/2"
  - "/c2f2d8da3e2d6b63970b6de8e365b84393ee0c69a9aec3a0bc750029a3552d07/sync/warp"
  - "/sup/sync/warp"
  - "/c2f2d8da3e2d6b63970b6de8e365b84393ee0c69a9aec3a0bc750029a3552d07/sync/2"
  - "/sup/sync/2"
  - "/c2f2d8da3e2d6b63970b6de8e365b84393ee0c69a9aec3a0bc750029a3552d07/light/2"
  - "/sup/light/2"
********
```

## Example Runs - DHT

```bash
❯ goro-p2p-inspector dht --dht-network krigan --peer-id 12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2
[2023-03-27T17:19:34.067274136Z INFO  goro_p2p_inspector::peer] Local peer id: "12D3KooWBTxJnQeXoVYWqudvchZVFUf6pqw3cA12fFAW4xFK4g8H"
[2023-03-27T17:19:34.093182277Z INFO  goro_p2p_inspector::peer] Connection established in 24.756767ms for "12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2" (via "/ip4/108.136.33.203/tcp/30333/p2p/")
[2023-03-27T17:19:34.600766919Z INFO  goro_p2p_inspector] Peer information from lookup
********
[Peer ID]
  "12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2"
[Public Key]
  "Ed25519(PublicKey(compressed): 1ce5f0ef6e89374afb625f1ae4c1546d31234e87e3c3f51a62b91dd6bfa57df)"
[Protocol Version]
  "/substrate/1.0"
[Agent Version]
  "GoRo Node/v1.0.0-alpha.3-f7187296f0c (krigan-boot)"
[Observed Address]
  "/ip4/103.81.222.143/tcp/56646"
[Listen Addresses]
  - "/ip4/108.136.33.203/tcp/30333"
  - "/ip4/127.0.0.1/tcp/30333"
  - "/ip4/172.31.22.112/tcp/30333"
  - "/ip4/192.168.233.1/tcp/30333"
  - "/ip4/192.168.96.1/tcp/30333"
  - "/ip4/172.17.0.1/tcp/30333"
[Protocols]
  - "/c2f2d8da3e2d6b63970b6de8e365b84393ee0c69a9aec3a0bc750029a3552d07/block-announces/1"
  - "/sup/block-announces/1"
  - "/c2f2d8da3e2d6b63970b6de8e365b84393ee0c69a9aec3a0bc750029a3552d07/transactions/1"
  - "/sup/transactions/1"
  - "/c2f2d8da3e2d6b63970b6de8e365b84393ee0c69a9aec3a0bc750029a3552d07/grandpa/1"
  - "/paritytech/grandpa/1"
  - "/ipfs/ping/1.0.0"
  - "/ipfs/id/1.0.0"
  - "/ipfs/id/push/1.0.0"
  - "/c2f2d8da3e2d6b63970b6de8e365b84393ee0c69a9aec3a0bc750029a3552d07/kad"
  - "/sup/kad"
  - "/c2f2d8da3e2d6b63970b6de8e365b84393ee0c69a9aec3a0bc750029a3552d07/state/2"
  - "/sup/state/2"
  - "/c2f2d8da3e2d6b63970b6de8e365b84393ee0c69a9aec3a0bc750029a3552d07/sync/warp"
  - "/sup/sync/warp"
  - "/c2f2d8da3e2d6b63970b6de8e365b84393ee0c69a9aec3a0bc750029a3552d07/sync/2"
  - "/sup/sync/2"
  - "/c2f2d8da3e2d6b63970b6de8e365b84393ee0c69a9aec3a0bc750029a3552d07/light/2"
  - "/sup/light/2"
********
```
