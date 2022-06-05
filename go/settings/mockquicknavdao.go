package settings

type MockQuickNavDao struct {
	calls   MockQuickNavDaoCalls
	returns MockQuickNavDaoReturns
}

type MockQuickNavDaoCalls struct {
	Read  MockQuickNavDaoReadCall
	Write MockQuickNavDaoWriteCall
}

type MockQuickNavDaoReadCall struct {
	Filepath string
}

type MockQuickNavDaoWriteCall struct {
	QuickNavs QuickNavDatabase
	Filepath  string
}

type MockQuickNavDaoReturns struct {
	Read  MockQuickNavDaoReadReturn
	Write MockQuickNavDaoWriteReturn
}

type MockQuickNavDaoReadReturn struct {
	Db  QuickNavDatabase
	Err error
}

type MockQuickNavDaoWriteReturn struct {
	Err error
}

func (m *MockQuickNavDao) SetReadReturnValue(returnValue MockQuickNavDaoReadReturn) {
	m.returns.Read = returnValue
}

func (m *MockQuickNavDao) SetWriteReturnValue(returnValue MockQuickNavDaoWriteReturn) {
	m.returns.Write = returnValue
}

func (m *MockQuickNavDao) GetCalls() MockQuickNavDaoCalls {
	return m.calls
}

func (m *MockQuickNavDao) Read(filepath string) (QuickNavDatabase, error) {
	m.calls.Read = MockQuickNavDaoReadCall{Filepath: filepath}
	return m.returns.Read.Db, m.returns.Read.Err
}

func (m *MockQuickNavDao) Write(quickNavs QuickNavDatabase, filepath string) error {
	m.calls.Write = MockQuickNavDaoWriteCall{QuickNavs: quickNavs, Filepath: filepath}
	return m.returns.Write.Err
}
