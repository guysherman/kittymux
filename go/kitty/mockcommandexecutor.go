package kitty

type MockCommandExecutor struct {
	returnValue      string
	onceReturnValues []string
	savedArgs        [][]string
}

func (c *MockCommandExecutor) SetReturnValue(returnValue string) {
	c.returnValue = returnValue
}

func (c *MockCommandExecutor) SetReturnValueOnce(returnValue string) {
	c.onceReturnValues = append(c.onceReturnValues, returnValue)
}

func (c *MockCommandExecutor) GetSavedArgs() [][]string {
	return c.savedArgs
}

func (c *MockCommandExecutor) ExecuteCommand(args []string) string {
	c.savedArgs = append(c.savedArgs, args)

	if len(c.onceReturnValues) > 0 {
		retVal := c.onceReturnValues[0]
		c.onceReturnValues = c.onceReturnValues[1:len(c.onceReturnValues)]
		return retVal
	}
	return c.returnValue
}
