import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { getProfile } from "../api/user";
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

export default function ProfilePage() {
  const [profile, setProfile] = useState(null);
  const [error, setError] = useState("");
  const [copied, setCopied] = useState(false);
  const { logout } = useAuth();
  const navigate = useNavigate();

  useEffect(() => {
    getProfile()
      .then((data) => setProfile(data?.data || data))
      .catch(() => setError("Gagal memuat profil"));
  }, []);

  const handleLogout = () => {
    logout();
    navigate("/login");
  };

  const handleCopy = () => {
    if (!profile?.uuid) return;
    navigator.clipboard.writeText(profile.uuid);
    setCopied(true);
    setTimeout(() => setCopied(false), 1800);
  };

  return (
    <div className="profile-page-wrap">
      <div className="profile-window">
        {/* Titlebar */}
        <div className="profile-window-titlebar">
          <span className="pw-dot" style={{ background: "#f87171" }} />
          <span className="pw-dot" style={{ background: "#fbbf24" }} />
          <span className="pw-dot" style={{ background: "#20b28c" }} />
          <span className="profile-window-title">profil saya</span>
        </div>

        {/* Body */}
        <div className="profile-window-body">
          {error && (
            <p
              style={{
                color: "#f87171",
                textAlign: "center",
                marginBottom: 16,
              }}
            >
              {error}
            </p>
          )}

          {!profile ? (
            <p style={{ color: "var(--text-muted)", textAlign: "center" }}>
              Memuat...
            </p>
          ) : (
            <>
              <div className="pw-avatar-lg">{avatar(profile.name)}</div>
              <div className="pw-display-name">{profile.name || "—"}</div>
              <div className="pw-display-email">{profile.email || "—"}</div>

              <div className="pw-divider" />

              <div className="pw-field">
                <div className="pw-field-label">Nama</div>
                <div className="pw-field-value">{profile.name || "—"}</div>
              </div>

              <div className="pw-field">
                <div className="pw-field-label">Email</div>
                <div className="pw-field-value">{profile.email || "—"}</div>
              </div>

              <div className="pw-field">
                <div className="pw-field-label">UUID</div>
                <div className="pw-field-value uuid-row">
                  <span className="uuid-text">{profile.uuid || "—"}</span>
                  <button
                    className={`uuid-copy-btn ${copied ? "copied" : ""}`}
                    onClick={handleCopy}
                    disabled={!profile.uuid}
                  >
                    {copied ? "Tersalin!" : "Salin"}
                  </button>
                </div>
              </div>
            </>
          )}

          <div className="pw-actions">
            <button className="pw-btn pw-btn-back" onClick={() => navigate(-1)}>
              ← Kembali
            </button>
            <button className="pw-btn pw-btn-logout" onClick={handleLogout}>
              🚪 Keluar
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
