package vault

import "errors"

var (
	ErrInvalidScript                        = errors.New("invalid script")
	ErrParsingFailed                        = errors.New("parsing failed")
	ErrInvalidNetwork                       = errors.New("invalid network")
	ErrFailedToSign                         = errors.New("failed to sign")
	ErrFailedToSignAndCollectSigs           = errors.New("failed to sign and collect sigs")
	ErrInvalidPsbt                          = errors.New("invalid psbt")
	ErrNoTapScriptSigs                      = errors.New("no tap script sigs")
	ErrFailedToAggregateTapScriptSigs       = errors.New("failed to aggregate tap script sigs")
	ErrFailedToFinalizePsbtAndExtractTx     = errors.New("failed to finalize psbt and extract tx")
	ErrFailedToBuildCovenantOnlyUnstakingTx = errors.New("failed to build custodian only unstaking tx")
)
