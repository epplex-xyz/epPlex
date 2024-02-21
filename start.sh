if ! pgrep -x "solana-test-validator" >/dev/null; then
    yarn validator
fi
yarn airdrop
yarn deploy-all
yarn copy-to-sdk
yarn copy-admin-sdk