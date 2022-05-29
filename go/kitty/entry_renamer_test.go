package kitty

import (
	. "github.com/smartystreets/goconvey/convey"
	"testing"
)

func TestEntryRenamer(t *testing.T) {
	Convey("Entry is an OS Window", t, func() {
		ce := &MockCommandExecutor{}
		ec := EntryRenamerBase{}
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

		ec.RenameEntry(ce, entry, "new name")
		So(ce.GetSavedArgs(), ShouldResemble, []string(nil))
	})

	Convey("Entry is a tab", t, func() {
		ce := &MockCommandExecutor{}
		ec := EntryRenamerBase{}
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

		ec.RenameEntry(ce, entry, "new name")
		So(ce.GetSavedArgs(), ShouldResemble, []string{"set-tab-title", "-m", "id:1", "new name"})
	})

	Convey("Entry is window", t, func() {
		ce := &MockCommandExecutor{}
		ec := EntryRenamerBase{}
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

		ec.RenameEntry(ce, entry, "new name")
		So(ce.GetSavedArgs(), ShouldResemble, []string{"set-window-title", "-m", "id:1", "new name"})
	})

}
