import { useState, useEffect, useRef } from "react";
import { useNavigate } from "react-router-dom";
import {
  getFriendList,
  getPendingReceived,
  getPendingSent,
  addFriend,
  acceptRequest,
  rejectRequest,
} from "../api/friend";
import { getProfile } from "../api/user";
import { createChatSocket } from "../api/ws";
import { useAuth } from "../context/useAuth";
import "../styles/main.css";

function avatar(name) {
  if (!name) return "?";
  return name
    .split(" ")
    .map((w) => w[0])
    .join("")
    .toUpperCase()
    .slice(0, 2);
}

export default function MainLayout() {
  const navigate = useNavigate();
  const { logout } = useAuth();

  const [tab, setTab] = useState("chats");
  const [friends, setFriends] = useState([]);
  const [received, setReceived] = useState([]);
  const [sent, setSent] = useState([]);
  const [profile, setProfile] = useState(null);
  const [addUserId, setAddUserId] = useState("");
  const [feedback, setFeedback] = useState("");
  const [showProfile, setShowProfile] = useState(false);

  const [activeFriend, setActiveFriend] = useState(null);
  const [messages, setMessages] = useState([]);
  const [input, setInput] = useState("");
  const [connected, setConnected] = useState(false);
  const wsRef = useRef(null);
  const messagesEndRef = useRef(null);
  const profileRef = useRef(null);

  const fetchAll = async () => {
    const [f, r, s] = await Promise.all([
      getFriendList(),
      getPendingReceived(),
      getPendingSent(),
    ]);
    setFriends(Array.isArray(f?.data) ? f.data : Array.isArray(f) ? f : []);
    setReceived(Array.isArray(r?.data) ? r.data : Array.isArray(r) ? r : []);
    setSent(Array.isArray(s?.data) ? s.data : Array.isArray(s) ? s : []);
  };

  useEffect(() => {
    const loadData = async () => {
      await fetchAll();
      const d = await getProfile();
      const p = d?.data || d;
      setProfile(p);
      profileRef.current = p;
    };
    loadData();
  }, []);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  const openChat = (friend) => {
    if (wsRef.current) wsRef.current.close();
    setActiveFriend(friend);
    setMessages([]);
    setConnected(false);

    const convId = friend.conversation_id || friend.id;
    navigate(`/app/chat/${convId}`);

    const ws = createChatSocket(
      convId,
      (data) => {
        const myId = profileRef.current?.uuid;
        if (Array.isArray(data)) {
          // history messages saat pertama connect
          const history = data.map((msg) => ({
            ...msg,
            self: msg.sender_id === myId,
          }));
          setMessages(history);
        } else {
          // pesan realtime masuk
          setMessages((prev) => [
            ...prev,
            { ...data, self: data.sender_id === myId },
          ]);
        }
      },
      () => setConnected(true),
      () => setConnected(false),
    );
    wsRef.current = ws;
  };

  const sendMessage = () => {
    if (!input.trim() || !wsRef.current || !connected) return;
    wsRef.current.send(JSON.stringify({ message: input }));
    const myId = profileRef.current?.uuid;
    setMessages((prev) => [
      ...prev,
      { message: input, self: true, sender_id: myId },
    ]);
    setInput("");
  };

  const handleKeyDown = (e) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      sendMessage();
    }
  };

  const handleAdd = async () => {
    if (!addUserId.trim()) return;
    const res = await addFriend(addUserId.trim());
    setFeedback(res?.message || JSON.stringify(res));
    setAddUserId("");
    setTimeout(() => setFeedback(""), 3000);
    fetchAll();
  };

  const handleAccept = async (userId) => {
    await acceptRequest(userId);
    fetchAll();
  };

  const handleReject = async (userId) => {
    await rejectRequest(userId);
    fetchAll();
  };

  const handleLogout = () => {
    logout();
    navigate("/login");
  };

  return (
    <div className="layout">
      {/* ── SIDEBAR ── */}
      <aside className="sidebar">
        <div className="sidebar-header">
          <span className="sidebar-header-title">chitchat</span>
        </div>

        <div className="sidebar-tabs">
          <button
            className={`tab-btn ${tab === "chats" ? "active" : ""}`}
            onClick={() => setTab("chats")}
          >
            Teman
          </button>
          <button
            className={`tab-btn ${tab === "received" ? "active" : ""}`}
            onClick={() => setTab("received")}
          >
            Permintaan
            {received.length > 0 && (
              <span className="tab-badge">{received.length}</span>
            )}
          </button>
          <button
            className={`tab-btn ${tab === "sent" ? "active" : ""}`}
            onClick={() => setTab("sent")}
          >
            Terkirim
            {sent.length > 0 && (
              <span className="tab-badge">{sent.length}</span>
            )}
          </button>
        </div>

        <div className="add-friend-panel">
          <div className="input-row">
            <input
              type="text"
              placeholder="Tambah teman (User ID)"
              value={addUserId}
              onChange={(e) => setAddUserId(e.target.value)}
              onKeyDown={(e) => e.key === "Enter" && handleAdd()}
            />
            <button className="add-btn" onClick={handleAdd}>
              Tambah
            </button>
          </div>
          {feedback && <p className="feedback-msg">{feedback}</p>}
        </div>

        <div className="sidebar-content">
          {tab === "chats" && (
            <>
              {friends.length === 0 && (
                <p className="empty-state">Belum ada teman 😕</p>
              )}
              {friends.map((f) => (
                <div
                  key={f.id || f.friend_id}
                  className={`friend-item ${activeFriend?.id === f.id ? "active" : ""}`}
                  onClick={() => openChat(f)}
                >
                  <div className="friend-avatar">{avatar(f.name)}</div>
                  <div className="friend-info">
                    <div className="friend-name">{f.name || f.friend_id}</div>
                    <div className="friend-sub">
                      {f.email || "Klik untuk chat"}
                    </div>
                  </div>
                </div>
              ))}
            </>
          )}

          {tab === "received" && (
            <>
              {received.length === 0 && (
                <p className="empty-state">Tidak ada permintaan masuk</p>
              )}
              {received.map((r) => (
                <div key={r.id || r.uuid} className="friend-item">
                  <div className="friend-avatar">{avatar(r.name)}</div>
                  <div className="friend-info">
                    <div className="friend-name">
                      {r.name || r.from_user_id}
                    </div>
                    <div className="friend-sub">
                      {r.email || r.from_user_id}
                    </div>
                  </div>
                  <div className="friend-actions">
                    <button
                      className="accept-btn"
                      onClick={() => handleAccept(r.from_user_id || r.id)}
                    >
                      ✓
                    </button>
                    <button
                      className="reject-btn"
                      onClick={() => handleReject(r.from_user_id || r.id)}
                    >
                      ✕
                    </button>
                  </div>
                </div>
              ))}
            </>
          )}

          {tab === "sent" && (
            <>
              {sent.length === 0 && (
                <p className="empty-state">Belum ada permintaan terkirim</p>
              )}
              {sent.map((s) => (
                <div key={s.id || s.uuid} className="friend-item">
                  <div className="friend-avatar">{avatar(s.name)}</div>
                  <div className="friend-info">
                    <div className="friend-name">{s.name || s.to_user_id}</div>
                    <div className="friend-sub">Menunggu konfirmasi...</div>
                  </div>
                </div>
              ))}
            </>
          )}
        </div>

        {/* Profile bottom */}
        <div style={{ position: "relative" }}>
          {showProfile && (
            <div className="profile-dropdown">
              <div
                style={{
                  padding: "14px 16px",
                  borderBottom: "1px solid var(--border)",
                }}
              >
                <div
                  style={{
                    fontSize: 14,
                    fontWeight: 600,
                    color: "var(--text-primary)",
                  }}
                >
                  {profile?.name || "User"}
                </div>
                <div
                  style={{
                    fontSize: 12,
                    color: "var(--text-muted)",
                    marginTop: 2,
                  }}
                >
                  {profile?.email || ""}
                </div>
              </div>
              <button className="dropdown-item danger" onClick={handleLogout}>
                🚪 Keluar
              </button>
            </div>
          )}
          <div
            className="profile-btn"
            onClick={() => setShowProfile((v) => !v)}
          >
            <div className="profile-avatar">{avatar(profile?.name || "U")}</div>
            <div>
              <div className="profile-name">{profile?.name || "User"}</div>
              <div className="profile-email">{profile?.email || ""}</div>
            </div>
            <span
              style={{
                color: "var(--text-muted)",
                fontSize: 12,
                marginLeft: "auto",
              }}
            >
              {showProfile ? "▼" : "▲"}
            </span>
          </div>
        </div>
      </aside>

      {/* ── CHAT AREA ── */}
      <main className="chat-area">
        {!activeFriend ? (
          <div className="chat-empty">
            <div className="chat-empty-icon">💬</div>
            <h2>Selamat datang!</h2>
            <p>Pilih teman untuk mulai chat</p>
          </div>
        ) : (
          <>
            <div className="chat-header">
              <div className="chat-header-avatar">
                {avatar(activeFriend.name)}
              </div>
              <div className="chat-header-info">
                <div className="chat-header-name">
                  {activeFriend.name || activeFriend.friend_id}
                </div>
                <div
                  className={`chat-header-status ${connected ? "" : "offline"}`}
                >
                  {connected ? "● Online" : "○ Connecting..."}
                </div>
              </div>
            </div>

            <div className="chat-messages">
              {messages.map((msg, i) => (
                <div
                  key={msg.uuid || i}
                  className={`msg-row ${msg.self ? "self" : ""}`}
                >
                  <div className="msg-bubble">{msg.message}</div>
                </div>
              ))}
              <div ref={messagesEndRef} />
            </div>

            <div className="chat-input-area">
              <textarea
                className="chat-input"
                rows={1}
                value={input}
                onChange={(e) => setInput(e.target.value)}
                onKeyDown={handleKeyDown}
                placeholder="Ketik pesan..."
              />
              <button
                className="send-btn"
                onClick={sendMessage}
                disabled={!connected || !input.trim()}
              >
                ➤
              </button>
            </div>
          </>
        )}
      </main>
    </div>
  );
}
