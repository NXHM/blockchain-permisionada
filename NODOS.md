# Información
# Información de los nodos
```shell
Alice
Key type	Key value
Node key	c12b6d18942f5ee8528c8e2baf4e147b5c5c18710926ea492d09cbd9f6c9f82a
Peer identifier generated from the node key	12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2
Peer identifier decoded to hex	0x0024080112201ce5f00ef6e89374afb625f1ae4c1546d31234e87e3c3f51a62b91dd6bfa57df

Bob
Key type	Key value
Node key	6ce3be907dbcabf20a9a5a60a712b4256a54196000a8ed4050d352bc113f8c58
Peer identifier generated from the node key	12D3KooWQYV9dGMFoRzNStwpXztXaBUjtPqi6aU76ZgUriHhKust
Peer identifier decoded to hex	0x002408011220dacde7714d8551f674b8bb4b54239383c76a2b286fa436e93b2b7eb226bf4de7
The two other development accounts—Charlie and Dave—don't have well-known node keys or peer identifiers defined in the genesis configuration. For demonstration purposes, you can use the following keys for these accounts.

Charlie
Key type	Key value
Node key	3a9d5b35b9fb4c42aafadeca046f6bf56107bd2579687f069b42646684b94d9e
Peer identifier generated from the node key	12D3KooWJvyP3VJYymTqG7eH4PM5rN4T2agk5cdNCfNymAqwqcvZ
Peer identifier decoded to hex	0x002408011220876a7b4984f98006dc8d666e28b60de307309835d775e7755cc770328cdacf2e
Dave

Key type	Key value
Node key	a99331ff4f0e0a0434a6263da0a5823ea3afcfffe590c9f3014e6cf620f2b19a
Peer identifier generated from the node key	12D3KooWPHWFrfaJzxPnqnAYAoRUyAHHKqACmEycGTVmeVhQYuZN
Peer identifier decoded to hex	0x002408011220c81bc1d7057a1511eb9496f056f6f53cdfe0e14c8bd5ffca47c70a8d76c1326d
```
# Comandos para iniciar nodos
```shell
./target/release/node-template \
--chain=local \
--base-path /tmp/validator1 \
--alice \
--node-key=c12b6d18942f5ee8528c8e2baf4e147b5c5c18710926ea492d09cbd9f6c9f82a \
--port 30333 \
--rpc-port 9944
```
```shell
./target/release/node-template \
--chain=local \
--base-path /tmp/validator2 \
--bob \
--node-key=6ce3be907dbcabf20a9a5a60a712b4256a54196000a8ed4050d352bc113f8c58 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2 \
--port 30334 \
--rpc-port 9945
```
Se debe de autorizar el tercer nodo por la interfaz gráfica para que pueda unirse, sino se rechaza su conexión
```shell
./target/release/node-template \
--chain=local \
--base-path /tmp/validator3 \
--name charlie  \
--node-key=3a9d5b35b9fb4c42aafadeca046f6bf56107bd2579687f069b42646684b94d9e \
--port 30335 \
--rpc-port=9946 \
--offchain-worker always
```