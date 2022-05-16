package kitty

import (
	. "github.com/smartystreets/goconvey/convey"
	"testing"
)

func TestWindowCreator(t *testing.T) {
	Convey("new window in the current tab", t, func() {
		ce := &MockCommandExecutor{}
		wc := WindowCreatorBase{}

		ce.SetReturnValue("99")

		id := wc.CreateWindow(ce, "test title", 0, "", false, "")
		So(id, ShouldEqual, 99)
		So(ce.GetSavedArgs(), ShouldResemble, []string{"new-window", "--title", "test title"})
	})

	Convey("new window in a tab with given id", t, func() {
		ce := &MockCommandExecutor{}
		wc := WindowCreatorBase{}

		ce.SetReturnValue("99")

		id := wc.CreateWindow(ce, "test title", 5, "", false, "")
		So(id, ShouldEqual, 99)
		So(ce.GetSavedArgs(), ShouldResemble, []string{"new-window", "--title", "test title", "-m", "id:5"})
	})

	Convey("new window in a tab with given name", t, func() {
		ce := &MockCommandExecutor{}
		wc := WindowCreatorBase{}

		ce.SetReturnValue("99")

		id := wc.CreateWindow(ce, "test title", 0, "ttt", false, "")
		So(id, ShouldEqual, 99)
		So(ce.GetSavedArgs(), ShouldResemble, []string{"new-window", "--title", "test title", "-m", "title:ttt"})
	})

	Convey("new window in a new tab", t, func() {
		ce := &MockCommandExecutor{}
		wc := WindowCreatorBase{}

		ce.SetReturnValue("99")

		id := wc.CreateWindow(ce, "test title", 0, "test tab", true, "")
		So(id, ShouldEqual, 99)
		So(ce.GetSavedArgs(), ShouldResemble, []string{"new-window", "--title", "test title", "--new-tab", "--tab-title", "\"test tab\""})
	})

	Convey("new window with a specified cwd", t, func() {
		ce := &MockCommandExecutor{}
		wc := WindowCreatorBase{}

		ce.SetReturnValue("99")

		id := wc.CreateWindow(ce, "test title", 0, "", false, "/foo")
		So(id, ShouldEqual, 99)
		So(ce.GetSavedArgs(), ShouldResemble, []string{"new-window", "--title", "test title", "--cwd", "/foo"})
	})
}
