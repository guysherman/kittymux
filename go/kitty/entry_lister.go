package kitty

import "fmt"

const PRE_INDENT = "    "
const INDENT = " └─ "

const TAB_INDENT = " ├─ "
const LAST_TAB_INDENT = " └─ "
const WINDOW_INDENT = " │  ├─ "
const LAST_WINDOW_INDENT = " │  └─ "
const LAST_TAB_WINDOW_INDENT = "    ├─ "
const LAST_TAB_LAST_WINDOW_INDENT = "    └─ "

type EntryLister interface {
	EntryList(windowLister *WindowLister, commandExecutor *CommandExecutor) []WindowListEntry
}

type EntryListerBase struct{}

func (el *EntryListerBase) EntryList(windowLister WindowLister, commandExecutor CommandExecutor) []WindowListEntry {
	windowList := windowLister.WindowList(commandExecutor)
	entryList := make([]WindowListEntry, 0)
	for i := 0; i < len(windowList); i++ {
		entryList = processOsWindow(entryList, windowList[i])
	}
	return entryList
}

func processOsWindow(entryList []WindowListEntry, osWindow KittyOsWindow) []WindowListEntry {
	entry := WindowListEntry{
		Id:                osWindow.Id,
		Text:              fmt.Sprint("kitty:", osWindow.Id),
		EntryType:         OsWindow,
		IsFocused:         osWindow.Is_focused,
		TabIsFocused:      osWindow.Is_focused,
		OsWindowIsFocused: osWindow.Is_focused,
	}

	entryList = append(entryList, entry)

	for i := 0; i < len(osWindow.Tabs); i++ {
		tab := osWindow.Tabs[i]
		entryList = processTab(entryList, osWindow, tab, i == len(osWindow.Tabs)-1)
	}

	return entryList
}

func processTab(entryList []WindowListEntry, osWindow KittyOsWindow, tab KittyTab, isLast bool) []WindowListEntry {
	indent := TAB_INDENT
	if isLast {
		indent = LAST_TAB_INDENT
	}

	star := ""
	if tab.Is_focused {
		star = "*"
	}

	entry := WindowListEntry{
		Id:                tab.Id,
		Title:             tab.Title,
		EntryType:         Tab,
		IsFocused:         tab.Is_focused,
		TabIsFocused:      tab.Is_focused,
		OsWindowIsFocused: osWindow.Is_focused,
		Tab:               &tab,
		Text:              fmt.Sprintf("%s%s (tab:%d) %s", indent, tab.Title, tab.Id, star),
	}

	entryList = append(entryList, entry)

	for i := 0; i < len(tab.Windows); i++ {
		window := tab.Windows[i]
		entryList = processWindow(entryList, osWindow, tab, window, isLast, i == len(tab.Windows)-1)
	}

	return entryList
}

func processWindow(entryList []WindowListEntry, osWindow KittyOsWindow, tab KittyTab, window KittyWindow, parentIsLast bool, isLast bool) []WindowListEntry {
	indent := ""
	if parentIsLast && isLast {
		indent = LAST_TAB_LAST_WINDOW_INDENT
	} else if parentIsLast {
		indent = LAST_TAB_WINDOW_INDENT
	} else if isLast {
		indent = LAST_WINDOW_INDENT
	} else {
		indent = WINDOW_INDENT
	}

	star := ""
	if window.Is_focused {
		star = "*"
	}

	entry := WindowListEntry{
		Id:                window.Id,
		Cwd:               window.Cwd,
		Pid:               window.Pid,
		Title:             window.Title,
		EntryType:         Window,
		Tab:               &tab,
		IsFocused:         window.Is_focused,
		TabIsFocused:      tab.Is_focused,
		OsWindowIsFocused: osWindow.Is_focused,
		Text:              fmt.Sprintf("%s%s (id:%d; pid:%d) %s", indent, window.Title, window.Id, window.Pid, star),
	}

	return append(entryList, entry)
}
