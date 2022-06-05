package settings

import (
	"fmt"

	"github.com/guysherman/kittymux/kitty"
)

type QuickNavDatabase struct {
	QuickNavs map[string][]QuickNavHandle
}

type QuickNavHandle struct {
	EntryId   int
	EntryType kitty.WindowListEntryType
}

func (d *QuickNavDatabase) ShortcutsByEntryId() map[string]string {
	result := map[string]string{}

	for shortcut, handles := range d.QuickNavs {
		for _, handle := range handles {
			entryType := ""
			switch handle.EntryType {
			case kitty.OsWindow:
				entryType = "o"
				break
			case kitty.Tab:
				entryType = "t"
				break
			case kitty.Window:
				entryType = "w"
				break
			}
			entryId := fmt.Sprintf("%s:%d", entryType, handle.EntryId)
			result[entryId] = shortcut
		}
	}
	return result
}
