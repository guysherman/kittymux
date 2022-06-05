package settings

import (
	"testing"

	"github.com/guysherman/kittymux/kitty"
	. "github.com/smartystreets/goconvey/convey"
)

func TestQuickNavDatabase(t *testing.T) {
	Convey("Converts to map by entry id", t, func() {
		db := QuickNavDatabase{
			QuickNavs: map[string][]QuickNavHandle{
				"a": {
					{
						EntryId:   1,
						EntryType: kitty.Tab,
					},
					{
						EntryId:   10,
						EntryType: kitty.Window,
					},
				},
			},
		}

		expected := map[string]string{
			"t:1":  "a",
			"w:10": "a",
		}

		actual := db.ShortcutsByEntryId()

		So(actual, ShouldResemble, expected)
	})
}
