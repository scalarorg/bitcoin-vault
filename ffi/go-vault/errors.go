package vault

import "errors"

var (
	ErrInvalidScript  = errors.New("invalid script")
	ErrParsingFailed  = errors.New("parsing failed")
	ErrInvalidNetwork = errors.New("invalid network")
)
