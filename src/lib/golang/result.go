package lib

import "strconv"

type status uint8

const (
	statusNotImplemented status = iota
	statusNoSample
	statusNoPartTwo
	statusUnsolved
)

// Informational results can be returned directly with a nil error.
var (
	NotImplemented = Result{status: statusNotImplemented}
	NoSample       = Result{status: statusNoSample}
	NoPartTwo      = Result{status: statusNoPartTwo}
	Unsolved       = Result{status: statusUnsolved}
)

func (value status) String() string {
	switch value {
	case statusNotImplemented:
		return "<Not Implemented>"
	case statusNoSample:
		return "<No Sample Defined>"
	case statusNoPartTwo:
		return "<No Part Two>"
	case statusUnsolved:
		return "<Unsolved>"
	default:
		panic("Invalid result status")
	}
}

// Result holds a formatted answer or a status. Its zero value is NotImplemented.
type Result struct {
	answer    string
	hasAnswer bool
	status    status
}

type SignedInteger interface {
	~int | ~int8 | ~int16 | ~int32 | ~int64
}

type UnsignedInteger interface {
	~uint | ~uint8 | ~uint16 | ~uint32 | ~uint64 | ~uintptr
}

func Signed[T SignedInteger](value T) Result {
	return Text(strconv.FormatInt(int64(value), 10))
}

func Unsigned[T UnsignedInteger](value T) Result {
	return Text(strconv.FormatUint(uint64(value), 10))
}

func Text(value string) Result { return Result{answer: value, hasAnswer: true} }

func (result Result) String() string {
	if result.hasAnswer {
		return result.answer
	}
	return result.status.String()
}
