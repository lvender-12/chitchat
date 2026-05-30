const BASE_URL = "/api"; // ← pakai proxy

export function getHeaders() {
  return {
    "Content-Type": "application/json",
    "X-API-SECRET": import.meta.env.VITE_API_SECRET,
  };
}

export { BASE_URL };
