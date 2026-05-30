import { useEffect, useRef, useState } from "react";
import { useParams, Link } from "react-router-dom";
import { createChatSocket } from "../api/ws";

export default function ChatPage() {
  const { conversationId } = useParams();
  const [messages, setMessages] = useState([]);
  const [input, setInput] = useState("");
  const [connected, setConnected] = useState(false);
  const wsRef = useRef(null);

  useEffect(() => {
    const ws = createChatSocket(
      conversationId,
      (data) => setMessages((prev) => [...prev, data]),
      () => setConnected(true),
      () => setConnected(false),
    );
    wsRef.current = ws;

    return () => ws.close();
  }, [conversationId]);

  const sendMessage = () => {
    if (!input.trim() || !wsRef.current) return;
    wsRef.current.send(JSON.stringify({ message: input }));
    setMessages((prev) => [...prev, { message: input, self: true }]);
    setInput("");
  };

  const handleKeyDown = (e) => {
    if (e.key === "Enter") sendMessage();
  };

  return (
    <div>
      <h2>Chat</h2>
      <p>Conversation: {conversationId}</p>
      <p>Status: {connected ? "🟢 Connected" : "🔴 Disconnected"}</p>
      <Link to="/friends">← Back</Link>

      <div
        style={{
          border: "1px solid #ccc",
          height: 300,
          overflowY: "auto",
          padding: 8,
        }}
      >
        {messages.map((msg, i) => (
          <div key={i} style={{ textAlign: msg.self ? "right" : "left" }}>
            {typeof msg === "object" ? JSON.stringify(msg) : msg}
          </div>
        ))}
      </div>

      <div>
        <input
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="Ketik pesan..."
        />
        <button onClick={sendMessage} disabled={!connected}>
          Kirim
        </button>
      </div>
    </div>
  );
}
