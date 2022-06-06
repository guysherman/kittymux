package sessions

import (
	"fmt"
	"log"
	"os"
	"time"

	"github.com/guysherman/kittymux/kitty"
	"github.com/guysherman/kittymux/settings"
)

type ISessionConnector interface {
	LoadSession(sessionName string)
}

type SessionConnector struct {
	kittyConnector kitty.IKittyConnector
	sessionDao     ISessionDao
	quickNavDao    settings.IQuickNavDao
}

func NewSessionConnector(sessionDao ISessionDao, kittyConnector kitty.IKittyConnector, quickNavDao settings.IQuickNavDao) *SessionConnector {
	sc := &SessionConnector{sessionDao: sessionDao, kittyConnector: kittyConnector, quickNavDao: quickNavDao}
	return sc
}

func (sc *SessionConnector) LoadSession(sessionName string) {
	stateDir := settings.GetStateDir()
	sessionPath := fmt.Sprintf("%s/%s.json", stateDir, sessionName)
	quickNavPath := fmt.Sprintf("%s/quicknavs.json", stateDir)

	session, err := sc.sessionDao.Read(sessionPath)
	if err != nil {
		log.Fatal(err)
		os.Exit(-2)
	}

	quickNavs, err := sc.quickNavDao.Read(quickNavPath)

	for i := 0; i < len(session.Windows); i++ {
		sessionWindow := session.Windows[i]
		windowId := 0
		if i == 0 {
			// Create the first window with new tab
			windowId = sc.kittyConnector.CreateWindow(sessionWindow.Title, 0, session.Title, true, sessionWindow.Cwd)

			// Find the tab and get its id
			var tab kitty.KittyTab
			retries := 0
			found := false

			for found != true {
				tab, found = findTab(sc.kittyConnector, session.Title)
				if found == true {
					break
				} else if retries >= 3 {
					log.Fatalf("Could not find tab [%s]", session.Title)
					os.Exit(-4)
				}
				time.Sleep(1000)
				retries++
			}

			// Add the quicknav for the tab
			quickNavs.QuickNavs[session.ShortcutKey] = append(quickNavs.QuickNavs[session.ShortcutKey], settings.QuickNavHandle{
				EntryId:   tab.Id,
				EntryType: kitty.Tab,
			})

			// Focus the tab
			entry := kitty.WindowListEntryFromTab(tab, false, false)
			sc.kittyConnector.FocusEntry(entry)
		} else {
			// create the window
			windowId = sc.kittyConnector.CreateWindow(sessionWindow.Title, 0, session.Title, false, sessionWindow.Cwd)
		}
		// add the quicknav for the window
		quickNavs.QuickNavs[sessionWindow.ShortcutKey] = append(quickNavs.QuickNavs[sessionWindow.ShortcutKey], settings.QuickNavHandle{
			EntryId:   windowId,
			EntryType: kitty.Window,
		})

		// start up the program
		sc.kittyConnector.SendCommand([]string{"cd", sessionWindow.ForegroundProcess.Cwd}, windowId)
		sc.kittyConnector.SendCommand(sessionWindow.ForegroundProcess.Args, windowId)
	}

	sc.quickNavDao.Write(quickNavs, quickNavPath)
}

func findTab(kc kitty.IKittyConnector, tabTitle string) (kitty.KittyTab, bool) {
	windowList := kc.WindowList()

	for i := 0; i < len(windowList); i++ {
		osWindow := windowList[i]
		if osWindow.Is_focused {
			for j := 0; j < len(osWindow.Tabs); j++ {
				tab := osWindow.Tabs[j]
				if tab.Title == tabTitle {
					return tab, true
				}
			}
		}
	}

	return kitty.KittyTab{}, false
}
