const BASE_URL = import.meta.env.VITE_API_URL || "/api";

export function getHeaders() {
  return {
    "Content-Type": "application/json",
    "X-API-SECRET": import.meta.env.VITE_API_SECRET,
  };
}

export { BASE_URL };
