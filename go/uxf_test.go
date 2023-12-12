package uxf

import (
    "testing"
)

func Test1(t *testing.T) {
    expected := "Hello uxf v0.1.0\n"
    actual := Hello()
    if actual != expected {
        t.Errorf("expected %q, got %q", expected, actual)
    }
}
