import Auth from './pages/Auth'
import { BrowserRouter, Routes, Route } from 'react-router-dom'
import Chat from './pages/Chat'
import { AuthProvider } from './context/authContext'

const App = () => {
  return (
    <div className='bg-black h-screen w-screen'>
      <BrowserRouter>
      <AuthProvider>
        <Routes>
          <Route path="/auth" element={<Auth />} />
          <Route path="/chat" element={<Chat />} />
        </Routes>
        </AuthProvider>
      </BrowserRouter>
    </div>
  )
}

export default App