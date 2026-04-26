import { useState } from "react";
import AuthPage from "./components/AuthPage";
import Dashboard from "./components/Dashboard";
import "./App.css";

function App() {
  const [currentUser, setCurrentUser] = useState("");
  const [isLoggedIn, setIsLoggedIn] = useState(false);

  const handleLoginSuccess = (userId) => {
    setCurrentUser(userId);
    setIsLoggedIn(true);
  };

  const handleLogout = () => {
    setCurrentUser("");
    setIsLoggedIn(false);
  };

  return (
    <div className="app-container">
      {!isLoggedIn ? (
        <AuthPage onLoginSuccess={handleLoginSuccess} />
      ) : (
        <Dashboard user={currentUser} onLogout={handleLogout} />
      )}
    </div>
  );
}

export default App;