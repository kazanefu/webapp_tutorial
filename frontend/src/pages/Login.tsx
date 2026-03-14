import { useState, useEffect } from "react";
import { Link } from "react-router-dom";
import { API_BASE_URL } from "../App";
import LogoutButton from "../components/LogoutButton"


export default function Login() {
    const [uid, setUid] = useState("");
    const [password, setPassword] = useState("");
    const [username, setUsername] = useState("");
    const [error, setError] = useState("");
    const [loading, setLoading] = useState(false);
    const [passwordShow, setPasswordShow] = useState(false);
    const [myname, setMyName] = useState("");

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
                credentials: "include",
                body: JSON.stringify({ uid, password })
            });

            if (!res.ok) {
                const msg = await res.text();
                setError(msg);
                return;
            }

            const data = await res.json();
            setUsername(data.username);
            me();
        } catch {
            setError("Failed to connect to server");
        } finally {
            setLoading(false);
        }
    }

    const me = async () => {
        setLoading(true);
        try {
            const res = await fetch(`${API_BASE_URL}/me`, {
                credentials: "include"
            });
            if (!res.ok) {
                const msg = await res.text();
                setMyName("");
                console.error(msg);
                return;
            }
            const data = await res.json();
            setMyName(data.username);
        } catch {
            setError("Failed to connect to server");
            setMyName("");
        } finally {
            setLoading(false);
        }

    }
    
    useEffect(
        () => {
            me();
        },
        []
    );

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
                type={passwordShow ? "text" : "password"}
                placeholder="password"
                value={password}
                onChange={e => setPassword(e.target.value)}
                disabled={loading}
            />
            <button onClick={() => setPasswordShow(!passwordShow)} >
                {passwordShow ? "hide" : "show"}
            </button>
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
            <button onClick={me} >
                {myname ? "User: "+myname:"check login state: not logged in"}
            </button>
            <div><LogoutButton onClick={me}/></div>
        </div>
    );
}