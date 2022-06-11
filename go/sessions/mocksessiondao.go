package sessions

type MockSessionDao struct {
	Calls   MockSessionDaoCalls
	Returns MockSessionDaoReturns
}

type MockSessionDaoCalls struct {
	Read  []MockSessionDaoReadCall
	Write []MockSessionDaoWriteCall
}

type MockSessionDaoReadCall struct {
	filepath string
}

type MockSessionDaoWriteCall struct {
	Session  Session
	Filepath string
}

type MockSessionDaoReturns struct {
	Read      MockSessionDaoReadReturn
	ReadOnce  []MockSessionDaoReadReturn
	Write     error
	WriteOnce []error
}

type MockSessionDaoReadReturn struct {
	session Session
	err     error
}

func (s *MockSessionDao) SetReadReturnValue(returnValue MockSessionDaoReadReturn) {
	s.Returns.Read = returnValue
}

func (s *MockSessionDao) SetReadReturnValueOnce(returnValue MockSessionDaoReadReturn) {
	s.Returns.ReadOnce = append(s.Returns.ReadOnce, returnValue)
}

func (s *MockSessionDao) GetCalls() MockSessionDaoCalls {
	return s.Calls
}

func (s *MockSessionDao) Read(filepath string) (Session, error) {
	s.Calls.Read = append(s.Calls.Read, MockSessionDaoReadCall{filepath: filepath})

	returnCount := len(s.Returns.ReadOnce)
	if returnCount > 0 {
		returnValue := s.Returns.ReadOnce[returnCount-1]
		s.Returns.ReadOnce = s.Returns.ReadOnce[:returnCount-1]
		return returnValue.session, returnValue.err
	}

	return s.Returns.Read.session, s.Returns.Read.err
}

func (s *MockSessionDao) Write(session Session, filepath string) error {
	s.Calls.Write = append(s.Calls.Write, MockSessionDaoWriteCall{Session: session, Filepath: filepath})

	returnCount := len(s.Returns.WriteOnce)
	if returnCount > 0 {
		returnValue := s.Returns.WriteOnce[returnCount-1]
		s.Returns.WriteOnce = s.Returns.WriteOnce[:returnCount-1]
		return returnValue
	}

	return s.Returns.Write
}
