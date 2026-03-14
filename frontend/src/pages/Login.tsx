import { useState } from "react";
import { Link } from "react-router-dom";

const API_BASE_URL = "http://localhost:3000";

export default function Login() {
    const [uid, setUid] = useState("");
    const [password, setPassword] = useState("");
    const [username, setUsername] = useState("");
    const [error, setError] = useState("");
    const [loading, setLoading] = useState(false);

    const login = async () => {
        setLoading(true);
        setError("");
        setUsername("");

        try {
            const res = await fetch(`${API_BASE_URL}/login`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({ uid, password })
            });

            if (!res.ok) {
                const msg = await res.text();
                setError(msg);
                return;
            }

            const data = await res.json();
            setUsername(data.username);
        } catch {
            setError("Failed to connect to server");
        } finally {
            setLoading(false);
        }
    }

    return (
        <div>
            <h1>Login</h1>
            <input
                placeholder="uid"
                value={uid}
                onChange={e => setUid(e.target.value)}
                disabled={loading}
            />
            <input
                type="password"
                placeholder="password"
                value={password}
                onChange={e => setPassword(e.target.value)}
                disabled={loading}
            />
            <button onClick={login} disabled={loading || !uid || !password}>
                {loading ? "Logging in..." : "Login"}
            </button>

            {error && <p style={{ color: "red" }}>{error}</p>}
            {username && (
                <div style={{ marginTop: "1rem", color: "green" }}>
                    <p>Welcome back, <strong>{username}</strong>!</p>
                </div>
            )}

            <p>
                <Link to="/">
                    <button disabled={loading}>Create account?</button>
                </Link>
            </p>
        </div>
    );
}