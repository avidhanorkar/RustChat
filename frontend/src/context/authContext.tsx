import React, { createContext, useState, useEffect, useContext } from "react";
import type { ReactNode } from "react";
import { jwtDecode } from "jwt-decode";
import type { JwtPayload } from "jwt-decode";
import { useNavigate } from "react-router-dom";

interface MyJwtPayload extends JwtPayload {
  user_id: string;
  email?: string;
  role?: string;
  iat: number;
  exp: number;
}

type AuthContextType = {
  user: MyJwtPayload | null;
  setUser: React.Dispatch<React.SetStateAction<MyJwtPayload | null>>;
  login: (token: string) => boolean;
  logout: () => void;
  isAuthenticated: boolean;
  isLoading: boolean;
};

const AuthContext = createContext<AuthContextType | undefined>(undefined);

// Custom hook to use auth context
export const useAuth = () => {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};

type AuthProviderProps = {
  children: ReactNode;
};

const AuthProvider = ({ children }: AuthProviderProps) => {
  const [user, setUser] = useState<MyJwtPayload | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const navigate = useNavigate();

  // Check if token is expired
  const isTokenExpired = (token: MyJwtPayload): boolean => {
    const currentTime = Date.now() / 1000;
    return token.exp < currentTime;
  };

  // Login function that validates and stores token
  const login = (token: string): boolean => {
    try {
      const decodedUser = jwtDecode<MyJwtPayload>(token);
      
      if (isTokenExpired(decodedUser)) {
        console.warn("Token is expired");
        return false;
      }
      
      sessionStorage.setItem('token', token);
      setUser(decodedUser);
      return true;
    } catch (error) {
      console.error("Error decoding token:", error);
      return false;
    }
  };

  const logout = () => {
    sessionStorage.removeItem('token');
    setUser(null);
    navigate("/");
  };

  useEffect(() => {
    const initializeAuth = () => {
      setIsLoading(true);
      const token = sessionStorage.getItem('token');

      if (token) {
        try {
          const decodedUser = jwtDecode<MyJwtPayload>(token);
          
          if (isTokenExpired(decodedUser)) {
            console.warn("Stored token is expired, removing it");
            sessionStorage.removeItem('token');
            setUser(null);
          } else {
            setUser(decodedUser);
          }
        } catch (error) {
          console.error("Error decoding stored token:", error);
          sessionStorage.removeItem('token');
          setUser(null);
        }
      }
      
      setIsLoading(false);
    };

    initializeAuth();
  }, []);

  // Auto-logout when token expires
  useEffect(() => {
    if (user && isTokenExpired(user)) {
      console.log("Token expired, logging out");
      logout();
    }
  }, [user]);

  const value: AuthContextType = {
    user,
    setUser,
    login,
    logout,
    isAuthenticated: !!user && !isTokenExpired(user),
    isLoading,
  };

  return (
    <AuthContext.Provider value={value}>
      {children}
    </AuthContext.Provider>
  );
};

export { AuthProvider, AuthContext };