pkill -f solana-test-validator || true
yarn validator &
yarn deploy-all 
yarn copy-to-sdk
