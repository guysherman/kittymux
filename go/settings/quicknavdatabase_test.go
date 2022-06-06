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

	Convey("Database Actions", t, func() {
		Convey("Given empty database", func() {
			Convey("Set shortcut", func() {
				qnd := MockQuickNavDao{}
				qnd.SetWriteReturnValue(MockQuickNavDaoWriteReturn{Err: nil})
				qnd.SetReadReturnValue(MockQuickNavDaoReadReturn{
					Db: QuickNavDatabase{
						QuickNavs: map[string][]QuickNavHandle{},
					},
					Err: nil,
				})
				db := NewQuickNavDatabase(&qnd)
				expectedDb := QuickNavDatabase{
					QuickNavs: map[string][]QuickNavHandle{
						"a": {
							{
								EntryId:   5,
								EntryType: kitty.Window,
							},
						},
					},
					dao: &qnd,
				}

				db = db.SetShortcut("a", QuickNavHandle{EntryId: 5, EntryType: kitty.Window})
				So(db, ShouldResemble, expectedDb)
				So(qnd.GetCalls().Write.QuickNavs, ShouldResemble, expectedDb)
			})
		})

		Convey("Given handle already in db", func() {
			qnd := MockQuickNavDao{}
			qnd.SetWriteReturnValue(MockQuickNavDaoWriteReturn{Err: nil})
			qnd.SetReadReturnValue(MockQuickNavDaoReadReturn{
				Db: QuickNavDatabase{
					QuickNavs: map[string][]QuickNavHandle{
						"b": {
							{
								EntryId:   5,
								EntryType: kitty.Window,
							},
						},
					},
				},
				Err: nil,
			})

			Convey("Set shortcut", func() {
				db := NewQuickNavDatabase(&qnd)
				expectedDb := QuickNavDatabase{
					QuickNavs: map[string][]QuickNavHandle{
						"a": {
							{
								EntryId:   5,
								EntryType: kitty.Window,
							},
						},
					},
					dao: &qnd,
				}

				db = db.SetShortcut("a", QuickNavHandle{EntryId: 5, EntryType: kitty.Window})
				So(db, ShouldResemble, expectedDb)
				So(qnd.GetCalls().Write.QuickNavs, ShouldResemble, expectedDb)
			})

			Convey("Remove shortcut", func() {
				db := NewQuickNavDatabase(&qnd)
				expectedDb := QuickNavDatabase{
					QuickNavs: map[string][]QuickNavHandle{},
					dao:       &qnd,
				}

				db = db.RemoveHandle(QuickNavHandle{EntryId: 5, EntryType: kitty.Window})
				So(db, ShouldResemble, expectedDb)
				So(qnd.GetCalls().Write.QuickNavs, ShouldResemble, expectedDb)

			})
		})
	})
}
