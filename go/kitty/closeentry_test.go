package kitty

import (
	. "github.com/smartystreets/goconvey/convey"
	"testing"
)

func TestCloseEntry(t *testing.T) {
	Convey("Entry is an OS Window", t, func() {
		ce := &MockCommandExecutor{}
		kc := NewKittyConnector(ce)
		entry := WindowListEntry{
			Id:                1,
			Text:              "TestEntry",
			EntryType:         OsWindow,
			Pid:               1,
			Cwd:               "/foo",
			Title:             "TestEntry",
			IsFocused:         false,
			TabIsFocused:      false,
			OsWindowIsFocused: false,
			Tab:               nil,
		}

		ce.SetReturnValue("")

		kc.CloseEntry(entry)
		So(ce.GetSavedArgs(), ShouldResemble, [][]string(nil))
	})

	Convey("Entry is a tab", t, func() {
		ce := &MockCommandExecutor{}
		kc := NewKittyConnector(ce)
		entry := WindowListEntry{
			Id:                1,
			Text:              "TestEntry",
			EntryType:         Tab,
			Pid:               1,
			Cwd:               "/foo",
			Title:             "TestEntry",
			IsFocused:         false,
			TabIsFocused:      false,
			OsWindowIsFocused: false,
			Tab:               nil,
		}

		ce.SetReturnValue("0")

		kc.CloseEntry(entry)
		So(ce.GetSavedArgs()[0], ShouldResemble, []string{"close-tab", "-m", "id:1"})
	})

	Convey("Entry is window", t, func() {
		ce := &MockCommandExecutor{}
		kc := NewKittyConnector(ce)
		entry := WindowListEntry{
			Id:                1,
			Text:              "TestEntry",
			EntryType:         Window,
			Pid:               1,
			Cwd:               "/foo",
			Title:             "TestEntry",
			IsFocused:         false,
			TabIsFocused:      false,
			OsWindowIsFocused: false,
			Tab:               nil,
		}

		ce.SetReturnValue("0")

		kc.CloseEntry(entry)
		So(ce.GetSavedArgs()[0], ShouldResemble, []string{"close-window", "-m", "id:1"})
	})
}
