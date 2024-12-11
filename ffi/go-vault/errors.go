package vault

import "errors"

var (
	ErrInvalidScript              = errors.New("invalid script")
	ErrParsingFailed              = errors.New("parsing failed")
	ErrInvalidNetwork             = errors.New("invalid network")
	ErrFailedToSign               = errors.New("failed to sign")
	ErrFailedToSignAndCollectSigs = errors.New("failed to sign and collect sigs")
)
