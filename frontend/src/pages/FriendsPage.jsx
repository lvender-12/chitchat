import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import {
  getFriendList,
  getPendingReceived,
  getPendingSent,
  addFriend,
  acceptRequest,
  rejectRequest,
} from "../api/friend";

export default function FriendsPage() {
  const [friends, setFriends] = useState([]);
  const [received, setReceived] = useState([]);
  const [sent, setSent] = useState([]);
  const [addUserId, setAddUserId] = useState("");
  const [msg, setMsg] = useState("");

  const fetchAll = async () => {
    const [f, r, s] = await Promise.all([
      getFriendList(),
      getPendingReceived(),
      getPendingSent(),
    ]);
    console.log("friends:", f, "received:", r, "sent:", s);
    setFriends(f);
    setReceived(r);
    setSent(s);
  };

  useEffect(() => {
    fetchAll();
  }, []);

  const handleAdd = async () => {
    if (!addUserId) return;
    const res = await addFriend(addUserId);
    console.log("add friend response:", res);
    setMsg(JSON.stringify(res));
    setAddUserId("");
    fetchAll();
  };

  const handleAccept = async (userId) => {
    const res = await acceptRequest(userId);
    setMsg(JSON.stringify(res));
    fetchAll();
  };

  const handleReject = async (userId) => {
    const res = await rejectRequest(userId);
    setMsg(JSON.stringify(res));
    fetchAll();
  };

  return (
    <div>
      <h2>Friends</h2>
      <Link to="/profile">← Back to Profile</Link>

      <section>
        <h3>Add Friend</h3>
        <input
          type="text"
          placeholder="User ID"
          value={addUserId}
          onChange={(e) => setAddUserId(e.target.value)}
        />
        <button onClick={handleAdd}>Add</button>
      </section>

      {msg && <p style={{ color: "green" }}>Response: {msg}</p>}

      <section>
        <h3>Friend List</h3>
        <pre>{JSON.stringify(friends, null, 2)}</pre>
      </section>

      <section>
        <h3>Pending Received</h3>
        {Array.isArray(received) && received.length > 0 ? (
          received.map((req) => (
            <div key={req.id}>
              <span>
                {req.id} - {req.name}
              </span>{" "}
              <button onClick={() => handleAccept(req.id)}>Accept</button>{" "}
              <button onClick={() => handleReject(req.id)}>Reject</button>
            </div>
          ))
        ) : (
          <pre>{JSON.stringify(received, null, 2)}</pre>
        )}
      </section>

      <section>
        <h3>Pending Sent</h3>
        <pre>{JSON.stringify(sent, null, 2)}</pre>
      </section>
    </div>
  );
}
