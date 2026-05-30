import { useEffect, useState } from "react";
import { useNavigate, Link } from "react-router-dom";
import { getProfile } from "../api/user";
import { useAuth } from "../context/useAuth";

export default function ProfilePage() {
  const [profile, setProfile] = useState(null);
  const [error, setError] = useState("");
  const { logout } = useAuth();
  const navigate = useNavigate();

  useEffect(() => {
    getProfile()
      .then((data) => {
        console.log("profile response:", data);
        setProfile(data);
      })
      .catch(() => setError("Gagal memuat profil"));
  }, []);

  const handleLogout = () => {
    logout();
    navigate("/login");
  };

  return (
    <div>
      <h2>Profile</h2>
      {error && <p style={{ color: "red" }}>{error}</p>}
      {profile ? (
        <pre>{JSON.stringify(profile, null, 2)}</pre>
      ) : (
        <p>Loading...</p>
      )}
      <nav>
        <Link to="/friends">Friends</Link>
      </nav>
      <button onClick={handleLogout}>Logout</button>
    </div>
  );
}
