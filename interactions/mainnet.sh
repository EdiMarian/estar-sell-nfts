PROJECT="${PWD}"

COLLECTION_ID="BLOODSHED-a62781"
COLLECTION_ID_HEX="0x$(echo -n ${COLLECTION_ID} | xxd -p -u | tr -d '\n')"

OURO_ID="OURO-9ecd6a"
OURO_ID_HEX="0x$(echo -n ${OURO_ID} | xxd -p -u | tr -d '\n')"

SFTS_ID="DHLOTTERY-f6fc85"
SFTS_ID_HEX="0x$(echo -n ${SFTS_ID} | xxd -p -u | tr -d '\n')"

VST_ID="VST-c40502"
VST_ID_HEX="0x$(echo -n ${VST_ID} | xxd -p -u | tr -d '\n')"

USDC_ID="USDC-c76f1f"
USDC_ID_HEX="0x$(echo -n ${USDC_ID} | xxd -p -u | tr -d '\n')"

PEM_FILE="/home/edi-marian/Desktop/wallet-estar/wallet-owner.pem"
PROXY=https://gateway.multiversx.com
CHAINID=1
ADDRESS=erd1qqqqqqqqqqqqqpgqcc2dakhdz23hk8gvlnn054uhzzeewn5xwmfsyqdssd
MY_ADDRESS=erd1szcgm7vq3tmyxfgd4wd2k2emh59az8jq5jjpj9799a0k59u0wmfss4vw3v

deploy() {
  mxpy --verbose contract deploy --bytecode="$PROJECT/output/sell-nfts.wasm" --recall-nonce --pem=${PEM_FILE} \
    --gas-limit=60000000 --send --outfile="${PROJECT}/interactions/logs/deploy.json" --metadata-payable-by-sc \
    --proxy=${PROXY} --chain=${CHAINID} \
    --arguments $COLLECTION_ID_HEX erd1qqqqqqqqqqqqqpgq8vem4kq208phuhny9gfy9qza47np63gq0a0s7edevj || return
}

updateContract() {
  mxpy --verbose contract upgrade ${ADDRESS} --bytecode="$PROJECT/output/sell-nfts.wasm" --recall-nonce --pem=${PEM_FILE} \
    --gas-limit=60000000 --send --outfile="${PROJECT}/interactions/logs/deploy.json" --metadata-payable-by-sc \
    --proxy=${PROXY} --chain=${CHAINID} \
    --arguments $COLLECTION_ID_HEX erd1qqqqqqqqqqqqqpgq8vem4kq208phuhny9gfy9qza47np63gq0a0s7edevj
}

addSwapOperation() {
  mxpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=12000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="addSwapOperation" \
    --arguments erd1qqqqqqqqqqqqqpgqzna793tw3sd2vshvzkt0ttwu9pdyj97e0a0s3j3dv2 $USDC_ID_HEX \
    --send \
    --outfile="${PROJECT}/interactions/logs/distribute.json"
}

clearSwapOperations() {
  mxpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=12000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="clearSwapOperations" \
    --send \
    --outfile="${PROJECT}/interactions/logs/distribute.json"
}

addToWhitelist() {
  mxpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=12000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="addToWhitelist" \
    --arguments $MY_ADDRESS \
    --send \
    --outfile="${PROJECT}/interactions/logs/distribute.json"
}

removeFromWhitelist() {
  mxpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=12000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="removeFromWhitelist" \
    --arguments $MY_ADDRESS \
    --send \
    --outfile="${PROJECT}/interactions/logs/distribute.json"
}

withdrawToken() {
  mxpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=12000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="withdrawToken" \
    --arguments $COLLECTION_ID_HEX 1214 \
    --send \
    --outfile="${PROJECT}/interactions/logs/distribute.json"
}

setFirstTokenPayment() {
  mxpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=12000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="setFirstTokenPayment" \
    --arguments $OURO_ID_HEX 10000000000000000 \
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
    --arguments $VST_ID_HEX 10000000000000000 \
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
    --arguments $ADDRESS 7 $COLLECTION_ID_HEX 128 1 $COLLECTION_ID_HEX 538 1 $COLLECTION_ID_HEX 1020 1 $COLLECTION_ID_HEX 1214 1 $COLLECTION_ID_HEX 148 1 $COLLECTION_ID_HEX 1686 1 $COLLECTION_ID_HEX 430 1 $method_name \
    --send \
    --outfile="${PROJECT}/interactions/logs/distribute.json"
}

mintWithOuro() {
  method_name="0x$(echo -n 'mint' | xxd -p -u | tr -d '\n')"
  mxpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=30000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="ESDTTransfer" \
    --arguments $OURO_ID_HEX 1040000000000000000 $method_name 1 \
    --send \
    --outfile="${PROJECT}/interactions/logs/distribute.json"
}

setUserFarm() {
  mxpy --verbose contract call erd1qqqqqqqqqqqqqpgqplw6qj45dvvdfcf7dcl30rp3y5zl0arawmfs6ratsj --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=30000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="setUserFarm" \
    --arguments $MY_ADDRESS 0 \
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
    --gas-limit=30000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="ESDTTransfer" \
    --arguments $VST_ID_HEX 10000000000000000 $method_name 1 \
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

getDexRouterAddress() {
  mxpy --verbose contract query ${ADDRESS} --function="getDexRouterAddress" \
    --proxy=${PROXY}
}

getNonces() {
  mxpy --verbose contract query ${ADDRESS} --function="getNonces" \
    --proxy=${PROXY}
}

getSwapOperations() {
  mxpy --verbose contract query ${ADDRESS} --function="getSwapOperations" \
    --proxy=${PROXY}
}