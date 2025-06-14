// auth.js - Create this file and include it in your HTML pages

// Function to check if JWT is valid (not expired)
function isJWTValid() {
  const jwt = localStorage.getItem('jwt');
  if (!jwt) return false;

  try {
    // Decode JWT payload (middle part)
    const payload = JSON.parse(atob(jwt.split('.')[1]));
    const currentTime = Math.floor(Date.now() / 1000);

    // Check if token is expired
    return payload.exp > currentTime;
  } catch (error) {
    console.error('Invalid JWT format:', error);
    return false;
  }
}

// Function to get user role from localStorage
function getUserRole() {
  return localStorage.getItem('userRole');
}

// Function to check if user is authenticated
function isAuthenticated() {
  return isJWTValid() && getUserRole();
}

// Function to clear auth data
function clearAuthData() {
  localStorage.removeItem('jwt');
  localStorage.removeItem('userRole');
  localStorage.removeItem('userName');
  localStorage.removeItem('userId');
}

// Function to redirect based on authentication status
function redirectIfAuthenticated() {
  if (isAuthenticated()) {
    const role = getUserRole();
    if (role === 'admin') {
      window.location.href = '/admin_dashboard.html';
    } else {
      window.location.href = '/dashboard.html';
    }
  }
}

// Function to protect routes - redirect to login if not authenticated
function protectRoute(requiredRole = null) {
  if (!isAuthenticated()) {
    // Clear any invalid/expired tokens
    clearAuthData();
    window.location.href = '/login.html';
    return;
  }

  // If specific role is required, check it
  if (requiredRole) {
    const userRole = getUserRole();
    if (userRole !== requiredRole) {
      // User doesn't have required role
      if (userRole === 'admin') {
        window.location.href = '/admin_dashboard.html';
      } else {
        window.location.href = '/dashboard.html';
      }
      return;
    }
  }
}

// Function to protect admin-only routes
function protectAdminRoute() {
  protectRoute('admin');
}

// Function to protect regular user routes
function protectUserRoute() {
  protectRoute('user');
}

function logout() {
  // Clear all authentication data
  clearAuthData();

  // Redirect to login page
  window.location.href = '/login.html';
}