PROJECT="${PWD}"

COLLECTION_ID="TEST-31bd9d"
COLLECTION_ID_HEX="0x$(echo -n ${COLLECTION_ID} | xxd -p -u | tr -d '\n')"

OURO_ID="OURO-854459"
OURO_ID_HEX="0x$(echo -n ${OURO_ID} | xxd -p -u | tr -d '\n')"

SFTS_ID="SFTS-b4ab0a"
SFTS_ID_HEX="0x$(echo -n ${SFTS_ID} | xxd -p -u | tr -d '\n')"

VST_ID="VST-4c1c9d"
VST_ID_HEX="0x$(echo -n ${VST_ID} | xxd -p -u | tr -d '\n')"

PEM_FILE="/home/edi-marian/Desktop/wallet-estar/wallet-owner.pem"
PROXY=https://devnet-gateway.multiversx.com
CHAINID=D
ADDRESS=erd1qqqqqqqqqqqqqpgq4qlux5s59g022ev66r0yu443v6dt7a47wmfsf59j43
MY_ADDRESS=erd1szcgm7vq3tmyxfgd4wd2k2emh59az8jq5jjpj9799a0k59u0wmfss4vw3v

deploy() {
  mxpy --verbose contract deploy --bytecode="$PROJECT/output/sell-nfts.wasm" --recall-nonce --pem=${PEM_FILE} \
    --gas-limit=60000000 --send --outfile="${PROJECT}/interactions/logs/deploy.json" \
    --proxy=${PROXY} --chain=${CHAINID} \
    --arguments $COLLECTION_ID_HEX || return
}

updateContract() {
  mxpy --verbose contract upgrade ${ADDRESS} --bytecode="$PROJECT/output/sell-nfts.wasm" --recall-nonce --pem=${PEM_FILE} \
    --gas-limit=60000000 --send --outfile="${PROJECT}/interactions/logs/deploy.json" \
    --proxy=${PROXY} --chain=${CHAINID} \
    --arguments $COLLECTION_ID_HEX
}

setFirstTokenPayment() {
  mxpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=12000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="setFirstTokenPayment" \
    --arguments $OURO_ID_HEX 2000000000000000000 \
    --send \
    --outfile="${PROJECT}/interactions/logs/distribute.json"
}

setSecondTokenPayment() {
  mxpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=12000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="setSecondTokenPayment" \
    --arguments $SFTS_ID_HEX 1 3 \
    --send \
    --outfile="${PROJECT}/interactions/logs/distribute.json"
}

setThirdTokenPayment() {
  mxpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=12000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="setThirdTokenPayment" \
    --arguments $VST_ID_HEX 6000000000000000000 \
    --send \
    --outfile="${PROJECT}/interactions/logs/distribute.json"
}

setNfts() {
  method_name="0x$(echo -n 'setNfts' | xxd -p -u | tr -d '\n')"
  mxpy --verbose contract call ${MY_ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=12000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="MultiESDTNFTTransfer" \
    --arguments $ADDRESS 1 $COLLECTION_ID_HEX 1 1 $method_name \
    --send \
    --outfile="${PROJECT}/interactions/logs/distribute.json"
}

mintWithOuro() {
  method_name="0x$(echo -n 'mint' | xxd -p -u | tr -d '\n')"
  mxpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=12000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="ESDTTransfer" \
    --arguments $OURO_ID_HEX 2000000000000000000 $method_name 1 \
    --send \
    --outfile="${PROJECT}/interactions/logs/distribute.json"
}

mintWithSfts() {
  method_name="0x$(echo -n 'mint' | xxd -p -u | tr -d '\n')"
  mxpy --verbose contract call ${MY_ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=12000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="ESDTNFTTransfer" \
    --arguments $SFTS_ID_HEX 1 3 $ADDRESS $method_name 1 \
    --send \
    --outfile="${PROJECT}/interactions/logs/distribute.json"
}

mintWithVst() {
  method_name="0x$(echo -n 'mint' | xxd -p -u | tr -d '\n')"
  mxpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=12000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="ESDTTransfer" \
    --arguments $VST_ID_HEX 6000000000000000000 $method_name 1 \
    --send \
    --outfile="${PROJECT}/interactions/logs/distribute.json"
}

getUserPremiumMints() {
  mxpy --verbose contract query ${ADDRESS} --function="getUserPremiumMints" --arguments $MY_ADDRESS \
    --proxy=${PROXY}
}

getUserMints() {
  mxpy --verbose contract query ${ADDRESS} --function="getUserMints" --arguments $MY_ADDRESS \
    --proxy=${PROXY}
}