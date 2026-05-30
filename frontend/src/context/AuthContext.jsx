import { useState } from "react";
import { AuthContext } from "./AuthContextValue";

export function AuthProvider({ children }) {
  const [isLoggedIn, setIsLoggedIn] = useState(
    () => sessionStorage.getItem("logged_in") === "true",
  );

  const saveToken = () => {
    sessionStorage.setItem("logged_in", "true");
    setIsLoggedIn(true);
  };

  const logout = () => {
    sessionStorage.removeItem("logged_in");
    setIsLoggedIn(false);
  };

  return (
    <AuthContext.Provider value={{ saveToken, logout, isLoggedIn }}>
      {children}
    </AuthContext.Provider>
  );
}
