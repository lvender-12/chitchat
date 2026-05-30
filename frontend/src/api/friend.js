import { BASE_URL, getHeaders } from "./config";

export async function getFriendList() {
  const res = await fetch(`${BASE_URL}/user/friend/list`, {
    method: "GET",
    headers: getHeaders(),
    credentials: "include",
  });
  return res.json();
}

export async function getPendingReceived() {
  const res = await fetch(`${BASE_URL}/user/friend/pending/received`, {
    method: "GET",
    headers: getHeaders(),
    credentials: "include",
  });
  return res.json();
}

export async function getPendingSent() {
  const res = await fetch(`${BASE_URL}/user/friend/pending/sent`, {
    method: "GET",
    headers: getHeaders(),
    credentials: "include",
  });
  return res.json();
}

export async function addFriend(userId) {
  const res = await fetch(`${BASE_URL}/user/friend/add/${userId}`, {
    method: "POST",
    headers: getHeaders(),
    credentials: "include",
  });
  return res.json();
}

export async function acceptRequest(userId) {
  const res = await fetch(`${BASE_URL}/user/friend/${userId}/accept`, {
    method: "PATCH",
    headers: getHeaders(),
    credentials: "include",
  });
  return res.json();
}

export async function rejectRequest(userId) {
  const res = await fetch(`${BASE_URL}/user/friend/${userId}/reject`, {
    method: "PATCH",
    headers: getHeaders(),
    credentials: "include",
  });
  return res.json();
}
