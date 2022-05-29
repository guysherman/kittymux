package kitty

import (
	. "github.com/smartystreets/goconvey/convey"
	"testing"
)

func TestSendCommand(t *testing.T) {
	Convey("should generate correct string including \\n at the end", t, func() {
		ce := &MockCommandExecutor{}
		kc := KittyConnector{}

		kc.SendCommand(ce, []string{"nvim", "."}, 5)
		So(
			ce.GetSavedArgs(),
			ShouldResemble,
			[]string{
				"send-text",
				"-m",
				"id:5",
				"'nvim .\\n'",
			},
		)
	})

	Convey("should surround multi-word arguments in double-quotes", t, func() {
		ce := &MockCommandExecutor{}
		kc := KittyConnector{}

		kc.SendCommand(ce, []string{"echo", "foo bar baz", ">", "out.txt"}, 23)
		So(
			ce.GetSavedArgs(),
			ShouldResemble,
			[]string{
				"send-text",
				"-m",
				"id:23",
				"'echo \"foo bar baz\" > out.txt\\n'",
			},
		)
	})
}
