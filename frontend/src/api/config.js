const BASE_URL = "/api"; // ← pakai proxy

export function getHeaders() {
  return {
    "Content-Type": "application/json",
    "X-API-SECRET": "your api secret",
  };
}

export { BASE_URL };
