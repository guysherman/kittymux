package settings

import (
	"os"
	"testing"

	"github.com/guysherman/kittymux/kitty"
	. "github.com/smartystreets/goconvey/convey"
)

func TestQuickNavDao(t *testing.T) {
	Convey("Serialize then Deserialize yields equivalent object", t, func() {
		testFilePath := "test_quick_navs_test.json"
		expected := QuickNavDatabase{
			QuickNavs: map[string][]QuickNavHandle{
				"a": {
					{
						EntryId:   1,
						EntryType: kitty.Window,
					},
				},
				"b": {
					{
						EntryId:   2,
						EntryType: kitty.Tab,
					},
				},
			},
		}

		qnd := &QuickNavDao{}

		err := qnd.Write(expected, testFilePath)
		So(err, ShouldBeNil)

		db, err := qnd.Read(testFilePath)
		So(err, ShouldBeNil)
		So(db, ShouldResemble, expected)

		os.Remove(testFilePath)

	})

	Convey("Deserialize from nothing yeilds empty map", t, func() {
		qnd := &QuickNavDao{}
		expected := QuickNavDatabase{
			QuickNavs: map[string][]QuickNavHandle{},
		}

		db, _ := qnd.Read("foozlenoozle.json")
		So(db, ShouldResemble, expected)
	})
}
