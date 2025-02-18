TEST_ENV=$1
TEST_MODULE=${2:-all}

if [ -z "$TEST_ENV" ]; then
    echo "Usage: $0 <test_env>, e.g. $0 regtest or $0 testnet4"
    exit 1
fi

echo "Running tests for $TEST_ENV"

if [ "$TEST_MODULE" = "custodians" -o "$TEST_MODULE" = "all" ]; then
    echo "Test custodians 🚀"
    SUITES=("test_staking" "test_basic_flow" "test_partial_unstaking" "test_partial_unstaking_multiple_utxos" "test_parallel_signing_multiple_utxos" "test_sign_wrong_pubkey")
    for SUITE in "${SUITES[@]}"; do
        echo "Running tests for $SUITE"
        TEST_ENV=$TEST_ENV ./scripts/test.sh test_custodians "$SUITE"
    done
fi

if [ "$TEST_MODULE" = "upc" -o "$TEST_MODULE" = "all" ]; then
    echo "Test upc 🚀"
    SUITES=("test_staking" "test_user_protocol" "test_custodian_user" "test_custodian_protocol" "test_parallel_custodian_user")
    for SUITE in "${SUITES[@]}"; do
        echo "Running tests for $SUITE"
        TEST_ENV=$TEST_ENV ./scripts/test.sh test_upc "$SUITE"
    done
fi
