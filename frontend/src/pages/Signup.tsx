import { useState } from "react";
import { Link } from "react-router-dom";
import {API_BASE_URL} from "../App";

export default function Signup() {
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");
    const [uid, setUid] = useState("");
    const [error, setError] = useState("");
    const [loading, setLoading] = useState(false);
    const [passwordShow, setPasswordShow] = useState(false);

    const signup = async () => {
        setLoading(true);
        setError("");
        try {
            const res = await fetch(`${API_BASE_URL}/signup`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({ username, password })
            });

            if (!res.ok) {
                const msg = await res.text();
                setError(msg);
                return;
            }

            const data = await res.json();
            setUid(data.uid);
        } catch {
            setError("Failed to connect to server");
        } finally {
            setLoading(false);
        }
    }

    return (
        <div>
            <h1>Signup</h1>
            <input
                placeholder="username"
                value={username}
                onChange={e => setUsername(e.target.value)}
                disabled={loading}
            />
            <input
                type={passwordShow ? "text" : "password"}
                placeholder="password"
                value={password}
                onChange={e => setPassword(e.target.value)}
                disabled={loading}
            />
            <button onClick={() => setPasswordShow(!passwordShow)} >
                {passwordShow ? "hide" : "show"}
            </button>
            <button onClick={signup} disabled={loading || !username || !password}>
                {loading ? "Creating..." : "Create Account"}
            </button>

            {error && <p style={{ color: "red" }}>{error}</p>}
            {uid && (
                <div style={{ marginTop: "1rem", padding: "1rem", border: "1px solid #ccc" }}>
                    <p>Successfully created! Your UID is below. Please save it for login.</p>
                    <p><strong>UID: {uid}</strong></p>
                </div>
            )}
            <p>
                <Link to="/login">
                    <button disabled={loading}>Go to Login</button>
                </Link>
            </p>
        </div>
    );
}