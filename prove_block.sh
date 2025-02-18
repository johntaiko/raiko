#!/bin/bash

# Use the first command line argument as the chain name
chain="$1"
# Use the second command line argument as the proof type
proof="$2"
# Use the third(/fourth) parameter(s) as the block number as a range
rangeStart="$3"
rangeEnd="$4"

# Check the caain name and set the corresponding RPC values
if [ "$chain" == "ethereum" ]; then
  rpc="https://rpc.ankr.com/eth"
elif [ "$chain" == "taiko_a6" ]; then
  rpc="https://rpc.katla.taiko.xyz"
  l1Rpc="https://l1rpc.katla.taiko.xyz"
  beaconRpc="https://l1beacon.hekla.taiko.xyz"
elif [ "$chain" == "taiko_a7" ]; then
  rpc="https://rpc.hekla.taiko.xyz/"
  l1Rpc="https://l1rpc.hekla.taiko.xyz/"
  beaconRpc="https://l1beacon.hekla.taiko.xyz"
else
  echo "Invalid chain name. Please use 'ethereum', 'taiko_a6' or 'taiko_a7'."
  exit 1
fi

if [ "$proof" == "native" ]; then
  proofParam='
    "proof_type": "native"
  '
elif [ "$proof" == "sp1" ]; then
  proofParam='
    "proof_type": "sp1"
  '
elif [ "$proof" == "sgx" ]; then
  proofParam='
    "proof_type": "sgx",
    "sgx": {
        "instance_id": 456,
        "setup": true,
        "bootstrap": true,
        "prove": true,
        "input_path": null
      }
    '
elif [ "$proof" == "risc0" ]; then
  proofParam='
    "proof_type": "risc0",
    "risc0": {
      "bonsai": false,
      "snark": false,
      "profile": true,
      "execution_po2": 18
    }
  '
elif [ "$proof" == "risc0-bonsai" ]; then
  proofParam='
    "proof_type": "risc0",
    "risc0": {
      "bonsai": true,
      "snark": true,
      "profile": false,
      "execution_po2": 20
    }
  '
else
  echo "Invalid proof name. Please use 'native', 'risc0[-bonsai]', 'sp1', or 'sgx'."
  exit 1
fi

if [ "$rangeStart" == "" ]; then
  echo "Please specify a valid block range like \"10\" or \"10 20\""
  exit 1
fi

if [ "$rangeEnd" == "" ]; then
  rangeEnd=$rangeStart
fi

prover="0x70997970C51812dc3A010C7d01b50e0d17dc79C8"
graffiti="8008500000000000000000000000000000000000000000000000000000000000"

for block in $(eval echo {$rangeStart..$rangeEnd});
do
  echo "- proving block $block"
  curl --location --request POST 'http://localhost:8080' \
       --header 'Content-Type: application/json' \
       --data-raw "{
         \"jsonrpc\": \"2.0\",
         \"id\": 1,
         \"method\": \"proof\",
         \"params\": [
           {
             \"network\": \"$chain\",
             \"rpc\": \"$rpc\",
             \"l1_rpc\": \"$l1Rpc\",
             \"beacon_rpc\": \"$beaconRpc\",
             \"block_number\": $block,
             \"prover\": \"$prover\",
             \"graffiti\": \"$graffiti\",
             $proofParam
           }
         ]
       }"
  echo "\\n"
done
