import { BASE_URL, getHeaders } from "./config";

export async function getProfile() {
  const res = await fetch(`${BASE_URL}/user/profile`, {
    method: "GET",
    headers: getHeaders(),
    credentials: "include",
  });
  return res.json();
}
