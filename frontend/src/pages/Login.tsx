import { useState } from "react";
import { Link } from "react-router-dom";

export default function Login() {

    const [uid, setUid] = useState("");
    const [password, setPassword] = useState("");
    const [username, setUsername] = useState("");

    const login = async () => {

        const res = await fetch("http://localhost:3000/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ uid, password })
        });

        const data = await res.json();

        setUsername(data.username);
    }

    return (
        <div>

            <h1>Login</h1>

            <input
                placeholder="uid"
                onChange={e => setUid(e.target.value)}
            />

            <input
                placeholder="password"
                onChange={e => setPassword(e.target.value)}
            />

            <button onClick={login}>
                login
            </button>

            <p>username: {username}</p>

            <p>
                <Link to="/">
                    <button>Create account?</button>
                </Link>
            </p>

        </div>
    );
}