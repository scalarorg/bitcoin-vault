use alloy::sol;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IERC20,
    "abi/IERC20.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IGateway,
    "abi/IGateway.json"
);
